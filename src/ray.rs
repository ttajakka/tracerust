use crate::{color::Color, sphere::Sphere, vec3::Vec3};

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

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.clone() + self.dir.clone() * t
    }

    pub fn hit_sphere(&self, sphere: Sphere) -> bool {
        let oc = sphere.center() - self.origin;
        let a = self.dir.dot(self.dir);
        let b = -2.0 * self.dir.dot(oc);
        let c = oc.dot(oc) - sphere.radius() * sphere.radius();
        // Check if the quadratic has solutions
        b * b - 4.0 * a * c >= 0.0
    }

    pub fn color(&self) -> Color {
        if self.hit_sphere(Sphere::new(Vec3(0.0,0.0,-1.0), 0.5)) {
            return Color::new(1.0, 0.0,0.0)
        }
        let u = self.dir.unit();
        let a = 0.5 * (u.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
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
