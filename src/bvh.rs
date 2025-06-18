use std::rc::Rc;

use crate::{
    hittable::{self, HitRecord, Hittable},
    ray::Ray,
    util::{EMPTY, Interval},
    vec3::Vec3,
};

/// Axis-Aligned Bounding Box
///
/// Representes a 3-dimensional parallelepiped
/// bounded by three pairs of plains, each pair
/// aligned with a coordinate plane and defined by an Interval.
#[derive(Clone, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    /// Returns an empty AABB, where each bounding interval is empty.
    pub fn empty() -> Self {
        Self {
            x: EMPTY,
            y: EMPTY,
            z: EMPTY,
        }
    }

    /// Consumes three Intervals and returns a new AABB defined by
    /// those intervals.
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// Treat the points a and b as extrema for the bounding box, so
    /// we don't require a particular minimum/maximum coordinate order.
    /// Consumes a and b.
    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        let x = if a.0 <= b.0 {
            Interval::new(a.0, b.0)
        } else {
            Interval::new(b.0, a.0)
        };
        let y = if a.1 <= b.1 {
            Interval::new(a.1, b.1)
        } else {
            Interval::new(b.1, a.1)
        };
        let z = if a.2 <= b.2 {
            Interval::new(a.2, b.2)
        } else {
            Interval::new(b.2, a.2)
        };
        Self { x, y, z }
    }

    pub fn from_boxes(box1: &Self, box2: &Self) -> Self {
        let x = Interval::from_intervals(&box1.x, &box2.x);
        let y = Interval::from_intervals(&box1.y, &box2.y);
        let z = Interval::from_intervals(&box1.z, &box2.z);
        Self { x, y, z }
    }

    /// Returns a requested axis interval
    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("axis index out of range"),
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = r.origin();
        let ray_orig = vec![ray_orig.0, ray_orig.1, ray_orig.2];
        let ray_dir = r.dir();
        let ray_dir = vec![ray_dir.0, ray_dir.1, ray_dir.2];

        let mut min = ray_t.min();
        let mut max = ray_t.max();
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1. / ray_dir[axis as usize]; // this can be f64::INFINITY or f64::NEG_INFINITY

            let t0 = (ax.min() - ray_orig[axis]) * adinv;
            let t1 = (ax.max() - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > min {
                    min = t0
                };
                if t1 < max {
                    max = t1
                };
            } else {
                if t1 > min {
                    min = t1
                };
                if t0 < max {
                    max = t0
                };
            }

            if max <= min {
                return false;
            }
        }
        true
    }
}

/// Bounding Volume Hierarcy node
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis_index: usize = rand::random_range(0..3);
        let span = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if span == 1 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start]);
        } else if span == 2 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start + 1]);
        } else {
            objects.sort_by(|a, b| hittable::box_compare(a, b, axis_index));

            let mid = start + span / 2;
            left = Rc::new(BVHNode::new(objects, start, mid));
            right = Rc::new(BVHNode::new(objects, mid, end));
        }

        let bbox = AABB::from_boxes(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        };

        let left_hit = self.left.hit(ray, ray_t);
        let new_max = match &left_hit {
            Some(rec) => rec.t,
            None => ray_t.max(),
        };
        let new_ray_t = Interval::new(ray_t.min(), new_max);

        if let Some(hit) = self.right.hit(ray, &new_ray_t) {
            return Some(hit);
        };

        left_hit
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_works() {
        let x = Interval::new(-1., 1.);
        let y = x.clone();
        let z = x.clone();
        let aabb = AABB::new(x, y, z);

        let origin = Vec3(0., 0., -5.);
        let dir = Vec3(0., 0., 1.);
        let ray = Ray::new(origin, dir, 0.);

        let ray_t = Interval::new(0., 100.);
        assert!(aabb.hit(&ray, &ray_t));

        let origin = Vec3(1., 1., -5.);
        let dir = Vec3(0., 0., 1.);
        let ray = Ray::new(origin, dir, 0.);
        assert!(!aabb.hit(&ray, &ray_t))
    }
}
