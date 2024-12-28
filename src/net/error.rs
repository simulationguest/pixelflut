use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] tokio::io::Error),

    #[error("parsing error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("format error: {0}")]
    FormatError(#[from] std::fmt::Error),
}
