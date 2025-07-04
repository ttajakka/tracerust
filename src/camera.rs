use crate::{
    color::Color,
    hittable::HittableList,
    ray::Ray,
    util::{Interval, PPM, degrees_to_radians},
    vec3::Vec3,
};
use rand;
use std::io::BufWriter;

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32,         // Maximum number of ray bounces into scene
    pub vfov: f64,              // vertical field of view in degrees
    pub lookfrom: Vec3,         // Point camera is looking from
    pub lookat: Vec3,           // Point camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction

    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Vec3,             // Camera center
    pixel00_loc: Vec3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    defocus_angle: f64,       // Variation angle of rays through each pixel
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
}

impl Camera {
    pub fn render(&self, world: HittableList) {
        let width = self.image_width;
        let height = self.image_height;
        let mut ppm = PPM::new(width, height);
        for j in 0..height {
            eprint!("\rScanlines remaining: {} ", (height - j));
            for i in 0..width {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::color_ray(&r, self.max_depth, &world)
                }

                ppm.push(self.pixel_samples_scale * pixel_color);
            }
        }
        eprint!("\rDone.                   \n");

        ppm.write_to_buffer(&mut BufWriter::new(std::io::stdout()));
    }

    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        focus_distance: f64,
        defocus_angle: f64, // degrees
    ) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = lookfrom;

        // Determine viewport dimensions
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        // Calculate the bases vectors u, v, w for the camera coordinate frame.
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = 1. / samples_per_pixel as f64;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * degrees_to_radians(defocus_angle / 2.).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn color_ray(ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }
        match world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            Some(rec) => match rec.mat.scatter(ray, &rec) {
                Some(scatres) => {
                    return scatres.attenuation
                        * Camera::color_ray(&scatres.scattered, depth - 1, world);
                }
                None => return Color::new(0., 0., 0.),
            },
            None => {
                let u = ray.dir().unit();
                let a = 0.5 * (u.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    /// Construct a camera ray originating from the defocus disk and directed at a
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(origin, pixel_sample - origin, rand::random())
    }

    fn sample_square() -> Vec3 {
        Vec3(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.0 * self.defocus_disk_u + p.1 * self.defocus_disk_v
    }
}
