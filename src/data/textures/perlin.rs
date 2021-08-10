use rand::{random, Rng, SeedableRng};

use crate::data::{Color, Point3, Vec3};

use super::Texture;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>,
    rng: rand::rngs::StdRng,
}

impl Perlin {
    pub fn empty() -> Perlin {
        Perlin {
            ranfloat: Vec::new(),
            perm_x: Vec::new(),
            perm_y: Vec::new(),
            perm_z: Vec::new(),
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn new() -> Perlin {
        let mut p = Perlin::empty();
        let mut ranfloat = vec![0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranfloat[i] = random();
        }
        p.ranfloat = ranfloat;
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let c_ijk = c[i][j][k];
                    let (i, j, k) = (i as f64, j as f64, k as f64);
                    accum += (i*u + (1.0-i)*(1.0-u)) *
                             (j*v + (1.0-j)*(1.0-v)) *
                             (k*w + (1.0-k)*(1.0-w)) * c_ijk; 
                }
            }
        }
        accum
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0-2.0*u);
        v = v * v * (3.0-2.0*v);
        w = w * w * (3.0-2.0*w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(
                          self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }
        Perlin::trilinear_interp(&c, u, v, w)
    }
}

pub struct PerlinTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(&(self.scale * *p))
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
