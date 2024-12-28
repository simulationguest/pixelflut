mod args;
mod art;
mod color;
mod connection;
mod coordinates;
mod painter;

use std::time::Duration;

use args::Args;
use art::{EyeSore, Image, SolidColor};
use clap::Parser;
use painter::{paint_blocks, LCG};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    video_rs::init().unwrap();

    let args = Args::parse();

    let mut pool = connection::Pool::new(args.addr);

    let size = pool.get_size().await?;

    let art = Image::new("./cat.jpg", size)?;
    let art = SolidColor(color::Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    });
    let art = EyeSore;

    let mut painter = LCG::new(size);
    let mut conn = pool.get().await?;

    for frame in 0..u32::MAX {
        painter.paint(size, &mut conn, art.clone(), frame).await?;
    }

    Ok(())
}
