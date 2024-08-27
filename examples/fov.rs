use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::Lambertian,
    point::Point,
    PI,
};

fn main() {
    // Default camera
    let camera = CameraBuilder::default().fov(100.0).build();

    // Materials
    let yellow_lambertian = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let green_lambertian = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // Objects in the world
    let r = f32::cos(PI / 4.0);
    let sphere1 = Sphere::new(Point::new(-r, 0.0, -1.0), r, yellow_lambertian.clone());
    let sphere2 = Sphere::new(Point::new(r, 0.0, -1.0), r, green_lambertian.clone());

    // World
    let mut world = World::new();
    world.push(Arc::new(sphere1));
    world.push(Arc::new(sphere2));

    // Render
    let file_name = "fov.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
