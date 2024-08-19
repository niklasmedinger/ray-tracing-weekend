use std::{
    io::{stdout, BufWriter},
    sync::Arc,
};

use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::{Lambertian, Metal},
    point::Point,
    vec3::Vec3,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Default camera
    let look_from = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .fov(20.0)
        .with_defocus(5.0, 4.0)
        .build();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // Objects in the world
    let ground_sphere = Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );
    let center_sphere = Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, material_center.clone());
    let left_sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let right_sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, material_right.clone());

    // World
    let mut world = World::new();
    world.push(Arc::new(ground_sphere));
    world.push(Arc::new(center_sphere));
    world.push(Arc::new(left_sphere));
    world.push(Arc::new(right_sphere));

    // Render
    let inner = stdout().lock();
    let writer = BufWriter::with_capacity(1024 * 32, inner);
    camera
        .render(&world, writer)
        .wrap_err("Failed to render image.")?;

    Ok(())
}
