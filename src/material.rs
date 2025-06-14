use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scattered_direction = rec.normal + Vec3::random_unit_vector();
        if scattered_direction.near_zero() {
            scattered_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scattered_direction);
        Some(ScatterResult {
            scattered,
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let scattered = Ray::new(rec.point, r_in.dir().reflect(&rec.normal));
        Some(ScatterResult {
            scattered,
            attenuation: self.albedo,
        })
    }
}
