use std::num::NonZeroU32;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub addr: String,
    #[arg(long, default_value_t = 0)]
    x: u32,
    #[arg(long, default_value_t = 0)]
    y: u32,
    #[arg(long)]
    width: Option<NonZeroU32>,
    #[arg(long)]
    height: Option<NonZeroU32>,
}
