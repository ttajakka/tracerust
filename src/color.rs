use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Vec3(r, g, b)
    }

    pub fn write_io<W: std::io::Write>(&self, w: &mut W) {
        let rbyte = (255.999 * self.0) as u8;
        let gbyte = (255.999 * self.1) as u8;
        let bbyte = (255.999 * self.2) as u8;
        writeln!(w, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
    }

    pub fn write_fmt<W: std::fmt::Write>(&self, w: &mut W) {
        let rbyte = (255.999 * self.0) as u8;
        let gbyte = (255.999 * self.1) as u8;
        let bbyte = (255.999 * self.2) as u8;
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
        assert_eq!(s, "127 255 0\n");
    }

    #[test]
    fn write_to_string_works() {
        let c = Vec3(1.0, 0.5, 0.0);
        let mut s = String::new();
        c.write_fmt(&mut s);
        assert_eq!(s, "255 127 0\n");
    }
}
