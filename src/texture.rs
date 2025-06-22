use crate::{color::Color, image_util::ImageTextureData, vec3::Vec3};
use rand::seq::SliceRandom;
use std::rc::Rc;

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

const POINT_COUNT: usize = 256;

struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut p = core::array::from_fn::<_, POINT_COUNT, _>(|i| i).to_vec();
        p.shuffle(&mut rand::rng());
        p.try_into().unwrap()
    }

    fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let u = u*u*(3. - 2.*u);
        let v = v*v*(3. - 2.*v);
        let w = w*w*(3. - 2.*w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.; 2]; 2]; 2];
        for (di, c_i) in c.iter_mut().enumerate() {
            for (dj, c_ij) in c_i.iter_mut().enumerate() {
                for (dk, c_ijk) in c_ij.iter_mut().enumerate() {
                    *c_ijk = self.randfloat[self.perm_x
                        [(i + di as i32).rem_euclid(POINT_COUNT as i32) as usize]
                        ^ self.perm_y[(j + dj as i32).rem_euclid(POINT_COUNT as i32) as usize]
                        ^ self.perm_z[(k + dk as i32).rem_euclid(POINT_COUNT as i32) as usize]];
                }
            }
        }

        trilinear_interp(c, u, v, w)
    }
}

fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.;
    for (i, ci) in c.iter().enumerate() {
        for (j, cj) in ci.iter().enumerate() {
            for (k, ck) in cj.iter().enumerate() {
                let i = i as f64;
                let j = j as f64;
                let k = k as f64;
                accum += (i * u + (1. - i) * (1. - u))
                    * (j * v + (1. - j) * (1. - v))
                    * (k * w + (1. - k) * (1. - w))
                    * ck;
            }
        }
    }
    accum
}

impl Default for Perlin {
    fn default() -> Self {
        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();
        let randfloat = core::array::from_fn(|_| rand::random::<f64>());

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
}

const WHITE: Color = Vec3(1., 1., 1.);

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, point: Vec3) -> Color {
        WHITE * self.noise.noise(&point)
    }
}
