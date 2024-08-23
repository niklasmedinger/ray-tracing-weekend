use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_weekend::{
    bvh::BVHNode,
    camera::CameraBuilder,
    color::Color,
    hittable::{Hittable, Sphere, World},
    material::Lambertian,
    point::Point,
    vec3::Vec3,
    PI,
};

pub fn grid(c: &mut Criterion) {
    // Camera
    let look_from = Point::new(0.0, 5.0, 0.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 0.0, -1.0);
    let camera = CameraBuilder::default()
        .with_orientation(look_from, look_at, vup)
        .image_width(200)
        .samples_per_pixel(10)
        .max_depth(10)
        .hide_progress(true)
        .build();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let yellow_lambertian = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));

    // Objects in the world
    let ground_sphere = Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );

    // World
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    objects.push(Arc::new(ground_sphere));

    // Create a grid of spheres
    let r = f32::cos(PI / 6.0);
    let columns = 5;
    let rows = 4;
    let x0 = -r * columns as f32 / 2.0;
    let z0 = -r * rows as f32 / 2.0;
    for i in 0..columns {
        for j in 0..rows {
            let sphere = Sphere::new(
                Point::new(x0 + i as f32 * r * 2.0, 0.0, z0 + j as f32 * r * 2.0),
                r,
                yellow_lambertian.clone(),
            );
            objects.push(Arc::new(sphere));
        }
    }

    let node = BVHNode::from_objects(objects);
    let mut world = World::new();
    world.push(Arc::new(node));

    // Render
    c.bench_function("criterion_grid_pixel", |b| {
        b.iter(|| camera.render_pixel(&world, 100, 100))
    });
}

criterion_group!(benches, grid);
criterion_main!(benches);
