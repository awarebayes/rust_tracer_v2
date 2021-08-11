pub mod checker;
pub mod image;
pub mod perlin;
pub mod solid_color;
pub mod texture;

pub use self::texture::{SharedTexture, Texture};
pub use checker::CheckerTexture;
pub use image::ImageTexture;
pub use perlin::PerlinTexture;
pub use solid_color::SolidColor;
