use crate::{color::Color, hittable::{Hittable, Sphere}, vec3::Vec3};

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

    pub fn color(&self) -> Color {
        let center = Vec3(0., 0., -1.);
        let sphere = Sphere::new(center, 0.5);
        match sphere.hit(self, 0., 1000.) {
            Some(t) => {
                let n = t.normal;
                return 0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.);
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
