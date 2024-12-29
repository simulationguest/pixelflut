use std::{fmt::Display, u8};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    #[inline]
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        let a = s * l.min(1. - l);

        #[inline]
        fn k(n: f32, h: f32) -> f32 {
            (n + h / 30.) % 12.
        }

        #[inline]
        fn f(n: f32, h: f32, l: f32, a: f32) -> u8 {
            ((l - a * (k(n, h) - 3.).min(9. - k(n, h).min(1.)).max(-1.)) * 255.) as u8
        }

        Color::rgb(f(0., h, l, a), f(8., h, l, a), f(4., h, l, a))
    }

    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }

    #[inline]
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)?;
        if self.a != u8::MAX {
            write!(f, "{:02X}", self.a)?;
        }
        Ok(())
    }
}
