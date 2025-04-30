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
    pixel_sample_scale: f64,

    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const FOV: f64 = 20.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCES: u32 = 50;
    const DEFOCUS_ANGLE: f64 = 0.1;
    const FOCUS_DIST: f64 = 10.0;

    pub fn new(camera_position: Vec3, look_at: Vec3, up: Vec3, image_width: usize) -> Self {
        let image_height = (image_width as f64 / Self::ASPECT_RATIO).max(1.0) as usize;

        let theta = Self::FOV.to_radians();
        let h = f64::tan(theta / 2.0);

        let defocus_angle = Self::DEFOCUS_ANGLE;
        let focus_dist = Self::FOCUS_DIST;

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (camera_position - look_at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u).normalized();

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_step_u = viewport_u / image_width as f64;
        let pixel_step_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            camera_position - (focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let top_left_pixel = viewport_upper_left + 0.5 * (pixel_step_u + pixel_step_v);

        let defocus_radius = focus_dist * f64::tan((defocus_angle / 2.0).to_radians());
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            position: camera_position,
            top_left_pixel,
            pixel_step_u,
            pixel_step_v,
            pixel_sample_scale: 1.0 / (Self::SAMPLES_PER_PIXEL as f64),
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &impl HitObject) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Color::BLACK;
                for _ in 0..Self::SAMPLES_PER_PIXEL {
                    color += self.ray_color(&self.get_ray(i, j), world, 0);
                }
                output_pixel(color * self.pixel_sample_scale);
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &impl HitObject, bounces: u32) -> Color {
        if bounces >= Self::MAX_BOUNCES {
            return Color::BLACK;
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter) = hit.material().scatter(ray, &hit) {
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

        let origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.sample_defocus_disk()
        };
        let dir = pixel_sample - self.position;

        Ray::new(origin, dir)
    }

    fn sample_defocus_disk(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.position + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
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
