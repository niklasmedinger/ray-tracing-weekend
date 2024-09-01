use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{RotationY, Translate, World},
    material::{DiffuseLight, Lambertian},
    point::Point,
    quad::Quad,
    texture::SolidColor,
    vec3::Vec3,
};

fn main() {
    // Default camera
    let look_from = Point::new(278.0, 278.0, -800.0);
    let look_at = Point::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .fov(40.0)
        .image_width(1080)
        .samples_per_pixel(400)
        .max_depth(50)
        .aspect_ratio(1.0)
        .background(Color::black())
        .build();

    // Materials
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        16.0, 16.0, 16.0,
    )))));

    // World
    let mut world = World::new();
    world.push(Arc::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.push(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.push(Arc::new(Quad::new(
        Point::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    world.push(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.push(Arc::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.push(Arc::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Arc::new(Quad::quad_box(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotationY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.push(box1);

    let box2 = Arc::new(Quad::quad_box(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(RotationY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.push(box2);

    // Render
    let file_name = "cornell_box.png";
    let image = camera.render(&world);
    // eprintln!("Color: {:?}", image);
    image.save(file_name).expect("Failed to save file.");
}
