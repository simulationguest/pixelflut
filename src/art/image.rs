use image::{DynamicImage, GenericImageView, ImageReader};

use std::{
    path::Path,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct Image(Arc<RwLock<DynamicImage>>);

impl Image {
    pub fn new<P: AsRef<Path>>(path: P, size: Coordinates) -> Result<Self, image::ImageError> {
        let img = ImageReader::open(path)?.decode()?.resize(
            size.x,
            size.y,
            image::imageops::FilterType::Lanczos3,
        );

        Ok(Self(Arc::new(RwLock::new(img))))
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

    fn get_pixel(&self, coordinates: Coordinates) -> Color {
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
