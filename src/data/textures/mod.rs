pub mod checker;
pub mod perlin;
pub mod solid_color;
pub mod texture;

pub use self::texture::{SharedTexture, Texture};
pub use checker::CheckerTexture;
pub use perlin::PerlinTexture;
pub use solid_color::SolidColor;
