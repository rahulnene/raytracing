use crate::{interval::Interval, material::Material, ray::Ray, vec::Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub time: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = interval.max;
        let mut hit_anything = None;

        for hittable in &self.list {
            if let Some(hit) = hittable.hit(
                r,
                &Interval {
                    min: interval.min,
                    max: closest_so_far,
                },
            ) {
                closest_so_far = hit.time;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
