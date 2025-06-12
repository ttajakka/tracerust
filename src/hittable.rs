use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        match ray.dir().dot(outward_normal) < 0. {
            true => self.normal = outward_normal,
            false => self.normal = -outward_normal,
        }
    }

    pub fn new(point: Vec3, t: f64, ray: Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.dir().dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.dir().length_squared();
        let h = ray.dir().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        // Check if the quadratic has solutions
        let disc = h * h - a * c;

        if disc < 0. {
            return None;
        }

        let sqrtd = disc.sqrt();
        let root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            let root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        return Some(HitRecord::new(
            ray.at(root),
            root,
            *ray,
            (ray.at(root) - self.center) / self.radius,
        ));
    }
}
