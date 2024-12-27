mod args;
mod color;
mod connection;
mod coordinates;
mod paintable;
mod painter;

use std::u8;

use args::Args;
use clap::Parser;
use color::Color;
use paintable::SolidColor;
use painter::paint_blocks;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut pool = connection::Pool::new(args.addr);

    let size = pool.get_size().await?;

    let paintable = SolidColor(Color {
        r: u8::MAX,
        g: 0,
        b: 0,
        a: u8::MAX,
    });

    loop {
        paint_blocks(size, 3, 3, &mut pool, paintable, 0).await?;
    }
}
