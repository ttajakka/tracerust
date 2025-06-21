use crate::{bvh::AABB, material::Material, ray::Ray, util::Interval, vec3::Vec3};
use std::rc::Rc;
use std::cmp::{ Ordering};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3,
        mat: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.dir().dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            mat: Rc::clone(&mat),
            t,
            u: 0.,
            v: 0.,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}

pub fn box_compare(a: &Rc::<dyn Hittable>, b: &Rc::<dyn Hittable>, axis_index: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);

    if a_axis_interval.min() < b_axis_interval.min() {
        return Ordering::Less
    } else {
        return Ordering::Greater
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![], bbox: AABB::empty()}
    }

    pub fn from_hittable(bvh: Rc<dyn Hittable>) -> Self {
        Self { objects: vec![Rc::clone(&bvh)], bbox: bvh.bounding_box().clone()}
    }

    pub fn count(&self) -> usize {
        self.objects.len()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec_out = None;
        let mut closest_so_far = ray_t.max();

        for o in &self.objects {
            if let Some(rec) = o.hit(ray, ray_t) {
                if rec.t < closest_so_far {
                    closest_so_far = rec.t;
                    rec_out = Some(rec);
                }
            };
        }

        rec_out
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.hit(ray, ray_t)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Rc<dyn Material>,
    bbox: AABB
}

impl Sphere {
    pub fn stationary(center: Vec3, radius: f64, mat: &Rc<dyn Material>) -> Self {
        let center_ray = Ray::new(center, Vec3(0., 0., 0.), 0.);
        let rvec = Vec3(radius, radius, radius);
        Self {
            center: center_ray,
            radius,
            material: Rc::clone(&mat),
            bbox: AABB::from_points(center - rvec, center + rvec)
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.);
        let rvec = Vec3(radius, radius, radius);
        let box1 = AABB::from_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box2 = AABB::from_points(center.at(1.) - rvec, center.at(1.) + rvec);
        Self {
            center,
            radius,
            material: Rc::clone(&mat),
            bbox: AABB::from_boxes(&box1, &box2)
        }
    }

    pub fn center(&self) -> Ray {
        self.center.clone()
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = current_center - ray.origin();
        let a = ray.dir().length_squared();
        let h = ray.dir().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        // Check if the quadratic has solutions
        let disc = h * h - a * c;

        if disc < 0. {
            return None;
        }

        let sqrtd = disc.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_t.min() || root >= ray_t.max() {
            root = (h + sqrtd) / a;
            if root <= ray_t.min() || root >= ray_t.max() {
                return None;
            }
        }

        return Some(HitRecord::new(
            ray.at(root),
            root,
            ray,
            (ray.at(root) - current_center) / self.radius,
            Rc::clone(&self.material),
        ));
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
