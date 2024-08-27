use std::sync::Arc;

use ray_tracing_weekend::{
    bvh::BVHNode,
    camera::CameraBuilder,
    color::Color,
    hittable::{Hittable, Sphere, World},
    material::{Dielectric, Lambertian, Material, Metal},
    point::Point,
    random_0_1_f32, random_0_1_vec3, random_f32, random_vec3,
    texture::CheckeredTexture,
    vec3::Vec3,
};

fn main() {
    // World
    let material_ground =
        CheckeredTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_sphere = Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(Arc::new(material_ground))),
    );
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    objects.push(Arc::new(ground_sphere));

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

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere1 = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1);
    let sphere2 = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2);
    let sphere3 = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3);
    objects.push(Arc::new(sphere1));
    objects.push(Arc::new(sphere2));
    objects.push(Arc::new(sphere3));

    // Set up camera
    let camera = CameraBuilder::new()
        .image_width(1200)
        .samples_per_pixel(400)
        .max_depth(50)
        .fov(20.0)
        .with_orientation(
            Point::new(13.0, 2.0, 3.0),
            Point::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
        .with_defocus(0.6, 10.0)
        .build();

    let node = BVHNode::from_objects(objects);
    let mut world = World::new();
    world.push(Arc::new(node));

    // Render
    let file_name = "final_checkered.png";
    let image = camera.render(&world);
    image.save(file_name).expect("Failed to save file.");
}
