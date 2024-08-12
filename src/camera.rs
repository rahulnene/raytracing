use crate::{
    color::{write_color, Color},
    hittable::{Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vec::{Point3, Vec3},
};
use nalgebra::Vector3;
use rand::random;
use std::io;

fn random_pt_in_unit_disk() -> Vec3 {
    let unit = Vec3::new(1.0, 1.0, 0.0);
    let mut p;
    loop {
        p = 2.0 * Vec3::new(random::<f64>(), random::<f64>(), 0.0) - unit;
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}
pub struct Camera {
    center: Point3,
    pixel_00_loc: Point3,
    pixel_offset_u: Vec3,
    pixel_offset_v: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn init(
        aperture_angle: f64,
        vfov: f64,
        point_look_from: Point3,
        point_look_at: Point3,
        vup: Vec3,
        aspect_ratio: f64,
        focal_length: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = focal_length * f64::tan(theta / 2.0);
        let half_width = half_height * aspect_ratio;

        let w = (point_look_from - point_look_at).normalize();
        let u = (vup.cross(&w)).normalize();
        let v = w.cross(&u);
        Camera {
            center: point_look_from,
            pixel_00_loc: point_look_from - half_height * v - half_width * u - focal_length * w,
            pixel_offset_u: half_width * 2.0 * u,
            pixel_offset_v: half_height * 2.0 * v,
            u,
            v,
            lens_radius: aperture_angle / 2.0,
        }
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &HittableList) -> Color {
        if let Some(hit) = world.hit(r, &Interval::new(0.0001, f64::MAX)) {
            if depth <= 0 {
                return Color::default();
            } else {
                if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                    let base_color = self.ray_color(&scattered, depth - 1, world);
                    return attenuation.zip_map(&base_color, |l, r| l * r);
                } else {
                    return Color::default();
                }
            }
        } else {
            let unit_dir = (r.direction()).normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::default()

            // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn render(
        &self,
        world: &HittableList,
        image_width: u32,
        image_height: u32,
        samples_per_pixel: u32,
        recursion_limit: i32,
    ) {
        print!("P3\n{} {}\n255\n", image_width, image_height);
        let mut percent_done;
        for j in (0..image_height).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            percent_done = (image_height - j) as f64 / image_height as f64;
            eprint!("{:.1}%", percent_done * 100.0);
            for i in 0..image_width {
                let mut pixel_color = Color::default();
                for _ in 0..samples_per_pixel {
                    let mut i = i as f64 / image_width as f64;
                    let mut j = j as f64 / image_height as f64;
                    i += random::<f64>();
                    j += random::<f64>();
                    let ray = self.get_ray(i, j);

                    pixel_color +=
                        self.ray_color(&ray, recursion_limit, world) / samples_per_pixel as f64;
                }
                write_color(&mut io::stdout(), pixel_color);
            }
        }
        eprintln!()
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let ray_direction = self.lens_radius * random_pt_in_unit_disk();
        let offset = ray_direction.x * self.u + self.v * ray_direction.y;
        let ray_origin = self.center + offset;
        let ray_direction = self.pixel_00_loc + i * self.pixel_offset_u + j * self.pixel_offset_v
            - self.center
            - offset;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0)
    }
}
