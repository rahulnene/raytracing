use crate::vec::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn pos(&self, lambda: f64) -> Point3 {
        self.origin + self.direction * lambda
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
}
