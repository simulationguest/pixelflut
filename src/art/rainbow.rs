use super::Art;

use crate::{Color, Coordinates};

#[derive(Clone)]
pub struct Rainbow {
    current_hue: f32,
    hue_step: f32,
    color_cache: Color,
}

fn color(hue: f32) -> Color {
    Color::hsl(hue, 1., 0.7)
}

impl Rainbow {
    pub fn new(hue_step: f32) -> Self {
        Self {
            hue_step,
            current_hue: 0.,
            color_cache: color(0.),
        }
    }
}

impl Art for Rainbow {
    fn get_pixel(&self, _: Coordinates) -> Color {
        self.color_cache
    }

    fn next_frame(&mut self) {
        self.current_hue += self.hue_step;
        self.current_hue %= 360.;
        self.color_cache = color(self.current_hue)
    }
}
