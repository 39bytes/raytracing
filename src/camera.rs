use crate::{
    linalg::{Interval, Vec3},
    ray::{HitObject, Ray},
};

type Color = Vec3;

pub struct Camera {
    image_width: usize,

    image_height: usize,
    position: Vec3,
    top_left_pixel: Vec3,
    pixel_step_u: Vec3,
    pixel_step_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_position = Vec3::ZERO;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_step_u = viewport_u / image_width as f64;
        let pixel_step_v = viewport_v / image_height as f64;

        let viewport_upper_left = camera_position
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let top_left_pixel = viewport_upper_left + 0.5 * (pixel_step_u + pixel_step_v);

        Camera {
            image_width,
            image_height,
            position: camera_position,
            top_left_pixel,
            pixel_step_u,
            pixel_step_v,
        }
    }

    pub fn render(&self, world: &impl HitObject) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel = self.top_left_pixel
                    + (i as f64 * self.pixel_step_u)
                    + (j as f64 * self.pixel_step_v);
                let ray_dir = pixel - self.position;
                let ray = Ray::new(self.position, ray_dir);

                output_pixel(self.ray_color(&ray, world));
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &impl HitObject) -> Color {
        if let Some(hit) = world.hit(ray, Interval::new(0.0, 10.0)) {
            let norm = hit.normal();
            return 0.5 * Color::new(norm.x() + 1.0, norm.y() + 1.0, norm.z() + 1.0);
        }

        let dir = ray.direction().normalized();
        let a = 0.5 * (dir.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

fn output_pixel(color: Color) {
    let ir = 255.999f64 * color.x();
    let ig = 255.999f64 * color.y();
    let ib = 255.999f64 * color.z();

    println!("{} {} {}", ir.floor(), ig.floor(), ib.floor());
}
