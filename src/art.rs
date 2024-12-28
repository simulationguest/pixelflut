use image::{DynamicImage, GenericImageView, ImageReader};
use std::sync::{Arc, RwLock};

use crate::{color::Color, coordinates::Coordinates};

pub type Frame = u32;

pub trait Art: Clone + Send + 'static {
    fn dimensions(&self) -> Option<Coordinates> {
        None
    }
    fn get_pixel(&self, coordinates: Coordinates, frame: Frame) -> Color;
}

#[derive(Copy, Clone)]
pub struct SolidColor(pub Color);

impl Art for SolidColor {
    fn get_pixel(&self, _: Coordinates, _: u32) -> Color {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct EyeSore;

impl Art for EyeSore {
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

#[derive(Clone)]
pub struct Image(Arc<RwLock<DynamicImage>>);

impl Image {
    pub fn new(path: &str, size: Coordinates) -> Result<Self, image::ImageError> {
        let img = ImageReader::open(path)?.decode()?.resize(
            size.x,
            size.y,
            image::imageops::FilterType::Lanczos3,
        );

        Ok(Self(Arc::new(RwLock::new(img))))
    }

    pub fn size(&self) -> Coordinates {
        let handle = self.0.read().unwrap();
        Coordinates {
            x: handle.width(),
            y: handle.height(),
        }
    }
}

impl Art for Image {
    fn dimensions(&self) -> Option<Coordinates> {
        let handle = self.0.read().unwrap();
        Some(Coordinates {
            x: handle.width(),
            y: handle.height(),
        })
    }
    fn get_pixel(&self, coordinates: Coordinates, _: Frame) -> Color {
        let handle = self.0.read().unwrap();
        let c = handle.get_pixel(coordinates.x, coordinates.y);
        Color {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
}
