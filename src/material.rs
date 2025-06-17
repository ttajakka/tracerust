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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scattered_direction = rec.normal + Vec3::random_unit_vector();
        if scattered_direction.near_zero() {
            scattered_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scattered_direction, r_in.time());
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
        let scattered = Ray::new(rec.point, reflected, r_in.time());
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

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1. - refraction_index) / (1. + refraction_index);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
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
        let cos_theta = -unit_direction.dot(&rec.normal.unit());
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.;

        let dir =
            if cannot_refract || Self::reflectance(cos_theta, ri) > rand::random::<f64>() {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal.unit(), ri).unit()
            };

        Some(ScatterResult {
            scattered: Ray::new(rec.point, dir, r_in.time()),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
