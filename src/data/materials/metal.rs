use crate::{
    data::{Color, Vec3},
    engine::{HitRecord, Ray},
};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().unit().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.dir().dot(&rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Metal {
        assert!(fuzz <= 1.0);
        Metal {
            albedo: Color::new(r, g, b),
            fuzz,
        }
    }
}
