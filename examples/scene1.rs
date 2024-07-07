use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::Camera,
    hittable::{Sphere, World},
    point::Point,
};

fn main() -> color_eyre::Result<()> {
    // Default camera
    let camera = Camera::default();

    // Objects in the world
    let sphere1 = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);

    // World
    let mut world = World::new();
    world.push(&sphere1);
    world.push(&sphere2);

    // Render
    camera.render(&world).wrap_err("Failed to render image.")?;

    Ok(())
}
