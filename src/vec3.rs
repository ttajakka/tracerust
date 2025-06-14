use crate::util::random_f64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 - other.0,
        )
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn random() -> Self {
        Vec3(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        )
    }

    pub fn random_mm(min: f64, max: f64) -> Vec3 {
        Vec3(
            random_f64(min, max),
            random_f64(min, max),
            random_f64(min, max),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random();
            let lensq = p.length_squared();
            if 1e-160 <= lensq && lensq <= 1. {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        return self.0.abs() < EPS && self.1.abs() < EPS && self.2.abs() < EPS;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        assert_eq!(v1 + v2, Vec3(3.0, 6.0, -3.0))
    }

    #[test]
    fn add_assign_works() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        v1 += v2;
        assert_eq!(v1, Vec3(3.0, 6.0, -3.0))
    }

    #[test]
    fn sub_works() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        assert_eq!(v1 - v2, Vec3(-1.0, -2.0, 9.0))
    }

    #[test]
    fn sub_assign_works() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        v1 -= v2;
        assert_eq!(v1, Vec3(-1.0, -2.0, 9.0))
    }

    #[test]
    fn mul_f64_works() {
        let v = Vec3(2.0, 4.0, -6.0);
        assert_eq!(v * -3.0, Vec3(-6.0, -12.0, 18.0))
    }

    #[test]
    fn right_mul_f64_works() {
        let v = Vec3(2.0, 4.0, -6.0);
        assert_eq!(-3.0 * v, Vec3(-6.0, -12.0, 18.0))
    }

    #[test]
    fn mul_self_works() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        assert_eq!(v1 * v2, Vec3(2.0, 8.0, -18.0))
    }

    #[test]
    fn mul_assign_f64_works() {
        let mut v = Vec3(2.0, 4.0, -6.0);
        v *= -3.0;
        assert_eq!(v, Vec3(-6.0, -12.0, 18.0))
    }

    #[test]
    fn mul_assign_self_works() {
        let mut v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        v1 *= v2;
        assert_eq!(v1, Vec3(2.0, 8.0, -18.0))
    }

    #[test]
    fn div_works() {
        let v = Vec3(2.0, 4.0, -6.0);
        assert_eq!(v / 2.0, Vec3(1.0, 2.0, -3.0))
    }

    #[test]
    fn div_assign_works() {
        let mut v = Vec3(2.0, 4.0, -6.0);
        v /= 2.0;
        assert_eq!(v, Vec3(1.0, 2.0, -3.0))
    }

    #[test]
    fn neg_works() {
        let v = Vec3(1.0, -2.0, 3.0);
        assert_eq!(-v, Vec3(-1.0, 2.0, -3.0))
    }

    #[test]
    fn length_squared_works() {
        let v = Vec3(1.0, 2.0, -2.0);
        assert_eq!(v.length_squared(), 9.0)
    }

    #[test]
    fn length_works() {
        let v = Vec3(1.0, 2.0, -2.0);
        assert_eq!(v.length(), 3.0)
    }

    #[test]
    fn dot_works() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 4.0, -6.0);
        assert_eq!(v1.dot(v2), -8.0)
    }

    #[test]
    fn cross_works() {
        let v1 = Vec3(1.0, 0.0, 0.0);
        let v2 = Vec3(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(&v2), Vec3(0.0, 0.0, 1.0))
    }

    #[test]
    fn unit_works() {
        let v = Vec3(1.0, -1.0, 0.0);
        let w = Vec3(1.0 / 2.0_f64.sqrt(), -1.0 / 2.0_f64.sqrt(), 0.0);
        assert_eq!(v.unit(), w)
    }
}
