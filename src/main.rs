use std::rc::Rc;

use tracerust::camera::Camera;
use tracerust::color::Color;
use tracerust::hittable::{HittableList, Sphere};
use tracerust::material::{Dielectric, Lambertian, Material, Metal};
use tracerust::vec3::Vec3;

fn main() {
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let material_bubble: Rc<dyn Material> = Rc::new(Dielectric::new(1.0 / 1.5));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.5));

    let mut world = HittableList { objects: vec![] };
    world.add(Rc::new(Sphere::new(
        Vec3(0., -100.5, -1.),
        100.,
        &material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3(0., 0., -1.2),
        0.5,
        &material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3(-1., 0., -1.),
        0.5,
        &material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3(-1., 0.1, -1.),
        0.3,
        &material_bubble,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3(1., 0., -1.),
        0.5,
        &material_right,
    )));

    // Set up camera
    let aspect_ratio = 16.0_f64 / 9.0_f64;
    let image_width = 400;
    let samples_per_pixel = 50;
    let max_depth = 10;
    let vfov = 90.;
    let lookfrom = Vec3(0., 0., 0.);
    let lookat = Vec3(0., 0., -1.);
    let vup = Vec3(0., 1., 0.);

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup
    );

    cam.render(world);
}
