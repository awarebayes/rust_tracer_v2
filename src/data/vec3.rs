use rand::{random, Rng};
use std::{cmp::min, ops::{self, Index}};

const CLOSE_PREC: f64 = 10e-6;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

fn rand_float(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random() -> Vec3 {
        Self::new(random(), random(), random())
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_float(min, max),
            rand_float(min, max),
            rand_float(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random();
            if p.len_sq() <= 1.0 {
                continue;
            };
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -1.0 * in_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let mut p = Vec3::rand_range(-1.0, 1.0);
            p.z = 0.0;
            if p.len_sq() >= 1.0 {
                continue;
            };
            return p;
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * self.dot(normal)
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-1.0 * *self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parralel = -(1.0 - r_out_perp.len_sq()).abs().sqrt() * *normal;
        r_out_perp + r_out_parralel
    }

    pub fn near_zero(&self) -> bool {
        (self.x.abs() < CLOSE_PREC) && (self.y.abs() < CLOSE_PREC) && (self.z.abs() < CLOSE_PREC)
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        );
    }

    pub fn as_color(&self, n_samples: usize) -> [u8; 4] {
        let scale = 1.0 / n_samples as f64;
        let s = *self * scale;
        let s = s.clamp(0.0, 0.999);
        let (mut r, mut g, mut b) = (s.x, s.y, s.z);
        r = r.sqrt();
        g = g.sqrt();
        b = b.sqrt();

        [
            (r * 255.999) as u8,
            (g * 255.999) as u8,
            (b * 255.999) as u8,
            255,
        ]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self * _rhs.x, self * _rhs.y, self * _rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

fn is_close(x: f64, y: f64) -> bool {
    (x - y).abs() <= CLOSE_PREC
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        is_close(self.x, other.x) && is_close(self.y, other.y) && is_close(self.z, other.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn add() {
        assert_eq!(
            Vec3::new(0.0, 0.0, 0.0) + Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3::new(0.0, 0.0, 0.0) - Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(-1.0, -2.0, -3.0)
        );
    }

    #[test]
    fn mul_num() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn div_num() {
        assert_eq!(Vec3::new(3.0, 6.0, 9.0) / 3.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn dot() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(&Vec3::new(1.0, 2.0, 3.0)),
            14.0
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).cross(&Vec3::new(5.0, 2.0, 1.0)),
            Vec3::new(-4.0, 14.0, -8.0)
        )
    }

    #[test]
    fn add_assign() {
        let mut a = Vec3::new(0.0, 0.0, 0.0);
        a += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn sub_assign() {
        let mut a = Vec3::new(0.0, 0.0, 0.0);
        a -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn mul_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a *= -1.0;
        assert_eq!(a, Vec3::new(-1.0, -2.0, -3.0));
    }
}
