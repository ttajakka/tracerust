use std::io::BufWriter;

use tracerust::ray::Ray;
use tracerust::util::PPM;
use tracerust::vec3::Vec3;

fn main() {
    // Choose aspect ratio and image width
    let aspect_ratio = 16.0_f64 / 9.0_f64;
    let image_width: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    // Note: wiewport widths less than 1 are ok since they are real-valued.
    let focal_length = 1.0_f64;
    let viewport_height = 2.0_f64;
    let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
    let camera_center = Vec3(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut ppm = PPM::new(image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            ppm.push(ray.color());
        }
    }

    ppm.write_to_buffer(&mut BufWriter::new(std::io::stdout()));
}
