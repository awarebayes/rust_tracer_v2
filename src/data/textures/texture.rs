use std::sync::Arc;

use crate::data::{Color, Vec3};

pub type SharedTexture = Arc<dyn Texture + Send + Sync>;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
