use core::panic;
use std::rc::Rc;

use image::{ImageReader, RgbImage};

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

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

struct ImageTextureData {
    height: usize,
    width: usize,
    data: RgbImage
}

impl ImageTextureData {
    fn from_file(filename: &str) -> Self {
        if let image::DynamicImage::ImageRgb8(img) =
            ImageReader::open(filename).unwrap().decode().unwrap()
        {
            return Self {
                height: img.height() as usize,
                width: img.width() as usize,
                data: img
            };
        }
        panic!()
    }

    pub fn pixel_data(&self, i: usize, j: usize) -> Color {
        let pxl = self.data.get_pixel(i as u32, j as u32).0;
        Vec3(pxl[0] as f64, pxl[1] as f64, pxl[2] as f64)
    }
}

pub struct ImageTexture {
    image: ImageTextureData,
}

impl ImageTexture {
    pub fn from_file(filename: &str) -> Self {
        Self {
            image: ImageTextureData::from_file(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3) -> Color {
        if self.image.height == 0 {
            return Vec3(0., 1., 1.);
        }

        let u = u.clamp(0., 1.);
        let v = 1. - v.clamp(0., 1.);
        let i = (u * self.image.width as f64) as usize;
        let j = (v * self.image.height as f64) as usize - 1;

        1.0 / 255. * self.image.pixel_data(i, j)
    }
}
