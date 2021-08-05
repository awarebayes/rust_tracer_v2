use crate::data::vec3::{Vec3, Point3};
use crate::engine::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3)
    {
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        self.normal = *outward_normal;
        if !self.front_face {
            self.normal *= -1.0;
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord { p: Vec3::new(0.0,0.0,0.0), normal: Vec3::new(0.0,0.0,0.0), t: 0.0, front_face: false }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}