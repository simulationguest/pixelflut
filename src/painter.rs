use crate::{
    art::{Art, Frame},
    connection::{Connection, Pool},
    coordinates::Coordinates,
};
use anyhow::Result;
use tokio::task::JoinSet;

async fn paint_simple(
    from: Coordinates,
    to: Coordinates,
    mut conn: Connection,
    art: impl Art,
    frame: Frame,
) -> Result<Connection> {
    for x in from.x..to.x {
        for y in from.y..to.y {
            let coordinates = Coordinates { x, y };
            let color = art.get_pixel(coordinates, frame);
            conn.write_pixel(coordinates, color).await?;
        }
    }
    Ok(conn)
}

pub async fn paint_blocks(
    size: Coordinates,
    block_count_x: u32,
    block_count_y: u32,
    pool: &mut Pool,
    art: impl Art,
    frame: Frame,
) -> Result<()> {
    let side_x = size.x / block_count_x;
    let side_y = size.y / block_count_y;

    let mut set = JoinSet::new();

    for x_block in 0..block_count_x {
        for y_block in 0..block_count_y {
            let from = Coordinates {
                x: x_block * side_x,
                y: y_block * side_y,
            };

            let to = Coordinates {
                x: (x_block + 1) * side_x,
                y: (y_block + 1) * side_y,
            };

            let conn = pool.get().await?;
            set.spawn(paint_simple(from, to, conn, art.clone(), frame));
        }
    }

    while let Some(res) = set.join_next().await {
        pool.put(res??);
    }

    Ok(())
}

/// Linear Congruetial Generator
///
/// * `m` MUST be a power of 2
/// * `m` and `c` MUST NOT share a common factor
/// * `a-1` MUST be divisible by all prime factors of `m`
/// * `a-1` MUST be divisible by 4 if `m` is divisible by 4
fn lcg(prev: u32, m: u32, a: u32, c: u32) -> u32 {
    (a * prev + c) % m
}

/// For a given number, returns the next larger power of two
fn next_po2(n: u32) -> u32 {
    let mut k = 1;
    while k < n {
        k *= 2;
    }
    k
}

pub struct LCG {
    m: u32,
    c: u32,
    grid_size: u32,
}

impl LCG {
    pub fn new(size: Coordinates) -> Self {
        let grid_size = size.x * size.y;
        Self {
            m: next_po2(grid_size),
            grid_size,
            c: 3,
        }
    }

    pub async fn paint(
        &mut self,
        size: Coordinates,
        conn: &mut Connection,
        art: impl Art,
        frame: Frame,
    ) -> Result<()> {
        let mut count = 0;
        let mut x = 5;
        let a = 17;
        self.c += 2;
        while count < self.grid_size {
            x = lcg(x, self.m, a, self.c);
            while x > self.grid_size {
                x = lcg(x, self.m, a, self.c);
            }
            let coordinates = Coordinates {
                x: x / size.y % size.x,
                y: x % size.y,
            };
            let color = art.get_pixel(coordinates, frame);
            conn.write_pixel(coordinates, color).await?;
            count += 1;
        }
        Ok(())
    }
}
