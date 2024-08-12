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
    let world = create_scene();
    let cam_look_from = Point3::new(0.0, 0.0, 10.0);
    let cam_look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = vec::Vec3::new(0.0, -1.0, 0.0);
    let cam = Camera::init(
        0.001,
        80.0,
        cam_look_from,
        cam_look_at,
        vup,
        16.0 / 9.0,
        20.0,
    );
    let now = Instant::now();
    cam.render(&world, 640, 480, 50, 20);
    eprintln!("\nElapsed: {:?}", now.elapsed());
}

fn create_scene() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
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
        Point3::new(0.0, 0.0, -1.2),
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
    world
}
