use std::rc::Rc;

use tracerust::bvh::BVHNode;
use tracerust::camera::{Camera, CameraParams, ImageParams};
use tracerust::color::Color;
use tracerust::hittable::{Hittable, HittableList, Quad, Sphere, Translate, hittable_box};
use tracerust::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use tracerust::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use tracerust::util;
use tracerust::vec3::Vec3;

fn main() {
    let (mut world, cam) = match 7 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_sphere(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => panic!(),
    };

    let count = world.count();
    let world = HittableList::from_hittable(BVHNode::new(&mut world.objects, 0, count));

    cam.render(world);
}

fn bouncing_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::default();

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
    let image_params = ImageParams {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 20,
        max_depth: 20,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_params = CameraParams {
        vfov: 20.,
        lookfrom: Vec3(13., 2., 3.),
        lookat: Vec3(0., 0., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.6,
    };

    let camera = Camera::new(image_params, camera_params);

    (world, camera)
}

fn checkered_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::default();

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
    let image_params = ImageParams {
        aspect_ratio: 16.0_f64 / 9.0_f64,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_params = CameraParams {
        vfov: 20.,
        lookfrom: Vec3(13., 2., 3.),
        lookat: Vec3(0., 0., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.,
    };

    let camera = Camera::new(image_params, camera_params);

    (world, camera)
}

fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let earth_texture = Rc::new(ImageTexture::from_file("assets/earthmap.jpg"));
    let earth_surface: Rc<dyn Material> = Rc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::stationary(Vec3(0., 0., 0.), 2., &earth_surface);
    world.add(Rc::new(globe));

    let image_params = ImageParams {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_params = CameraParams {
        vfov: 20.,
        lookfrom: Vec3(0., 0., 12.),
        lookat: Vec3(0., 0., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.,
    };

    let camera = Camera::new(image_params, camera_params);
    (world, camera)
}

fn perlin_sphere() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let noise_tex = Rc::new(NoiseTexture::new(4.));
    let noise_mat: Rc<dyn Material> = Rc::new(Lambertian::from_texture(noise_tex));
    let globe = Sphere::stationary(Vec3(0., 2., 0.), 2., &noise_mat);
    world.add(Rc::new(globe));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., -1000., 0.),
        1000.,
        &noise_mat,
    )));

    // Set up camera
    let image_params = ImageParams {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_params = CameraParams {
        vfov: 20.,
        lookfrom: Vec3(13., 2., 3.),
        lookat: Vec3(0., 0., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.6,
    };

    let camera = Camera::new(image_params, camera_params);

    (world, camera)
}

fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let left_red: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1., 0.2, 0.2)));
    let back_green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 1., 0.2)));
    let right_blue: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.)));
    let upper_orange: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1., 0.5, 0.0)));
    let lower_teal: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(
        Vec3(-3., -2., 5.),
        Vec3(0., 0., -4.),
        Vec3(0., 4., 0.),
        &left_red,
    )));
    world.add(Rc::new(Quad::new(
        Vec3(-2., -2., 0.),
        Vec3(4., 0., 0.),
        Vec3(0., 4., 0.),
        &back_green,
    )));
    world.add(Rc::new(Quad::new(
        Vec3(3., -2., 1.),
        Vec3(0., 0., 4.),
        Vec3(0., 4., 0.),
        &right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Vec3(-2., 3., 1.),
        Vec3(4., 0., 0.),
        Vec3(0., 0., 4.),
        &upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Vec3(-2., -3., 5.),
        Vec3(4., 0., 0.),
        Vec3(0., 0., -4.),
        &lower_teal,
    )));

    // Set up camera
    let image_params = ImageParams {
        aspect_ratio: 1.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_params = CameraParams {
        vfov: 80.,
        lookfrom: Vec3(0., 0., 9.),
        lookat: Vec3(0., 0., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.,
    };

    let camera = Camera::new(image_params, camera_params);

    (world, camera)
}

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let noise_tex = Rc::new(NoiseTexture::new(4.));
    let noise_mat: Rc<dyn Material> = Rc::new(Lambertian::from_texture(noise_tex));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., -1000., 0.),
        1000.,
        &noise_mat,
    )));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., 2., 0.),
        2.,
        &noise_mat,
    )));

    let difflight: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Color::new(4., 4., 4.)));
    world.add(Rc::new(Sphere::stationary(
        Vec3(0., 7., 0.),
        2.,
        &difflight,
    )));
    world.add(Rc::new(Quad::new(
        Vec3(3., 1., -2.),
        Vec3(2., 0., 0.),
        Vec3(0., 2., 0.),
        &difflight,
    )));

    // Set up camera
    let image_params = ImageParams {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0., 0., 0.),
    };

    let camera_params = CameraParams {
        vfov: 20.,
        lookfrom: Vec3(26., 3., 6.),
        lookat: Vec3(0., 2., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.,
    };

    let camera = Camera::new(image_params, camera_params);

    (world, camera)
}

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let red: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3(0.65, 0.05, 0.05)));
    let white: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3(0.73, 0.73, 0.73)));
    let green: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3(0.12, 0.45, 0.15)));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Vec3(15., 15., 15.)));

    // add walls
    world.add(Rc::new(Quad::new(
        Vec3(555., 0., 0.),
        Vec3(0., 555., 0.),
        Vec3(0., 0., 555.),
        &green,
    )));

    world.add(Rc::new(Quad::new(
        Vec3(0., 0., 0.),
        Vec3(0., 555., 0.),
        Vec3(0., 0., 555.),
        &red,
    )));

    world.add(Rc::new(Quad::new(
        Vec3(343., 554., 332.),
        Vec3(-130., 0., 0.),
        Vec3(0., 0., -105.),
        &light,
    )));

    world.add(Rc::new(Quad::new(
        Vec3(0., 0., 0.),
        Vec3(555., 0., 0.),
        Vec3(0., 0., 555.),
        &white,
    )));

    world.add(Rc::new(Quad::new(
        Vec3(555., 555., 555.),
        Vec3(-555., 0., 0.),
        Vec3(0., 0., -555.),
        &white,
    )));

    world.add(Rc::new(Quad::new(
        Vec3(0., 0., 555.),
        Vec3(555., 0., 0.),
        Vec3(0., 555., 0.),
        &white,
    )));

    // add boxes
    let box1: Rc<dyn Hittable> = Rc::new(hittable_box(
        &Vec3(0., 0., 0.),
        &Vec3(165., 330., 165.),
        &white,
    ));
    let box1: Rc<dyn Hittable> = Rc::new(Translate::new(&box1, Vec3(265., 0., 295.)));

    world.add(box1);

    let box2: Rc<dyn Hittable> = Rc::new(hittable_box(
        &Vec3(0., 0., 0.),
        &Vec3(165., 165., 165.),
        &white,
    ));
    let box2: Rc<dyn Hittable> = Rc::new(Translate::new(&box2, Vec3(130., 0., 65.)));
    world.add(box2);

    // Set up camera
    let image_params = ImageParams {
        aspect_ratio: 1.,
        image_width: 300,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0., 0., 0.),
    };

    let camera_params = CameraParams {
        vfov: 40.,
        lookfrom: Vec3(278., 278., -800.),
        lookat: Vec3(278., 278., 0.),
        vup: Vec3(0., 1., 0.),
        focus_distance: 10.,
        defocus_angle: 0.,
    };

    let camera = Camera::new(image_params, camera_params);

    (world, camera)
}
