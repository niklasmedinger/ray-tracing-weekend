use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing_weekend::{
    bvh::BVHNode,
    camera::CameraBuilder,
    color::Color,
    hittable::{Hittable, Sphere, World},
    material::{Dielectric, Lambertian, Material, Metal},
    point::Point,
    random_0_1_f32, random_0_1_vec3, random_f32, random_vec3,
    vec3::Vec3,
};

pub fn many_scene(c: &mut Criterion) {
    // World
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    objects.push(Arc::new(ground_sphere));

    for a in -20..20 {
        for b in -11..11 {
            let choose_mat = random_0_1_f32();
            let center = Point::new(
                a as f32 + 0.9 * random_0_1_f32(),
                0.2,
                b as f32 + 0.9 * random_0_1_f32(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere;
                let material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // lambertian
                    let albedo: Color = random_0_1_vec3().into();
                    material = Arc::new(Lambertian::new(albedo));
                    sphere = Arc::new(Sphere::new(center, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3(0.0, 0.5).into();
                    let fuzz = random_f32(0.0, 0.5);
                    material = Arc::new(Metal::new(albedo, fuzz));
                    sphere = Arc::new(Sphere::new(center, 0.2, material));
                } else {
                    // glass
                    material = Arc::new(Dielectric::new(1.5));
                    sphere = Arc::new(Sphere::new(center, 0.2, material));
                }
                objects.push(sphere);
            }
        }
    }

    let camera = CameraBuilder::new()
        .image_width(100)
        .samples_per_pixel(5)
        .max_depth(5)
        .with_orientation(
            Point::new(13.0, 2.0, 3.0),
            Point::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
        .hide_progress(true)
        .build();

    let mut world = World::new();
    // let node = BVHNode::from_objects(objects);
    // world.push(Arc::new(node));
    for object in objects.into_iter() {
        world.push(object)
    }

    // Render
    let writer = std::io::sink();
    c.bench_function("criterion_many_scene", |b| {
        b.iter(|| camera.render(&world, writer).expect("Failed to render."))
    });
}

criterion_group!(benches, many_scene);
criterion_main!(benches);
