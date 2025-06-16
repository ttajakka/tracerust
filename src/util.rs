use crate::color::Color;
use rand;
use std::io::{BufWriter, Write};

pub fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.
}

pub struct PPM {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl PPM {
    pub fn new(width: u32, height: u32) -> Self {
        PPM {
            width,
            height,
            pixels: Vec::new(),
        }
    }

    pub fn push(&mut self, color: Color) {
        self.pixels.push(color);
    }

    pub fn write_to_buffer<W: Write>(&self, writer: &mut BufWriter<W>) {
        let w = self.width;
        let h = self.height;
        writer
            .write(format!("P3\n{} {}\n255\n", w, h).as_bytes())
            .unwrap();

        for j in 0..h {
            for i in 0..w {
                self.pixels[(j * w + i) as usize].write_io(writer);
            }
        }
    }
}

pub fn image_test() {
    let image_width = 256;
    let image_height = 256;

    let mut ppm = PPM::new(image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let color = Color::new(
                0.0_f64,
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
            );
            ppm.push(color);
        }
    }

    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    ppm.write_to_buffer(&mut stdout);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ppm_writer_works() {
        let ppm = PPM {
            width: 2,
            height: 2,
            pixels: vec![
                Color::new(0.0, 1.0, 0.0),
                Color::new(0.0, 0.5, 0.0),
                Color::new(0.0, 0.0, 0.0),
                Color::new(0.0, 0.0, 0.0),
            ],
        };

        let actual: Vec<u8> = vec![];
        let mut writer = BufWriter::new(actual);
        ppm.write_to_buffer(&mut writer);

        let expected = "P3\n2 2\n255\n0 255 0\n0 181 0\n0 0 0\n0 0 0\n".as_bytes();
        assert_eq!(writer.buffer(), expected)
    }

    #[test]
    fn random_f64_works() {
        for _ in 0..100 {
            let random = random_f64(2., 5.);
            assert!(2. <= random && random < 5.)
        }
    }
}
