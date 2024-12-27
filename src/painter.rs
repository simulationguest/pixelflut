use crate::{
    connection::{Connection, Pool},
    coordinates::Coordinates,
    paintable::{Frame, Paintable},
};
use anyhow::Result;
use tokio::task::JoinSet;

async fn paint_simple(
    from: Coordinates,
    to: Coordinates,
    mut conn: Connection,
    paintable: impl Paintable,
    frame: Frame,
) -> Result<Connection> {
    for x in from.x..to.x {
        for y in from.y..to.y {
            let coordinates = Coordinates { x, y };
            let color = paintable.get_pixel(coordinates, frame);
            conn.write_pixel(coordinates, color).await?;
        }
    }
    Ok(conn)
}

pub async fn paint_blocks(
    size: Coordinates,
    block_count_x: u16,
    block_count_y: u16,
    pool: &mut Pool,
    paintable: impl Paintable,
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
            set.spawn(paint_simple(from, to, conn, paintable, frame));
        }
    }

    while let Some(res) = set.join_next().await {
        pool.put(res??);
    }

    Ok(())
}
