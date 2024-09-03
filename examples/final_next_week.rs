use std::sync::Arc;

use ray_tracing_weekend::{
    bvh::BVHNode,
    camera::CameraBuilder,
    color::Color,
    constant_medium::ConstantMedium,
    hittable::{RotationY, Sphere, Translate, World},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    point::Point,
    quad::Quad,
    random_f32, random_vec3,
    texture::{ImageTexture, NoiseTexture, SolidColor},
    vec3::Vec3,
};

fn main() {
    let mut boxes1 = World::new();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = random_f32(1.0, 101.0);

            boxes1.push(Arc::new(Quad::quad_box(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                material_ground.clone(),
            )))
        }
    }

    let mut world = World::new();
    world.push(Arc::new(BVHNode::from_objects(boxes1.into_objects())));

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        7.0, 7.0, 7.0,
    )))));
    world.push(Arc::new(Quad::new(
        Point::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Point::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.push(Arc::new(Sphere::new_moving(
        center1,
        50.0,
        sphere_material,
        center2.into(),
    )));

    world.push(Arc::new(Sphere::new(
        Point::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Arc::new(Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.push(boundary.clone());
    world.push(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.push(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new(
        "./assets/earthmap.jpg",
    ))));
    world.push(Arc::new(Sphere::new(
        Point::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.push(Arc::new(Sphere::new(
        Point::new(220.0, 200.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2 = World::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Arc::new(Sphere::new(
            random_vec3(0.0, 165.0).into(),
            10.0,
            white.clone(),
        )))
    }

    world.push(Arc::new(Translate::new(
        Arc::new(RotationY::new(
            Arc::new(BVHNode::from_objects(boxes2.into_objects())),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    // Set up camera
    let camera = CameraBuilder::new()
        .image_width(1080)
        .samples_per_pixel(5000)
        .max_depth(50)
        .fov(40.0)
        .with_orientation(
            Point::new(478.0, 278.0, -600.0),
            Point::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
        .background(Color::black())
        .build();

    // Render
    let file_name = "final_next_week.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
