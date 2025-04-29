use crate::{
    color::Color,
    linalg::{Interval, Vec3},
    ray::{HitObject, Ray},
};

pub struct Camera {
    image_width: usize,

    image_height: usize,
    position: Vec3,
    top_left_pixel: Vec3,
    pixel_step_u: Vec3,
    pixel_step_v: Vec3,
    samples_per_pixel: u32,
    pixel_sample_scale: f64,
    max_bounces: u32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: u32,
        max_bounces: u32,
    ) -> Self {
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
            samples_per_pixel,
            pixel_sample_scale: 1.0 / (samples_per_pixel as f64),
            max_bounces,
        }
    }

    pub fn render(&self, world: &impl HitObject) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Color::BLACK;
                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(i, j), world, 0);
                }
                output_pixel(color * self.pixel_sample_scale);
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &impl HitObject, bounces: u32) -> Color {
        if bounces >= self.max_bounces {
            return Color::BLACK;
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter) = hit.material().scatter(&ray, &hit) {
                return scatter.attenuation * self.ray_color(&scatter.ray, world, bounces + 1);
            }
        }

        let dir = ray.direction().normalized();
        let a = 0.5 * (dir.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.top_left_pixel
            + ((i as f64 + offset.x()) * self.pixel_step_u)
            + ((j as f64 + offset.y()) * self.pixel_step_v);

        Ray::new(self.position, pixel_sample - self.position)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }
}

fn output_pixel(color: Color) {
    let (r, g, b) = color.gamma_transform().to_bytes();
    println!("{r} {g} {b}");
}
