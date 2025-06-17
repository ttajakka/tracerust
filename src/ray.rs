use core::f64;

use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
    tm: f64
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, time: f64) -> Self {
        Ray {
            origin: origin,
            dir,
            tm: time
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.clone() + self.dir.clone() * t
    }
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {return self.min};
        if x > self.max {return self.max};
        x
    }
}

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_works() {
        let ray = Ray {
            origin: Vec3(1.0, 1.0, 0.0),
            dir: Vec3(0.0, 2.0, 1.0),
            tm: 0.
        };
        assert_eq!(ray.at(-1.0), Vec3(1.0, -1.0, -1.0))
    }

    #[test]
    fn size_works() {
        let intvl = Interval { min: -2., max: 3. };
        assert_eq!(intvl.size(), 5.)
    }

    #[test]
    fn contains_works() {
        let intvl = Interval { min: -2., max: 3. };
        assert!(intvl.contains(3.0));
        assert!(!intvl.contains(4.));
    }

    #[test]
    fn surrounds_works() {
        let intvl = Interval { min: -2., max: 3. };
        assert!(intvl.surrounds(2.));
        assert!(!intvl.surrounds(3.0));
    }

    #[test]
    fn empty_and_universe() {
        assert!(!EMPTY.contains(0.));
        assert!(UNIVERSE.contains(0.));
    }
}
