use crate::{Color, Coordinates};

pub trait Art: Sync + Clone {
    fn get_pixel(&self, coordinates: Coordinates) -> Color;
    fn next_frame(&mut self) {}
}

mod rainbow;
mod solid_color;

#[cfg(feature = "image")]
mod image;

pub use rainbow::*;
pub use solid_color::*;

#[cfg(feature = "image")]
pub use image::*;
