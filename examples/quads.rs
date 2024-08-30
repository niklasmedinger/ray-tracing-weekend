use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder, color::Color, hittable::World, material::Lambertian, point::Point,
    quad::Quad, vec3::Vec3,
};

fn main() {
    // Default camera
    let look_from = Point::new(0.0, 0.0, 9.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .fov(80.0)
        .image_width(800)
        .samples_per_pixel(100)
        .max_depth(50)
        .aspect_ratio(1.0)
        .build();

    // Materials
    let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    // World
    let mut world = World::new();
    world.push(Arc::new(Quad::new(
        Point::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(left_red),
    )));

    world.push(Arc::new(Quad::new(
        Point::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(back_green),
    )));

    world.push(Arc::new(Quad::new(
        Point::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(right_blue),
    )));

    world.push(Arc::new(Quad::new(
        Point::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(upper_orange),
    )));

    world.push(Arc::new(Quad::new(
        Point::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(lower_teal),
    )));

    // Render
    let file_name = "quads.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
