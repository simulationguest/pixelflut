use std::fmt::Display;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
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
