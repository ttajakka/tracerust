use std::rc::Rc;

use tracerust::camera::Camera;
use tracerust::hittable::{HittableList, Sphere};
use tracerust::vec3::Vec3;

fn main() {
    let mut world = HittableList { objects: vec![] };
    world.add(Rc::new(Sphere::new(Vec3(0., -100.5, -1.), 100.)));
    world.add(Rc::new(Sphere::new(Vec3(0., 0., -1.), 0.5)));

    let cam = Camera::new(16.0_f64 / 9.0_f64, 400, 100, 50);

    cam.render(world);
}
