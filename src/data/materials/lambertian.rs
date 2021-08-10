use std::sync::Arc;

use crate::{
    data::{
        textures::{SharedTexture, SolidColor},
        Color, Texture, Vec3,
    },
    engine::{HitRecord, Ray},
};

use super::Material;

pub struct Lambertian {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

impl Lambertian {
    pub fn black_sh() -> Arc<dyn Material + Send + Sync> {
        Arc::new(Lambertian::from_color(Color::new(0.0, 0.0, 0.0)))
    }

    pub fn from_color(color: Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::from_color(color)),
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian::from_color(Color::new(r, g, b))
    }

    pub fn from_texture(texture: SharedTexture) -> Lambertian {
        Lambertian { albedo: texture }
    }
}
