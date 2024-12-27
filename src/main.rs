mod args;
mod color;
mod coordinates;
mod painter;
mod writer;

use std::u8;

use args::Args;
use clap::Parser;
use color::Color;
use coordinates::Coordinates;
use writer::Writer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut writer = Writer::new(&args.host).await?;
    println!("{}", writer.get_size().await?);

    for x in 0..300 {
        for y in 0..300 {
            writer
                .write_pixel(
                    Coordinates { x, y },
                    Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: u8::MAX,
                    },
                )
                .await?;
        }
    }

    Ok(())
}
