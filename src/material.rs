use crate::color::Color;
use crate::linalg::Vec3;
use crate::ray::{Hit, Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scattered = hit.normal() + Vec3::random_unit();
        if scattered.near_zero() {
            scattered = hit.normal();
        }
        Some(Scatter {
            ray: Ray::new(hit.point(), scattered),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = Vec3::reflect(ray.direction(), hit.normal());
        let reflected = reflected.normalized() + (self.fuzz * Vec3::random_unit());
        let ray = Ray::new(hit.point(), reflected);

        if ray.direction().dot(hit.normal()) <= 0.0 {
            return None;
        }

        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Color::WHITE;
        let refraction_index = if hit.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray.direction().normalized();
        let cos = f64::min((-unit_dir).dot(hit.normal()), 1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let should_reflect = refraction_index * sin > 1.0;
        let scattered =
            if should_reflect || Self::reflectance(cos, refraction_index) > rand::random::<f64>() {
                Vec3::reflect(unit_dir, hit.normal())
            } else {
                Vec3::refract(unit_dir, hit.normal(), refraction_index)
            };

        Some(Scatter {
            ray: Ray::new(hit.point(), scattered),
            attenuation,
        })
    }
}
