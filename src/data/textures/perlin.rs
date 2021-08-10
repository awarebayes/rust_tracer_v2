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
        ranfloat
            .iter_mut()
            .map(|_| random::<f64>())
            .collect::<Vec<f64>>();
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
        for i in (0..n).rev() {
            let target = self.rng.gen_range(0..i);
            let temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as usize & (POINT_COUNT - 1);
        let j = (4.0 * p.y()) as usize & (POINT_COUNT - 1);
        let k = (4.0 * p.z()) as usize & (POINT_COUNT - 1);

        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}

pub struct PerlinTexture {
    noise: Perlin,
}

impl Texture for PerlinTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
