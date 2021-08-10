mod data;
mod engine;
mod util;

use data::{materials::Dielectric, worlds::marble_land, Color, Lambertian, Material, Metal, Point3, Vec3};
use engine::{Camera, HitRecord, Hittable, HittableList, Ray, Sphere};
use util::thread_pool::{PlacedPixel, RTThreadPool};

use std::sync::{Arc, Mutex};

use lodepng;
use rand::{random, Rng};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 800;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 50;
const MAX_DEPTH: usize = 100;
const N_THREADS: usize = 10;

fn ray_color(
    r: &Ray,
    world: Arc<dyn Hittable>,
    rng: &mut rand::rngs::StdRng,
    depth: usize,
) -> Vec3 {
    let mut rec = HitRecord::empty();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut attenuation = Color::new(1.0, 1.0, 1.0);
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, rng, depth - 1);
        }
        return Color::zero();
    }
    let unit_dir = r.dir().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn generate_image() -> Vec<[u8; 4]> {
    let world = marble_land();

    let mut pool = RTThreadPool::new(N_THREADS, WIDTH, HEIGHT);
    pool.start_collecting();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        ASPECT_RATIO,
        20.0,
        aperture,
        dist_to_focus,
    );
    let camera = Arc::new(camera);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let process_pixel = move |rng: Arc<Mutex<rand::rngs::StdRng>>| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 1..=SAMPLES_PER_PIXEL {
                    let mut rng = rng.lock().unwrap();
                    let r1: f64 = rng.gen();
                    let r2: f64 = rng.gen();
                    let u = (i as f64 + r1) / (WIDTH - 1) as f64;
                    let v = (j as f64 + r2) / (HEIGHT - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, world.clone(), &mut rng, MAX_DEPTH);
                }
                Ok(PlacedPixel {
                    i,
                    j,
                    color: pixel_color.as_color(SAMPLES_PER_PIXEL),
                })
            };
            pool.execute(process_pixel);
        }
    }
    pool.collect();
    let end_image = pool.end_image.lock().unwrap().clone();
    end_image
}

fn main() {
    // let mut buffer: [[u8; 4]; WIDTH * HEIGHT] = [[0, 0, 0, 255]; WIDTH * HEIGHT];
    let buffer = generate_image();
    match lodepng::encode32_file("out.png", buffer.as_ref(), WIDTH, HEIGHT) {
        Ok(_) => println!("Image saved..."),
        Err(_) => println!("Error"),
    };
}
