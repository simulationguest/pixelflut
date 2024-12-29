use super::Art;
use crate::{Color, Coordinates};

#[derive(Clone)]
pub struct Rectangle {
    pub color: Color,
    pub size: Coordinates,
}

impl Art for Rectangle {
    #[inline]
    fn get_pixel(&self, _: Coordinates) -> Color {
        self.color
    }

    fn size(&self) -> Coordinates {
        self.size
    }
}
