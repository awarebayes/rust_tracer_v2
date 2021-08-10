use crate::data::{Color, Vec3};

use super::Texture;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color: Color::new(r, g, b),
        }
    }

    pub fn from_color(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.color
    }
}
