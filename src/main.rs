use camera::Camera;
use linalg::Vec3;
use objects::{ObjectGroup, Sphere};

mod camera;
mod linalg;
mod objects;
mod ray;

fn main() {
    env_logger::init();

    let mut world = ObjectGroup::new();
    world.add(Box::new(Sphere::new(Vec3::with_z(-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}
