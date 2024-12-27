use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
