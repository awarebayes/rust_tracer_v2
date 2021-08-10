use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use super::{Hittable, AABB};

pub struct BVHnode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    my_box: AABB,
}

impl BVHnode {
    pub fn new(
        src_objects: &mut Vec<Arc<dyn Hittable + Send + Sync>>,
        start: usize,
        end: usize,
    ) -> BVHnode {
        let mut rng = rand::thread_rng();
        let objects = src_objects;
        let axis = rng.gen_range(start..=end);
        let comparator = |a, b| AABB::box_cmp(a, b, axis);
        let object_span = end - start;

        let left = match object_span {
            1 => Arc::clone(&objects[start]),
            2 => Arc::clone(match comparator(&objects[start], &objects[start + 1]) {
                Ordering::Less => &objects[start],
                _ => &objects[start + 1],
            }),
            _ => {
                objects.sort_by(|a, b| AABB::box_cmp(a, b, axis));
                let mid = start + object_span / 2;
                Arc::new(BVHnode::new(objects, start, mid))
            }
        };
        let right = match object_span {
            1 => Arc::clone(&objects[start]),
            2 => Arc::clone(match comparator(&objects[start], &objects[start + 1]) {
                Ordering::Less => &objects[start + 1],
                _ => &objects[start],
            }),
            _ => {
                let mid = start + object_span / 2;
                Arc::new(BVHnode::new(objects, mid, end))
            }
        };

        let mut box_left = AABB::empty();
        let mut box_right = AABB::empty();

        if !left.bounding_box(&mut box_left) || !right.bounding_box(&mut box_right) {
            eprintln!("No bb in bvh node constructor");
        }
        BVHnode {
            left,
            right,
            my_box: AABB::surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hittable for BVHnode {
    fn hit(&self, ray: &super::Ray, t_min: f64, t_max: f64, rec: &mut super::HitRecord) -> bool {
        if !self.my_box.hit(ray, t_min, t_max, rec) {
            return false;
        }
        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self.right.hit(ray, t_min, t_max, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = self.my_box.clone();
        true
    }
}
