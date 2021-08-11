use lodepng;
use rgb;

use crate::data::{Color, Vec3};

use super::Texture;

pub struct ImageTexture {
    data: Vec<rgb::RGBA<u8>>,
    height: usize,
    width: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let image = lodepng::decode32_file(filename).unwrap();
        ImageTexture {
            data: image.buffer,
            width: image.width,
            height: image.height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        };
        if j >= self.height {
            j = self.height - 1;
        };

        let pixel = self.data[j * self.width + i];
        return Color::from_rgb(pixel.r, pixel.g, pixel.b);
    }
}
