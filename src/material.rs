use rand::{random, seq::index};

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec::{normalized, Vec3},
};

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    *vec - 2.0 * vec.dot(normal) * *normal
}

fn refract(vec: &Vec3, normal: &Vec3, eta_ratio: f64) -> Option<Vec3> {
    let normed = normalized(*vec);
    let dt = normed.dot(normal);
    let discriminant = 1.0 - eta_ratio.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        Some(eta_ratio * (normed - *normal * dt) - *normal * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, eta_ratio: f64) -> f64 {
    let r0 = (1.0 - eta_ratio) / (1.0 + eta_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.length_squared() < 0.0001 {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metallic {
    albedo: Color,
    fuzz: f64,
}

impl Metallic {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Self { albedo, fuzz }
        } else {
            Self { albedo, fuzz: 1.0 }
        }
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = reflect(&normalized(ray.direction()), &hit_record.normal);
        if self.fuzz > 0.0 {
            reflected += self.fuzz * Vec3::random_in_unit_sphere();
        }
        if reflected.dot(&hit_record.normal) > 0.0 {
            Some((Ray::new(hit_record.point, reflected), self.albedo))
        } else {
            None
        }
    }
}

pub struct Refractive {
    refractive_index: f64,
}

impl Refractive {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Refractive {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let (outward_normal, index_ratio, cosine) = if ray.direction().dot(&hit_record.normal) > 0.0
        {
            let cosine = self.refractive_index * ray.direction().dot(&hit_record.normal)
                / ray.direction().magnitude();
            (-hit_record.normal, self.refractive_index, cosine)
        } else {
            let cosine = -ray.direction().dot(&hit_record.normal) / ray.direction().magnitude();
            (hit_record.normal, 1.0 / self.refractive_index, cosine)
        };
        if let Some(refracted) = refract(&ray.direction(), &outward_normal, index_ratio) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            if random::<f64>() >= reflect_prob {
                let scattered = Ray::new(hit_record.point, refracted);
                return Some((scattered, attenuation));
            }
        }
        let reflected = reflect(&ray.direction(), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        Some((scattered, attenuation))
    }
}
