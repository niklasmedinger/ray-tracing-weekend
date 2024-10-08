use std::sync::Arc;

use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::{Dielectric, Lambertian, Material, Metal},
    point::Point,
    random_0_1_f32, random_0_1_vec3, random_f32, random_vec3,
    vec3::Vec3,
};

fn main() {
    // World
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    let mut world = World::new();
    world.push(Arc::new(ground_sphere));

    for a in -11..11 {
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
                    let moves_to = center + Vec3::new(0.0, random_f32(0.0, 0.5), 0.0);
                    material = Arc::new(Lambertian::new(albedo));
                    sphere = Arc::new(Sphere::new_moving(center, 0.2, material, moves_to));
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
                world.push(sphere);
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere1 = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1);
    let sphere2 = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2);
    let sphere3 = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3);
    world.push(Arc::new(sphere1));
    world.push(Arc::new(sphere2));
    world.push(Arc::new(sphere3));

    // Set up camera
    let camera = CameraBuilder::new()
        .image_width(800)
        .samples_per_pixel(50)
        .max_depth(50)
        .fov(20.0)
        .with_orientation(
            Point::new(13.0, 2.0, 3.0),
            Point::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
        .with_defocus(0.6, 10.0)
        .build();

    // Render
    let file_name = "motionblur.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
