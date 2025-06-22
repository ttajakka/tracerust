use image::{ImageReader, RgbImage};
use crate::vec3::Vec3;
use crate::color::Color;

pub struct ImageTextureData {
    pub height: usize,
    pub width: usize,
    data: RgbImage
}

impl ImageTextureData {
    pub fn from_file(filename: &str) -> Self {
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