use crate::{color::Color, coordinates::Coordinates};

pub type Frame = u32;

pub trait Paintable: Clone + Copy + Send + 'static {
    fn dimensions() -> Option<Coordinates> {
        None
    }
    fn get_pixel(&self, coordinates: Coordinates, frame: Frame) -> Color;
}

#[derive(Copy, Clone)]
pub struct SolidColor(pub Color);

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
