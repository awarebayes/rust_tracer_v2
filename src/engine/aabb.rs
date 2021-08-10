use std::{cmp::Ordering, sync::Arc};

use crate::data::Point3;

use super::Hittable;

#[derive(Clone)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    pub fn empty() -> AABB {
        AABB { min: Point3::zero(), max: Point3::zero() }
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f64::min(box0.min.x(), box1.min.x()),
            f64::min(box0.min.y(), box1.min.y()),
            f64::min(box0.min.z(), box1.min.z()),
        );

        let big = Point3::new(
            f64::max(box0.max.x(), box1.max.x()),
            f64::max(box0.max.y(), box1.max.y()),
            f64::max(box0.max.z(), box1.max.z()),
        );
        AABB::new(small, big)
    }

    pub fn box_cmp<'a>(a: &'a Arc<dyn Hittable + Send + Sync>, b: &'a Arc<dyn Hittable + Send + Sync>, axis: usize) -> Ordering {
        let mut box_a = AABB::empty();
        let mut box_b = AABB::empty();

        if !a.bounding_box(&mut box_a) || !b.bounding_box(&mut box_b) 
        {
            eprint!("No bounding box in bvh nore constructor");
        }

        let a = box_a.min[axis];
        let b = box_b.min[axis];
        a.partial_cmp(&b).unwrap()
    }
}

impl Hittable for AABB {
    fn hit(&self, r: &super::Ray, t_min: f64, t_max: f64, _rec: &mut super::HitRecord) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let inv_d =  1.0 / r.dir()[a];
            let t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let t1 = (self.max[a] - r.origin()[a]) * inv_d;
            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);
            if t_max <= t_min
            {
                return false;
            }
        }
        true
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        todo!()
    }
}