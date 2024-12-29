use crate::{net, Art, Coordinates, Pool};

pub async fn paint(pool: Pool, art: impl Art, offset: Coordinates) -> Result<(), net::Error> {
    let mut handle = pool.acquire().await?;
    let size = art.size();

    for x in 0..size.x {
        for y in 0..size.y {
            let coordinates = Coordinates { x, y };
            let color = art.get_pixel(coordinates);
            handle.write_pixel(coordinates + offset, color).await?;
        }
    }

    Ok(())
}
