use std::rc::Rc;

use camera::Camera;
use color::Color;
use linalg::Vec3;
use objects::{ObjectGroup, Sphere};

mod camera;
mod color;
mod linalg;
mod material;
mod objects;
mod ray;

fn main() {
    env_logger::init();

    let mut world = ObjectGroup::new();

    let material_ground = Rc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::with_z(-1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new(16.0 / 9.0, 800, 10, 50);
    camera.render(&world);
}
