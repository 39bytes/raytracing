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

    let ground_material = Rc::new(material::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mat: f64 = rand::random();
            let position = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (position - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                match mat {
                    ..0.8 => {
                        let albedo = Color::random() * Color::random();
                        let material = Rc::new(material::Lambertian::new(albedo));
                        world.add(Box::new(Sphere::new(position, 0.2, material)));
                    }
                    0.8..0.95 => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz: f64 = rand::random_range(0.0..0.5);
                        let material = Rc::new(material::Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(position, 0.2, material)));
                    }
                    _ => {
                        let material = Rc::new(material::Dielectric::new(1.5));
                        world.add(Box::new(Sphere::new(position, 0.2, material)));
                    }
                }
            }
        }
    }

    let material1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        400,
    );
    camera.render(&world);
}
