use std::rc::Rc;

use crate::{
    linalg::{Interval, Vec3},
    material::Material,
};

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct Hit {
    point: Vec3,
    normal: Vec3,
    t: f64,
    material: Rc<dyn Material>,
    front_face: bool,
}

impl Hit {
    pub fn new(
        t: f64,
        point: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Hit {
            t,
            point,
            normal,
            front_face,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

pub trait HitObject {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit>;
}
