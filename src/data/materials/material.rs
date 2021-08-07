use std::sync::Arc;

use crate::{
    data::Color,
    engine::{HitRecord, Ray},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
