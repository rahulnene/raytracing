use crate::vec::Point3;
use nalgebra::Vector3;

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn pos(&self, lambda: f64) -> Point3 {
        self.origin + self.direction * lambda
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
}
