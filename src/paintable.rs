use crate::{color::Color, coordinates::Coordinates};

pub trait Paintable: Clone + Copy {
    fn dimensions() -> Option<Coordinates> {
        None
    }
    fn get_pixel(&self, coordinates: Coordinates, frame: u32) -> Color;
}

#[derive(Copy, Clone)]
pub struct SolidColor(Color);

impl Paintable for SolidColor {
    fn get_pixel(&self, _: Coordinates, _: u32) -> Color {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct EyeSore;

impl Paintable for EyeSore {
    fn get_pixel(&self, _: Coordinates, frame: u32) -> Color {
        if frame % 2 == 0 {
            Color {
                r: 255,
                g: 0,
                b: 0,
                a: u8::MAX,
            }
        } else {
            Color {
                r: 0,
                g: 255,
                b: 0,
                a: u8::MAX,
            }
        }
    }
}
