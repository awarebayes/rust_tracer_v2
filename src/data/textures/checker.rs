use std::sync::Arc;

use crate::data::{Color, Vec3};

use super::{SharedTexture, SolidColor, Texture};

pub struct CheckerTexture {
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(even: SharedTexture, odd: SharedTexture) -> CheckerTexture {
        CheckerTexture { even, odd }
    }

    pub fn from_colors(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture::new(
            Arc::new(SolidColor::from_color(c1)),
            Arc::new(SolidColor::from_color(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines > 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
