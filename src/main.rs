mod data;
mod engine;

use crate::data::{Vec3};
use crate::engine::{Ray, HittableList, HitRecord, Hittable, Sphere};

use std::sync::Arc;

use indicatif::ProgressBar;
use lodepng;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 640;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;


fn ray_color(r: &Ray, world: Arc<dyn Hittable>) -> Vec3 {
    let mut hit_record = HitRecord::empty();
    if world.hit(r, 0.0, f64::INFINITY, &mut hit_record)
    {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_dir = r.dir().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn generate_image(buffer: &mut [[u8; 4]]) {
    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    
    // world
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let world = Arc::new(world);

    let bar = ProgressBar::new(HEIGHT as u64);
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, world.clone());
            buffer[(HEIGHT - 1 - j) * WIDTH + i] = pixel_color.as_color();
        }
        bar.inc(1);
    }
    bar.finish();
}

fn main() {
    let mut buffer: [[u8; 4]; WIDTH * HEIGHT] = [[0, 0, 0, 255]; WIDTH * HEIGHT];
    generate_image(&mut buffer);
    match lodepng::encode32_file("out.png", &buffer, WIDTH, HEIGHT) {
        Ok(_) => println!("Image saved..."),
        Err(_) => println!("Error"),
    };
}
