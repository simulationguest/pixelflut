use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
