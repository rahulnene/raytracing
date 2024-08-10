mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec;
use camera::Camera;
use vec::Point3;

fn main() {
    let mut world = hittable::HittableList::new();
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(a as f64, b as f64, -10.0);
            world.push(Box::new(sphere::Sphere::new(center, 0.5)));
        }
    }

    let cam = Camera::init(16.0 / 9.0, 640);
    let now = std::time::Instant::now();
    cam.render(&world);
    eprintln!("Elapsed: {:?}", now.elapsed());
}
