use crate::{Color, Coordinates};

pub trait Art: Send + Clone {
    fn dimensions(&self) -> Option<Coordinates> {
        None
    }
    fn get_pixel(&self, coordinates: Coordinates) -> Color;
}

mod solid_color;

#[cfg(feature = "image")]
mod image;
