use std::sync::Arc;

use crate::data::vec3::{Point3, Vec3};
use crate::data::{Lambertian, Material};
use crate::engine::ray::Ray;

use super::AABB;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material + Send + Sync>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        self.normal = *outward_normal;
        if !self.front_face {
            self.normal *= -1.0;
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: Lambertian::black_sh(),
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, output_box: &mut AABB) -> bool;
}
