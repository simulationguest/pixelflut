use super::Art;
use crate::{Color, Coordinates};

#[derive(Clone)]
pub struct SolidColor(pub Color);

impl Art for SolidColor {
    fn get_pixel(&self, _: Coordinates) -> Color {
        self.0
    }
}
