use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.clone() + self.dir.clone() * t
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
