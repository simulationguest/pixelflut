use std::num::NonZeroU32;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    host: String,
    #[arg(long)]
    x: u32,
    #[arg(long)]
    y: u32,
    #[arg(long)]
    width: Option<NonZeroU32>,
    #[arg(long)]
    height: Option<NonZeroU32>,
}
