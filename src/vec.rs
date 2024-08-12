use nalgebra::Vector3;
use rand::random;
// use std::fmt::{Display, Formatter, Result};
// use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Vec3 {
//     coords: [f64; 3],
// }

// impl Default for Vec3 {
//     fn default() -> Self {
//         Vec3::new(0.0, 0.0, 0.0)
//     }
// }

// impl Vec3 {
//     pub fn new(x: f64, y: f64, z: f64) -> Self {
//         Self { coords: [x, y, z] }
//     }

//     pub fn x(&self) -> f64 {
//         self.coords[0]
//     }
//     pub fn y(&self) -> f64 {
//         self.coords[1]
//     }
//     pub fn z(&self) -> f64 {
//         self.coords[2]
//     }

//     pub fn magnitude(&self) -> f64 {
//         self.length_squared().sqrt()
//     }
//     pub fn length_squared(&self) -> f64 {
//         self.coords.iter().map(|c| c * c).sum()
//     }

//     pub fn dot(&self, rhs: &Self) -> f64 {
//         self.coords
//             .iter()
//             .zip(rhs.coords.iter())
//             .map(|(a, b)| a * b)
//             .sum()
//     }

//     pub fn cross(&self, rhs: &Self) -> Self {
//         Self::new(
//             self.y() * rhs.z() - self.z() * rhs.y(),
//             self.z() * rhs.x() - self.x() * rhs.z(),
//             self.x() * rhs.y() - self.y() * rhs.x(),
//         )
//     }

//     pub fn random() -> Self {
//         Self::new(random(), random(), random())
//     }

//     pub fn random_range(min: f64, max: f64) -> Self {
//         Self::new(
//             random::<f64>() * (max - min) + min,
//             random::<f64>() * (max - min) + min,
//             random::<f64>() * (max - min) + min,
//         )
//     }

//     pub fn random_in_unit_sphere() -> Self {
//         loop {
//             let p = Self::random_range(-1.0, 1.0);
//             if p.length_squared() < 1.0 {
//                 return p;
//             }
//         }
//     }

//     pub fn random_unit_vector() -> Self {
//         Self::random_in_unit_sphere().normalized()
//     }

//     pub fn random_in_hemisphere(normal: Self) -> Self {
//         let in_unit_sphere = Self::random_in_unit_sphere();
//         if in_unit_sphere.dot(&normal) > 0.0 {
//             in_unit_sphere
//         } else {
//             -in_unit_sphere
//         }
//     }

//     pub fn normalized(&self) -> Self {
//         *self / (self.magnitude() + f64::EPSILON)
//     }
// }

pub type Point3 = Vector3<f64>;
pub type Vec3 = Vector3<f64>;

pub fn random_in_unit_sphere() -> Vec3 {
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vec3::new(random::<f64>(), random::<f64>(), random::<f64>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

// impl Display for Vec3 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{} {} {}", self.x(), self.y(), self.z())
//     }
// }

// impl Neg for Vec3 {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         Self::new(-self.x(), -self.y(), -self.z())
//     }
// }

// impl Add for Vec3 {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
//     }
// }
// impl AddAssign for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }

// impl Sub for Vec3 {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
//     }
// }
// impl SubAssign for Vec3 {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs;
//     }
// }

// impl Mul<f64> for Vec3 {
//     type Output = Self;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
//     }
// }
// impl MulAssign<f64> for Vec3 {
//     fn mul_assign(&mut self, rhs: f64) {
//         *self = *self * rhs;
//     }
// }

// impl Div<f64> for Vec3 {
//     type Output = Self;

//     fn div(self, rhs: f64) -> Self::Output {
//         self * (1.0 / rhs)
//     }
// }
// impl DivAssign<f64> for Vec3 {
//     fn div_assign(&mut self, rhs: f64) {
//         *self = *self / rhs;
//     }
// }

// impl Mul<Vec3> for f64 {
//     type Output = Vec3;

//     fn mul(self, v: Vec3) -> Vec3 {
//         Vec3::new(self * v.x(), self * v.y(), self * v.z())
//     }
// }

// impl Mul<i32> for Vec3 {
//     type Output = Self;

//     fn mul(self, rhs: i32) -> Self::Output {
//         Self::new(
//             self.x() * rhs as f64,
//             self.y() * rhs as f64,
//             self.z() * rhs as f64,
//         )
//     }
// }

// impl Mul<Vec3> for Vec3 {
//     type Output = Self;

//     fn mul(self, rhs: Vec3) -> Self::Output {
//         Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
//     }
// }
