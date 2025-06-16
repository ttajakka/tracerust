use crate::ray::Interval;
use crate::vec3::Vec3;

const INTENSITY: Interval = Interval{min: 0., max: 0.999};

fn linear_to_gamma(linear_component: f64) -> f64 {
    match linear_component > 0. {
        true => linear_component.sqrt(),
        false => 0.
    }
}

pub type Color = Vec3;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Vec3(r, g, b)
    }

    pub fn write_io<W: std::io::Write>(&self, w: &mut W) {
        let r = linear_to_gamma(self.0);
        let g = linear_to_gamma(self.1);
        let b = linear_to_gamma(self.2);

        let rbyte = (256. * INTENSITY.clamp(r)) as u8;
        let gbyte = (256. * INTENSITY.clamp(g)) as u8;
        let bbyte = (256. * INTENSITY.clamp(b)) as u8;
        writeln!(w, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_stdout_works() {
        let c = Vec3(0.5, 1.0, 0.0);
        let mut s = Vec::new();
        c.write_io(&mut s);
        let s = String::from_utf8(s).unwrap();
        assert_eq!(s, "181 255 0\n");
    }
}
