use std::sync::Arc;

use crate::data::vec3::Vec3;
use crate::data::{Lambertian, Material};
use crate::engine::hittable::{HitRecord, Hittable};

use super::AABB;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Arc<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &super::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = *ray.origin() - self.center;
        let a = ray.dir().len_sq();
        let half_b = oc.dot(ray.dir());
        let c = oc.len_sq() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return false;
        }

        let sqrtd = disc.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);
        return true;
    }

    fn bounding_box(&self, output_box: &mut super::AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
