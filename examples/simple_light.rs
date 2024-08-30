use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::{DiffuseLight, Lambertian},
    point::Point,
    quad::Quad,
    texture::{NoiseTexture, SolidColor},
    vec3::Vec3,
};

fn main() {
    // Default camera
    let look_from = Point::new(26.0, 3.0, 6.0);
    let look_at = Point::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .fov(20.0)
        .image_width(1080)
        .samples_per_pixel(200)
        .max_depth(50)
        .background(Color::black())
        .build();

    // Materials
    let noise_texture = Arc::new(NoiseTexture::new(4.0));
    let diff_light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        4.0, 4.0, 4.0,
    )))));

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
    world.push(Arc::new(Quad::new(
        Point::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light,
    )));

    // Render
    let file_name = "simple_light.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
