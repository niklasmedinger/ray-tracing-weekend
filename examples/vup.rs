use std::{
    io::{stdout, BufWriter},
    rc::Rc,
};

use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::Lambertian,
    point::Point,
    vec3::Vec3,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Default camera
    let camera = CameraBuilder::default()
        .with_orientation(
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 0.0, -1.0),
            Vec3::new(-1.0, 0.0, 0.0),
        )
        .build();

    // Materials
    let yellow_lambertian = Rc::new(Lambertian::new(Color::new(1.0, 0.02, 0.02)));
    let green_lambertian = Rc::new(Lambertian::new(Color::new(0.02, 1.0, 0.02)));

    // Objects in the world
    let sphere1 = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, yellow_lambertian.clone());
    let sphere2 = Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        green_lambertian.clone(),
    );

    // World
    let mut world = World::new();
    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));

    // Render
    let inner = stdout().lock();
    let writer = BufWriter::with_capacity(1024 * 32, inner);
    camera
        .render(&world, writer)
        .wrap_err("Failed to render image.")?;

    Ok(())
}
