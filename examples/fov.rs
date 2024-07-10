use std::rc::Rc;

use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::Lambertian,
    point::Point,
    PI,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Default camera
    let camera = CameraBuilder::default().fov(100.0).build();

    // Materials
    let yellow_lambertian = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let green_lambertian = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // Objects in the world
    let r = f32::cos(PI / 4.0);
    let sphere1 = Sphere::new(Point::new(-r, 0.0, -1.0), r, yellow_lambertian.clone());
    let sphere2 = Sphere::new(Point::new(r, 0.0, -1.0), r, green_lambertian.clone());

    // World
    let mut world = World::new();
    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));

    // Render
    camera.render(&world).wrap_err("Failed to render image.")?;

    Ok(())
}
