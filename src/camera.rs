use std::io::{BufWriter, Write};

use indicatif::ProgressBar;

use crate::{
    hittable::{Hittable, World},
    random_0_1_f32,
    ray::Ray,
    Color, Interval, Point, Vec3, INFINITY,
};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    pixel_samples_scale: f32,
    center: Point,
    pixel_00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32, samples_per_pixel: u32) -> Self {
        // Calculate image height
        let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        // Camera
        let focal_length: f32 = 1.0;
        let focal_length_vec = Vec3::new(0.0, 0.0, focal_length);
        let center = Point::new(0.0, 0.0, 0.0);

        // Viewport
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = *center - focal_length_vec - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = (viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v)).into();

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &World) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        // Create progress bar
        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        // Lock stdout for better writing performance.
        let mut writer = BufWriter::with_capacity(1024 * 64, stdout.lock());
        write!(
            writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    *pixel_color += *self.ray_color(&ray, world);
                }
                let pixel_color: Color = (self.pixel_samples_scale * *pixel_color).into();
                write!(writer, "{} ", pixel_color,)?;
                bar.inc(1);
            }
            writeln!(writer, "")?;
        }
        bar.finish_and_clear();
        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = *self.pixel_00_loc
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - *ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_0_1_f32() - 0.5, random_0_1_f32() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, world: &World) -> Color {
        let interval = Interval::new(0.0, INFINITY);
        if let Some(hit_record) = world.hit(ray, interval) {
            return (0.5 * (hit_record.normal() + Vec3::new(1.0, 1.0, 1.0))).into();
        }
        let unit_direction = ray.direction().unit();
        let a: f32 = 0.5 * (unit_direction.y() + 1.0);
        ((1.0 - a) * *Color::white() + a * *Color::new(0.5, 0.7, 1.0)).into()
    }
}
