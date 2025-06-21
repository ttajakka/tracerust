use std::rc::Rc;

use crate::{color::Color, vec3::Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Rc::new(SolidColor { albedo: c1 }) as Rc<dyn Texture>,
            Rc::new(SolidColor { albedo: c2 }) as Rc<dyn Texture>,
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color {
        let x_integer = (self.inv_scale * point.x()).floor() as i32;
        let y_integer = (self.inv_scale * point.y()).floor() as i32;
        let z_integer = (self.inv_scale * point.z()).floor() as i32;

        let is_even = if (x_integer + y_integer + z_integer) % 2 == 0 {
            true
        } else {
            false
        };

        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
