use crate::data::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { orig: origin, dir }
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.orig + self.dir * t;
    }
}
