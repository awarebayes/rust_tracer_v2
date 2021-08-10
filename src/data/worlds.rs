
use std::sync::Arc;

use rand::random;

use crate::data::{materials::Dielectric, Color, Lambertian, Material, Metal, Point3, Vec3};
use crate::engine::{HittableList, Sphere};

pub fn marble_land() -> Arc<HittableList> {

    let mut world = HittableList::new();

    let ground_mat = Arc::new(Lambertian::new(0.5, 0.5, 0.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);
            let choose_mat: f64 = random();
            let center = Point3::new(a + 0.9 * random::<f64>(), 0.2, b + 0.9 * random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let mut material_ptr: Option<Arc<dyn Material + Send + Sync>> = None;
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    material_ptr = Some(Arc::new(Lambertian::new(
                        albedo.x(),
                        albedo.y(),
                        albedo.z(),
                    )));
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        material_ptr.take().unwrap(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::rand_range(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    material_ptr = Some(Arc::new(Metal::new(
                        albedo.x(),
                        albedo.y(),
                        albedo.z(),
                        fuzz,
                    )));
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        material_ptr.take().unwrap(),
                    )));
                } else {
                    material_ptr = Some(Arc::new(Dielectric::new(1.5)));
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        material_ptr.take().unwrap(),
                    )));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Arc::new(Lambertian::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = Arc::new(Metal::new(0.7, 0.6, 0.5, 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    Arc::new(world)
}

pub fn three_balls() -> Arc<HittableList> {
    let mut world = HittableList::new();
    let mat_ground = Arc::new(Lambertian::new(0.8, 0.8, 0.0));
    let mat_center = Arc::new(Lambertian::new(0.1, 0.2, 0.5));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(0.8, 0.6, 0.2, 0.0));

    // world
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        mat_center.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        mat_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground.clone(),
    )));

    Arc::new(world)
}