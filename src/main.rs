use std::rc::Rc;

use tracerust::bvh::BVHNode;
use tracerust::camera::Camera;
use tracerust::color::Color;
use tracerust::hittable::{HittableList, Sphere};
use tracerust::material::{Dielectric, Lambertian, Material, Metal};
use tracerust::texture::CheckerTexture;
use tracerust::util;
use tracerust::vec3::Vec3;

fn main() {
    let mut world;
    let cam;
    match 2 {
        1 => (world, cam) = bouncing_spheres(),
        2 => (world, cam) = checkered_spheres(),
        _ => panic!(),
    }

    let count = world.count();
    let world = HittableList::from_hittable(BVHNode::new(&mut world.objects, 0, count));

    cam.render(world);
}

fn bouncing_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    // let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        Vec3(0.2, 0.3, 0.1),
        Vec3(0.9, 0.9, 0.9),
    ));
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::from_texture(checker));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., -1000., 0.),
        1000.,
        &ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //  diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3(0., util::random_f64(0., 0.2), 0.);
                    let sphere = Sphere::moving(center, center2, 0.2, material);
                    world.add(Rc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_mm(0.5, 1.);
                    let fuzz = util::random_f64(0., 0.5);
                    let material: Rc<dyn Material> = Rc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::stationary(center, 0.2, &material);
                    world.add(Rc::new(sphere));
                } else {
                    // glass
                    let material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
                    let sphere = Sphere::stationary(center, 0.2, &material);
                    world.add(Rc::new(sphere));
                }
            }
        }
    }

    let material_1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., 1., 0.),
        1.,
        &material_1,
    )));

    let material_3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Rc::new(Sphere::stationary(
        Vec3(4., 1., 0.),
        1.0,
        &material_3,
    )));
    let material_2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::stationary(
        Vec3(-4., 1., 0.),
        1.,
        &material_2,
    )));

    // Set up camera
    let aspect_ratio = 16.0_f64 / 9.0_f64;
    // let image_width = 800;
    // let samples_per_pixel = 100;
    // let max_depth = 50;
    let image_width = 400;
    let samples_per_pixel = 20;
    let max_depth = 20;

    let vfov = 20.;
    let lookfrom = Vec3(13., 2., 3.);
    let lookat = Vec3(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);
    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    let camera = Camera::new(
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

    (world, camera)
}

fn checkered_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        Vec3(0.2, 0.3, 0.1),
        Vec3(0.9, 0.9, 0.9),
    ));

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::from_texture(checker));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., -10., 0.),
        10.,
        &ground_material,
    )));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., 10., 0.),
        10.,
        &ground_material,
    )));

    // Set up camera
    let aspect_ratio = 16.0_f64 / 9.0_f64;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.;
    let lookfrom = Vec3(13., 2., 3.);
    let lookat = Vec3(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);
    let defocus_angle = 0.;
    let focus_distance = 10.0;

    let camera = Camera::new(
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

    (world, camera)
}
