use crate::color::Color;
use rand;
use std::io::{BufWriter, Write};

pub fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.
}

#[derive(Clone)]
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
    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max <= b.max { a.max } else { b.max };
        Interval { min, max }
    }

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
        if x < self.min {
            return self.min;
        };
        if x > self.max {
            return self.max;
        };
        x
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
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
