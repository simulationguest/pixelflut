mod args;
mod canvas;
mod color;

use args::Args;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    dbg!(args);
}
