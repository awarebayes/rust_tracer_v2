use crate::engine::hittable::{Hittable, HitRecord};
use super::ray::Ray;
use std::sync::Arc;

pub struct HittableList
{
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {

    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>)
    {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty(); 
        let mut hit_anything = false;
        let mut closest = t_max;

        for i in self.objects.iter() {
            if i.hit(ray, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}