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
            fuzz: f64::max(fuzz, 1.0),
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
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Color::WHITE;
        let refraction_ratio = if hit.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let refracted = Vec3::refract(ray.direction(), hit.normal(), refraction_ratio);

        Some(Scatter {
            ray: Ray::new(hit.point(), refracted),
            attenuation,
        })
    }
}
