//! This module contains the camera code which renders the image.
//! See [Camera] for the implementation of the camera, see [CameraBuilder]
//! for creating cameras.
use std::{fmt::Debug, io::Write};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    color::Color,
    degrees_to_radians,
    hittable::{Hittable, World},
    interval::Interval,
    point::Point,
    random_0_1_f32, random_in_unit_disk,
    ray::Ray,
    vec3::Vec3,
    INFINITY,
};

/// A camera that views the world.
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    /// The width of the image we want to render.
    image_width: u32,
    /// The height of the image we want to render.
    image_height: u32,
    /// The amount of samples we sample per pixel for antialiasing.
    samples_per_pixel: u32,
    /// The weight each sample has in the color computation of a pixel.
    pixel_samples_scale: f32,
    /// The maximum amount of times the traycing of a [Ray] can recurse. I.e.,
    /// how often a [Ray] can be scattered inside of the world.
    max_depth: u32,
    /// The center of the camera.
    center: Point,
    /// The location of the pixel with coordinates (0, 0) in the world.
    pixel_00_loc: Point,
    /// The distance between each pixel in the x direction.
    pixel_delta_u: Vec3,
    /// The distance between each pixel in the y direction.
    pixel_delta_v: Vec3,
    /// The variation angle of rays through each pixel.
    defocus_angle: f32,
    /// The defocus desk horizontal radius.
    defocus_disk_u: Vec3,
    /// The defocus desk vertical radius.
    defocus_disk_v: Vec3,
    /// Toggle to hide the progress bar.
    hide_progress: bool,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        fov: f32,
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        defocus_angle: f32,
        focus_distance: f32,
        hide_progress: bool,
    ) -> Self {
        // Calculate image height
        let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        let center = look_from;

        // Viewport
        let theta = degrees_to_radians(fov);
        let h = f32::tan(theta / 2.0);
        let viewport_height: f32 = 2.0 * h * focus_distance;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit().as_vec3();
        let u = (vup.cross(w)).unit().as_vec3();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            max_depth,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
            hide_progress,
        }
    }

    /// Render the [World] to stdout in the `.ppm` format. Note that this
    /// renders a progress bar to stderr.
    /// Yes, this is not behavior you want from a library function, but we will
    /// only be consumed by our own applications :)
    pub fn render(&self, world: &World, mut writer: impl Write) -> std::io::Result<()> {
        // Create progress bar
        let bar = if self.hide_progress {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        // Set the style of the progress bar
        let style = ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] [{wide_bar}] ({percent}% {eta_precise})")
            .expect("Malformed progress bar template.");
        bar.set_style(style);

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
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }
                write!(writer, "{} ", self.pixel_samples_scale * pixel_color)?;
                bar.inc(1);
            }
        }
        bar.finish_and_clear();
        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_0_1_f32();
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_0_1_f32() - 0.5, random_0_1_f32() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }

    fn ray_color(ray: &Ray, depth: u32, world: &World) -> Color {
        if depth == 0 {
            return Color::black();
        }
        let interval = Interval::new(0.001, INFINITY);
        if let Some(hit_record) = world.hit(ray, interval) {
            let (scattered, attenuation) = hit_record.material().scatter(ray, hit_record.copy());
            return attenuation * Self::ray_color(&scattered, depth - 1, world);
        }
        let unit_direction = ray.direction().unit();
        let a: f32 = 0.5 * (unit_direction.as_vec3().y() + 1.0);
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }
}

/// A builder for [Camera].
#[derive(Debug, Copy, Clone)]
pub struct CameraBuilder {
    /// The aspect ratio for the [Camera].
    aspect_ratio: f32,
    /// The image width for the [Camera].
    image_width: u32,
    /// The samples per pixel used for antialising by the [Camera].
    samples_per_pixel: u32,
    /// The max recursion depth for computing the color of a [Ray] by the [Camera].
    /// I.e., the maximum amount of scattered rays produced by an initial [Ray].
    max_depth: u32,
    /// The field-of-view for the [Camera].
    fov: f32,
    /// The origin for the [Camera].
    look_from: Point,
    /// The point the [Camera] looks at.
    look_at: Point,
    /// The [Vec3] that is considered `up` by the [Camera].
    vup: Vec3,
    /// Variation angle of rays through each pixel.
    defocus_angle: f32,
    /// Distance from camera look_from point to plane of perfect focus.
    focus_distance: f32,
    /// Toggle to hide the progress bar.
    hide_progress: bool,
}

impl CameraBuilder {
    /// Construct a new, default builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a [Camera] from this builder.
    pub fn build(self) -> Camera {
        Camera::new(
            self.aspect_ratio,
            self.image_width,
            self.samples_per_pixel,
            self.max_depth,
            self.fov,
            self.look_from,
            self.look_at,
            self.vup,
            self.defocus_angle,
            self.focus_distance,
            self.hide_progress,
        )
    }

    /// Set the aspect ratio of the [Camera].
    pub fn aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Set the image width of the [Camera].
    pub fn image_width(&mut self, image_width: u32) -> &mut Self {
        self.image_width = image_width;
        self
    }

    /// Set the samples per pixel used by the [Camera] for antialiasing.
    pub fn samples_per_pixel(&mut self, samples_per_pixel: u32) -> &mut Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    /// Set maximum amount of times the traycing of a [Ray] can recurse. I.e.,
    /// how often a [Ray] can be scattered inside of the world.
    pub fn max_depth(&mut self, max_depth: u32) -> &mut Self {
        self.max_depth = max_depth;
        self
    }

    /// Set the field of view of the [Camera].
    pub fn fov(&mut self, fov: f32) -> &mut Self {
        self.fov = fov;
        self
    }

    /// Set the field of view of the [Camera].
    pub fn hide_progress(&mut self, hide_progress: bool) -> &mut Self {
        self.hide_progress = hide_progress;
        self
    }

    /// Customize where the [Camera] is centered and where it looks to.
    ///
    /// * `look_from` - The origin the [Camera] is centered at.
    /// * `look_at` - The [Point] the [Camera] is looking at.
    /// * `vup` - The vector that defines camera-up. I.e., where up in the rendered image will be.
    pub fn with_orientation(&mut self, look_from: Point, look_at: Point, vup: Vec3) -> &mut Self {
        self.look_from = look_from;
        self.look_at = look_at;
        self.vup = vup;
        self
    }

    /// Customize defocus (also called depth of field) of the [Camera].
    ///
    /// * `defocus_angle` - The angle of the cone originating at the plane of
    ///    perfect focus with apex at the camera center.
    ///    A greater angle means a bigger radius of the defocus disc.
    /// * `focus_distance` - The distance from the camera center to the plane of
    ///    perfect focus.
    pub fn with_defocus(&mut self, defocus_angle: f32, focus_distance: f32) -> &mut Self {
        self.defocus_angle = defocus_angle;
        self.focus_distance = focus_distance;
        self
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 800,
            samples_per_pixel: 50,
            max_depth: 25,
            fov: 90.0,
            look_from: Point::new(0.0, 0.0, 0.0),
            look_at: Point::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            hide_progress: false,
        }
    }
}
