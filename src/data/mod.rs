pub mod materials;
pub mod vec3;
pub mod worlds;


pub use vec3::{Color, Point3, Vec3};

pub use materials::{Lambertian, Material, Metal};

pub use worlds::{marble_land};