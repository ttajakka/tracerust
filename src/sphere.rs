use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere {center, radius}
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}