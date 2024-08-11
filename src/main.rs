mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec;
use std::time::Instant;

use camera::Camera;
use color::Color;
use hittable::HittableList;
use material::{Lambertian, Metallic, Refractive};
use vec::Point3;

fn main() {
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.2, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Refractive::new(1.5);
    let material_inside_left = Refractive::new(1.0 / 1.5);
    let material_right = Metallic::new(Color::new(0.8, 0.8, 0.8), 0.04);

    world.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.3,
        material_center,
    )));
    world.push(Box::new(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Box::new(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.38,
        material_inside_left,
    )));
    world.push(Box::new(sphere::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // world.push(Box::new(sphere::Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     Lambertian::new(Color::new(0.2, 0.2, 0.5)),
    // )));
    // world.push(Box::new(sphere::Sphere::new(
    //     Point3::new(0.0, 100.5, -1.0),
    //     100.0,
    //     Metallic::new(Color::new(1.0, 1.0, 1.0), 0.1),
    // )));
    let cam = Camera::init(16.0 / 9.0, 640, 50, 100);
    let now = Instant::now();
    cam.render(&world);
    eprintln!("Elapsed: {:?}", now.elapsed());
}
