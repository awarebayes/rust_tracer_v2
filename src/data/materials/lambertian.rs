use std::sync::Arc;

use crate::{
    data::{Color, Vec3},
    engine::{HitRecord, Ray},
};

use super::Material;

pub struct Lambertian {
    albedo: Color,
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
        *attenuation = self.albedo;
        true
    }
}

impl Lambertian {
    pub fn black_sh() -> Arc<dyn Material + Send + Sync> {
        Arc::new(Lambertian {
            albedo: Color::new(0.0, 0.0, 0.0),
        })
    }

    pub fn new(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian {
            albedo: Color::new(r, g, b),
        }
    }
}
