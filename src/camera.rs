use crate::{
    color::{write_color, Color},
    hittable::{Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vec::{normalized, Point3, Vec3},
};
use rand::random;
use std::io;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_offset_u: Vec3,
    pixel_offset_v: Vec3,
    samples_per_pixel: i32,
    recursion_limit: i32,
}

impl Camera {
    pub fn init(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        recursion_limit: i32,
    ) -> Self {
        let focal_length = 1.0;
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let view_port_height = 2.0;
        let view_port_width = aspect_ratio * view_port_height;
        let viewport_u = Vec3::new(view_port_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -view_port_height, 0.0);
        let viewport_origin = Point3::default()
            - viewport_u / 2.0
            - viewport_v / 2.0
            - Vec3::new(0.0, 0.0, focal_length);
        let pixel_offset_u = viewport_u / (image_width as f64);
        let pixel_offset_v = viewport_v / (image_height as f64);
        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            recursion_limit,
            center: Point3::default(),
            pixel_00_loc: viewport_origin + 0.5 * (pixel_offset_u + pixel_offset_v),
            pixel_offset_u,
            pixel_offset_v,
        }
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &HittableList) -> Color {
        if let Some(hit) = world.hit(r, &Interval::new(0.0001, f64::MAX)) {
            if depth <= 0 {
                return Color::default();
            } else {
                if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                    return attenuation * self.ray_color(&scattered, depth - 1, world);
                } else {
                    return Color::default();
                }
            }
        } else {
            let unit_dir = normalized(r.direction());
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64, j as f64);

                    pixel_color += self.ray_color(&ray, self.recursion_limit, world);
                }
                write_color(
                    &mut io::stdout(),
                    pixel_color / self.samples_per_pixel as f64,
                );
            }
        }
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_offset_u * (i + offset.x()))
            + (self.pixel_offset_v * (j + offset.y()));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0)
    }
}
