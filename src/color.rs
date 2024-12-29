use std::{fmt::Display, num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }

    #[inline]
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Display for Color {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)?;
        if self.a != u8::MAX {
            write!(f, "{:02X}", self.a)?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid length")]
    InvalidLength,

    #[error("parse int error: {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.len() != 6 || s.len() != 8 {
            return Err(Error::InvalidLength);
        }

        fn parse(s: &str) -> Result<u8, ParseIntError> {
            u8::from_str_radix(s, 16)
        }

        let r = parse(&s[0..2])?;
        let g = parse(&s[2..4])?;
        let b = parse(&s[4..6])?;

        let a = if s.len() == 8 {
            parse(&s[6..8])?
        } else {
            u8::MAX
        };

        Ok(Color::rgba(r, g, b, a))
    }
}
