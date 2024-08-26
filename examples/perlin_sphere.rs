use std::{
    io::{stdout, BufWriter},
    sync::Arc,
};

use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::CameraBuilder,
    hittable::{Sphere, World},
    material::Lambertian,
    point::Point,
    texture::NoiseTexture,
    vec3::Vec3,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Default camera
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .fov(20.0)
        .image_width(1080)
        .samples_per_pixel(100)
        .max_depth(50)
        .build();

    // Materials
    let noise_texture = Arc::new(NoiseTexture::new(4.0));

    // World
    let mut world = World::new();
    world.push(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(noise_texture.clone())),
    )));
    world.push(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(noise_texture)),
    )));

    // Render
    let inner = stdout().lock();
    let writer = BufWriter::with_capacity(1024 * 32, inner);
    camera
        .render(&world, writer)
        .wrap_err("Failed to render image.")?;

    Ok(())
}
