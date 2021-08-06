use crate::data::vec3::Vec3;
use crate::engine::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
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

        return true;
    }
}
