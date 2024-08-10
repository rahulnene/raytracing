use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec::{normalized, Point3, Vec3},
};
use std::io;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_offset_u: Vec3,
    pixel_offset_v: Vec3,
}

impl Camera {
    pub fn init(aspect_ratio: f64, image_width: i32) -> Self {
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
            center: Point3::default(),
            pixel_00_loc: viewport_origin + 0.5 * (pixel_offset_u + pixel_offset_v),
            pixel_offset_u,
            pixel_offset_v,
        }
    }

    fn ray_color(&self, r: &Ray, world: &impl Hittable) -> Color {
        let mut hit_record = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut hit_record) {
            return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
        }

        let unit_direction = normalized(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &impl Hittable) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..self.image_width {
                let ray_start =
                    self.pixel_00_loc + (self.pixel_offset_u * i) + (self.pixel_offset_v * j);
                let ray_direction = ray_start - self.center;
                let r = Ray::new(ray_start, ray_direction);
                let pixel_color = self.ray_color(&r, world);
                write_color(&mut io::stdout(), pixel_color);
            }
        }
    }
}
