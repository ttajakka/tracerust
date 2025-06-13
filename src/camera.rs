use std::io::BufWriter;
use crate::{color::Color, hittable::HittableList, ray::Interval, ray::Ray, util::PPM, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,

    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: HittableList) {
        let width = self.image_width;
        let height = self.image_height;
        let mut ppm = PPM::new(width, height);
        for j in 0..height {
            for i in 0..width {
                let pixel_center =
                    self.pixel00_loc + (i as f64) * self.pixel_delta_u + (j as f64) * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                ppm.push(Self::color_ray(&ray, &world));
            }
        }

        ppm.write_to_buffer(&mut BufWriter::new(std::io::stdout()));
    }

    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = Vec3(0., 0., 0.);

        let focal_length = 1.0_f64;
        let viewport_height = 2.0_f64;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn color_ray(ray: &Ray, world: &HittableList) -> Color {
        match world.hit(
            ray,
            &Interval {
                min: 0.,
                max: f64::INFINITY,
            },
        ) {
            Some(rec) => return 0.5 * (rec.normal + Vec3(1., 1., 1.)),
            None => {
                let u = ray.dir().unit();
                let a = 0.5 * (u.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
