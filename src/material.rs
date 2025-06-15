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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 0. {
            0.
        } else if fuzz > 1. {
            1.
        } else {
            fuzz
        };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected =
            r_in.dir().reflect(&rec.normal).unit() + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new(rec.point, reflected);
        match scattered.dir().dot(&rec.normal) > 0. {
            true => Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            }),
            false => None,
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let ri = if rec.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.dir().unit();
        let refracted = unit_direction.refract(&rec.normal.unit(), ri).unit();
        Some(ScatterResult {
            scattered: Ray::new(rec.point, refracted),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
