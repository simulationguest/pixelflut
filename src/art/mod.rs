use crate::{Color, Coordinates};

pub trait Art: Sync + Clone {
    fn get_pixel(&self, coordinates: Coordinates) -> Color;
    fn size(&self) -> Coordinates;
    fn next_frame(&mut self) {}
}

mod rectangle;

#[cfg(feature = "image")]
mod image;

pub use rectangle::*;

#[cfg(feature = "image")]
pub use image::*;
