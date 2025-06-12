use core::f64;

use crate::{color::Color, hittable::HittableList, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray {
            origin: origin,
            dir,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.clone() + self.dir.clone() * t
    }

    pub fn color(&self, world: &HittableList) -> Color {
        match world.hit(self, 0., f64::INFINITY) {
            Some(rec) => {
                return 0.5 * (rec.normal + Vec3(1., 1., 1.))
            }
            None => {
                let u = self.dir.unit();
                let a = 0.5 * (u.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_works() {
        let ray = Ray {
            origin: Vec3(1.0, 1.0, 0.0),
            dir: Vec3(0.0, 2.0, 1.0),
        };
        assert_eq!(ray.at(-1.0), Vec3(1.0, -1.0, -1.0))
    }
}
