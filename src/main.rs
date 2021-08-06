mod data;
mod engine;
mod util;

use data::{Color, Point3, Vec3};
use engine::{Camera, HitRecord, Hittable, HittableList, Ray, Sphere};
use util::thread_pool::{PlacedPixel, RTThreadPool, ResultMessage};

use std::sync::{Arc, Mutex};

use lodepng;
use rand::Rng;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 640;
pub const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
pub const SAMPLES_PER_PIXEL: usize = 100;


fn ray_color(r: &Ray, world: Arc<dyn Hittable>) -> Vec3 {
    let mut hit_record = HitRecord::empty();
    if world.hit(r, 0.0, f64::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_dir = r.dir().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn generate_image() -> Vec<[u8; 4]> {
    // world
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut pool = RTThreadPool::new(10, WIDTH, HEIGHT);

    let world = Arc::new(world);
    let camera = Camera::new();
    let camera = Arc::new(camera);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let process_pixel = move |rng: Arc<Mutex<rand::rngs::StdRng>>| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 1..=SAMPLES_PER_PIXEL {
                    let r1: f64 = rng.lock().unwrap().gen();
                    let r2: f64 = rng.lock().unwrap().gen();
                    let u = (i as f64 + r1) / (WIDTH - 1) as f64;
                    let v = (j as f64 + r2) / (HEIGHT - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, world.clone());
                }
                ResultMessage::Ok(PlacedPixel {
                    i,
                    j,
                    color: pixel_color.as_color(SAMPLES_PER_PIXEL),
                })
            };
            pool.execute(process_pixel);
        }
    }
    pool.collect();
    pool.end_image.clone()
}

fn main() {
    // let mut buffer: [[u8; 4]; WIDTH * HEIGHT] = [[0, 0, 0, 255]; WIDTH * HEIGHT];
    let buffer = generate_image();
    match lodepng::encode32_file("out.png", buffer.as_ref(), WIDTH, HEIGHT) {
        Ok(_) => println!("Image saved..."),
        Err(_) => println!("Error"),
    };
}
