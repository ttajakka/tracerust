use crate::{ray::Ray, vec3::Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f64>;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = self.center - ray.origin();
        let a = ray.dir().length_squared();
        let h = ray.dir().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        // Check if the quadratic has solutions
        let disc = h * h - a * c;

        match disc >= 0. {
            false => None,
            true => Some((h - disc.sqrt()) / a),
        }
    }
}
