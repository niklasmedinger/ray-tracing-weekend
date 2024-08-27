use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    hittable::{Sphere, World},
    material::Lambertian,
    point::Point,
    texture::NoiseTexture,
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
        .image_width(800)
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
    let file_name = "perlin_sphere.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
