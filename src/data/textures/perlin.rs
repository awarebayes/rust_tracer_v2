use rand::{random, Rng, SeedableRng};

use crate::data::{Color, Point3, Vec3};

use super::Texture;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>,
    rng: rand::rngs::StdRng,
}

impl Perlin {
    pub fn empty() -> Perlin {
        Perlin {
            ranvec: Vec::new(),
            perm_x: Vec::new(),
            perm_y: Vec::new(),
            perm_z: Vec::new(),
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn new() -> Perlin {
        let mut p = Perlin::empty();
        let mut ranvec = vec![Vec3::zero(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranvec[i] = Vec3::rand_range(-1.0, 1.0);
        }
        p.ranvec = ranvec;
        p.perm_x = p.generate_perm();
        p.perm_y = p.generate_perm();
        p.perm_z = p.generate_perm();
        p
    }

    fn generate_perm(&mut self) -> Vec<i64> {
        let mut p: Vec<i64> = vec![0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i64
        }

        self.permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(&mut self, p: &mut Vec<i64>, n: usize) {
        for i in (1..n).rev() {
            let target = self.rng.gen_range(0..i);
            let temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let c_ijk = c[i][j][k];
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let (i, j, k) = (i as f64, j as f64, k as f64);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * c_ijk.dot(&weight_v);
                }
            }
        }
        accum
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }
        Perlin::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

pub struct PerlinTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        // Color::new(1.0, 1.0, 1.0) * self.noise.turb(&(self.scale * *p), 7)
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(&(self.scale * *p), 7)).sin())
    }
}

impl PerlinTexture {
    pub fn new(scale: f64) -> PerlinTexture {
        PerlinTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}
