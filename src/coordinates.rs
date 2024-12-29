use std::{
    fmt::Display,
    ops::{Add, Sub},
};

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

impl Add<Coordinates> for Coordinates {
    type Output = Coordinates;
    fn add(mut self, rhs: Coordinates) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl Sub<Coordinates> for Coordinates {
    type Output = Coordinates;
    fn sub(mut self, rhs: Coordinates) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}
