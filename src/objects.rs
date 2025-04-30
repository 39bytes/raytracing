use std::sync::Arc;

use crate::{
    linalg::{Interval, Vec3},
    material::Material,
    ray::{Hit, HitObject, Ray},
};

pub struct Sphere {
    position: Vec3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Sphere {
            position,
            radius,
            material,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl HitObject for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        let cq = self.position - ray.origin();
        let d = ray.direction();

        let a = d.magnitude_squared();
        let h = d.dot(cq);
        let c = cq.magnitude_squared() - self.radius * self.radius;

        let discrim = h * h - a * c;
        if discrim < 0.0 {
            return None;
        }

        let sqrt = discrim.sqrt();

        let root1 = (h - sqrt) / a;
        let root2 = (h + sqrt) / a;

        let root = if ray_t.surrounds(root1) {
            root1
        } else if ray_t.surrounds(root2) {
            root2
        } else {
            return None;
        };

        let point = ray.at(root);
        let normal = (point - self.position) / self.radius;

        Some(Hit::new(root, point, ray, normal, self.material.clone()))
    }
}

pub struct ObjectGroup {
    objects: Vec<Box<dyn HitObject + Send + Sync>>,
}

impl ObjectGroup {
    pub fn new() -> Self {
        ObjectGroup {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn HitObject + Send + Sync>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl HitObject for ObjectGroup {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        let mut closest_hit = None;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(
                ray,
                Interval::new(ray_t.min, closest_hit.as_ref().map_or(ray_t.max, Hit::t)),
            ) {
                closest_hit = Some(hit)
            }
        }

        closest_hit
    }
}
