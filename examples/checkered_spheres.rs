use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::Lambertian,
    point::Point,
    texture::CheckeredTexture,
    vec3::Vec3,
};

fn main() {
    // Default camera
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .fov(20.0)
        .image_width(1080)
        .samples_per_pixel(200)
        .max_depth(50)
        .build();

    // Materials
    let checkered = Arc::new(CheckeredTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    // World
    let mut world = World::new();
    world.push(Arc::new(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checkered.clone())),
    )));
    world.push(Arc::new(Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checkered)),
    )));

    // Render
    let file_name = "checkered_spheres.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
