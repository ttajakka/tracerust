use std::rc::Rc;

use tracerust::camera::Camera;
use tracerust::color::Color;
use tracerust::hittable::{HittableList, Sphere};
use tracerust::material::{Lambertian, Metal};
use tracerust::vec3::Vec3;

fn main() {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8,0.8,0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8,0.6,0.2)));

    let mut world = HittableList { objects: vec![] };
    world.add(Rc::new(Sphere::new(Vec3(0., -100.5, -1.), 100., material_ground)));
    world.add(Rc::new(Sphere::new(Vec3(0., 0., -1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Vec3(-1., 0., -1.), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Vec3(1., 0., -1.), 0.5, material_right)));


    let cam = Camera::new(16.0_f64 / 9.0_f64, 400, 100, 50);

    cam.render(world);
}
