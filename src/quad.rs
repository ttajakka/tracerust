use std::rc::Rc;

use crate::{
    bvh::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    util::{Interval, UNIT},
    vec3::Vec3,
};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::DiffuseLight;
    use crate::ray::Ray;

    #[test]
    fn hit_works() {
        let difflight: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Color::new(4., 4., 4.)));
        let quad = Rc::new(Quad::new(
            Vec3(3., 2., 0.),
            Vec3(2., 0., 0.),
            Vec3(0., 2., 0.),
            &difflight,
        ));

        let ray = Ray::new(Vec3(3., 2., 1.), Vec3(0., 0., -1.), 0.);
        let ray_t = Interval::new(0., 100.);
        let hit = quad.hit(&ray, &ray_t);

        match hit {
            Some(_) => {
                panic!()
            }
            None => {
                panic!()
            }
        }
    }
}
