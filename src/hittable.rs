use crate::{interval::Interval, ray::Ray, vec::Vec3};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub time: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64) -> Self {
        Self {
            point: p,
            normal,
            time: t,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool;
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
    fn hit(&self, r: &Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for hittable in &self.list {
            if hittable.hit(
                r,
                &Interval::new(interval.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.time;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}
