use crate::{Color, Coordinates};

use super::Art;

use std::{
    path::Path,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use image::{DynamicImage, GenericImageView, ImageReader};

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

    fn handle(&self) -> RwLockReadGuard<'_, DynamicImage> {
        self.0.read().unwrap()
    }
}

impl Art for Image {
    fn get_pixel(&self, coordinates: Coordinates) -> Color {
        let handle = self.handle();
        let c = handle.get_pixel(coordinates.x, coordinates.y);
        Color::rgba(c[0], c[1], c[2], c[3])
    }
}
