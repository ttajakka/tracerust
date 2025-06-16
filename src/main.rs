use std::rc::Rc;

use tracerust::camera::Camera;
use tracerust::color::Color;
use tracerust::hittable::{HittableList, Sphere};
use tracerust::material::{Dielectric, Lambertian, Material, Metal};
use tracerust::vec3::Vec3;

fn main() {
    let mut world = HittableList { objects: vec![] };

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        &ground_material,
    )));

    let material_1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Vec3(0., 1., 0.), 1., &material_1)));

    let material_2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3(-4., 1., 0.), 1., &material_2)));

    let material_3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Rc::new(Sphere::new(Vec3(4., 1., 0.), 1.0, &material_3)));

    // Set up camera
    let aspect_ratio = 16.0_f64 / 9.0_f64;
    let image_width = 200;
    let samples_per_pixel = 10;
    let max_depth = 10;

    let vfov = 20.;
    let lookfrom = Vec3(13., 2., 3.);
    let lookat = Vec3(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);
    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        focus_distance,
        defocus_angle,
    );

    cam.render(world);
}
