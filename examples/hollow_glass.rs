use std::{
    io::{stdout, BufWriter},
    rc::Rc,
};

use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::{Dielectric, Lambertian, Metal},
    point::Point,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Default camera
    let camera = CameraBuilder::default().build();

    // Materials
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let material_right = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));

    // Objects in the world
    let ground_sphere = Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );
    let center_sphere = Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, material_center.clone());
    let left_sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let right_sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, material_right.clone());
    let bubble = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.4, material_bubble.clone());

    // World
    let mut world = World::new();
    world.push(Box::new(center_sphere));
    world.push(Box::new(ground_sphere));
    world.push(Box::new(left_sphere));
    world.push(Box::new(right_sphere));
    world.push(Box::new(bubble));

    // Render
    let inner = stdout().lock();
    let writer = BufWriter::with_capacity(1024 * 32, inner);
    camera
        .render(&world, writer)
        .wrap_err("Failed to render image.")?;

    Ok(())
}
