use crate::{
    bvh::AABB,
    material::Material,
    ray::Ray,
    util::{Interval, UNIT, degrees_to_radians},
    vec3::Vec3,
};
use core::f64;
use std::cmp::Ordering;
use std::rc::Rc;

const PI: f64 = f64::consts::PI;

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
        u: f64,
        v: f64,
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
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}

pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);

    if a_axis_interval.min() < b_axis_interval.min() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: vec![],
            bbox: AABB::empty(),
        }
    }
}

impl HittableList {
    pub fn from_hittable(bvh: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![Rc::clone(&bvh)],
            bbox: bvh.bounding_box().clone(),
        }
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
    bbox: AABB,
}

impl Sphere {
    pub fn stationary(center: Vec3, radius: f64, mat: &Rc<dyn Material>) -> Self {
        let center_ray = Ray::new(center, Vec3(0., 0., 0.), 0.);
        let rvec = Vec3(radius, radius, radius);
        Self {
            center: center_ray,
            radius,
            material: Rc::clone(mat),
            bbox: AABB::from_points(center - rvec, center + rvec),
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
            bbox: AABB::from_boxes(&box1, &box2),
        }
    }

    pub fn center(&self) -> Ray {
        self.center.clone()
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn get_uv(&self, p: &Vec3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = libm::atan2(-p.z(), p.x()) + PI;
        (phi / (2. * PI), theta / PI)
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

        let outward_normal = (ray.at(root) - current_center) / self.radius;
        let (u, v) = self.get_uv(&outward_normal);

        Some(HitRecord::new(
            ray.at(root),
            root,
            ray,
            outward_normal,
            u,
            v,
            Rc::clone(&self.material),
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64, // a x + b y + c z = D
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: &Rc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);
        let bbox = Self::get_bounding_box(q, u, v);

        Self {
            q,
            u,
            v,
            w,
            mat: Rc::clone(mat),
            bbox,
            normal,
            d,
        }
    }

    fn get_bounding_box(q: Vec3, u: Vec3, v: Vec3) -> AABB {
        let bbox_diagonal_1 = AABB::from_points(q, q + u + v);
        let bbox_diagonal_2 = AABB::from_points(q + u, q + v);
        AABB::from_boxes(&bbox_diagonal_1, &bbox_diagonal_2)
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<crate::hittable::HitRecord> {
        let denom = self.normal.dot(&ray.dir());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return None if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(&ray.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let point = ray.at(t);
        let planar_component = point - self.q;
        let alpha = self.w.dot(&planar_component.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_component));

        if !UNIT.contains(alpha) || !UNIT.contains(beta) {
            return None;
        }

        Some(HitRecord::new(
            point,
            t,
            ray,
            self.normal,
            alpha,
            beta,
            Rc::clone(&self.mat),
        ))
    }
}

/// Returns a 3D box (six sides) that contains the two opposite vertices a & b.
pub fn hittable_box(a: &Vec3, b: &Vec3, mat: &Rc<dyn Material>) -> HittableList {
    let mut sides = HittableList::default();

    let min = Vec3(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Vec3(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3(max.x() - min.x(), 0., 0.);
    let dy = Vec3(0., max.y() - min.y(), 0.);
    let dz = Vec3(0., 0., max.z() - min.z());

    sides.add(Rc::new(Quad::new(
        Vec3(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat,
    )));
    sides.add(Rc::new(Quad::new(
        Vec3(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat,
    )));
    sides.add(Rc::new(Quad::new(
        Vec3(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat,
    )));
    sides.add(Rc::new(Quad::new(
        Vec3(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat,
    )));
    sides.add(Rc::new(Quad::new(
        Vec3(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat,
    )));
    sides.add(Rc::new(Quad::new(
        Vec3(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat,
    )));

    sides
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: &Rc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            object: Rc::clone(object),
            offset,
            bbox: object.bounding_box().clone() + offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.dir(), ray.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        match self.object.hit(&offset_ray, ray_t) {
            None => None,
            Some(mut rec) => {
                rec.point += self.offset;
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: &Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = vec![f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = vec![f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.x.max() + (1. - i as f64) * bbox.x.min();
                    let y = (j as f64) * bbox.y.max() + (1. - j as f64) * bbox.y.min();
                    let z = (k as f64) * bbox.z.max() + (1. - k as f64) * bbox.z.min();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = vec![newx, y, newz];
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c])
                    }
                }
            }
        }

        let min = Vec3(min[0], min[1], min[2]);
        let max = Vec3(max[0], max[1], max[2]);

        let bbox = AABB::from_points(min, max);

        Self {
            object: Rc::clone(object),
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.
        let co = self.cos_theta;
        let si = self.sin_theta;

        let orig = ray.origin();
        let rdir = ray.dir();

        let origin = Vec3(
            co * orig.x() - si * orig.z(),
            orig.y(),
            si * orig.x() + co * orig.z(),
        );

        let dir = Vec3(
            co * rdir.x() - si * rdir.z(),
            rdir.y(),
            si * rdir.x() + co * rdir.z(),
        );

        let rotated_ray = Ray::new(origin, dir, ray.time());

        match self.object.hit(&rotated_ray, ray_t) {
            None => None,
            Some(mut rec) => {
                rec.point = Vec3(
                    co * rec.point.x() + si * rec.point.z(),
                    rec.point.y(),
                    -si * rec.point.x() + co * rec.point.z(),
                );
                rec.normal = Vec3(
                    co * rec.normal.x() + si * rec.normal.z(),
                    rec.normal.y(),
                    -si * rec.normal.x() + co * rec.normal.z(),
                );
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Dielectric;

    #[test]
    fn get_uv_works() {
        let mat: Rc<dyn Material> = Rc::new(Dielectric::new(1.0));
        let sphere = Sphere::stationary(Vec3(0., 0., 0.), 1., &mat);
        assert_eq!(sphere.get_uv(&Vec3(1., 0., 0.)), (0.5, 0.5));
        assert_eq!(sphere.get_uv(&Vec3(0., 1., 0.)), (0.5, 1.0));
        assert_eq!(sphere.get_uv(&Vec3(0., 0., 1.)), (0.25, 0.5));
        assert_eq!(sphere.get_uv(&Vec3(-1., 0., 0.)), (0., 0.5));
        assert_eq!(sphere.get_uv(&Vec3(0., -1., 0.)), (0.5, 0.));
        assert_eq!(sphere.get_uv(&Vec3(0., 0., -1.)), (0.75, 0.5));
    }
}
