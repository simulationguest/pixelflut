use thiserror::Error;

use crate::color;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] tokio::io::Error),

    #[error("parsing error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("format error: {0}")]
    FormatError(#[from] std::fmt::Error),

    #[error("recv error: {0}")]
    RecvError(#[from] async_channel::TryRecvError),

    #[error("parse color error: {0}")]
    ParseColorError(#[from] color::Error),
}
