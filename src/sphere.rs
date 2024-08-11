use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec::Point3,
};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant <= 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-b - sqrt_discriminant) / a;
        if interval.surrounds(t) {
            let p = ray.pos(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord {
                time: t,
                point: p,
                normal,
                material: &self.material,
            });
        }
        t = (-b + sqrt_discriminant) / a;
        if interval.surrounds(t) {
            let p = ray.pos(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord {
                time: t,
                point: p,
                normal,
                material: &self.material,
            });
        }
        None
    }
}
