use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::{Dielectric, Lambertian, Metal},
    point::Point,
};

fn main() {
    // Default camera
    let camera = CameraBuilder::default().build();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let material_right = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));

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
    world.push(Arc::new(center_sphere));
    world.push(Arc::new(ground_sphere));
    world.push(Arc::new(left_sphere));
    world.push(Arc::new(right_sphere));
    world.push(Arc::new(bubble));

    // Render
    let file_name = "hollow_glass.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
