use std::rc::Rc;

use color_eyre::eyre::Context;
use ray_tracing_weekend::{
    camera::CameraBuilder,
    color::Color,
    hittable::{Sphere, World},
    material::{Dielectric, Lambertian, Material, Metal},
    point::Point,
    random_0_1_f32, random_0_1_vec3, random_f32, random_vec3,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // World
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let ground_sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    let mut world = World::new();
    world.push(Box::new(ground_sphere));

    for a in 0..11 {
        for b in 0..11 {
            let choose_mat = random_0_1_f32();
            let center = Point::new(
                a as f32 + 0.9 * random_0_1_f32(),
                0.2,
                b as f32 + 0.9 * random_0_1_f32(),
            );

            let sphere;
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    // lambertian
                    let albedo: Color = random_0_1_vec3().into();
                    material = Rc::new(Lambertian::new(albedo));
                    sphere = Box::new(Sphere::new(center, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    // sphere_material = make_shared<metal>(albedo, fuzz);
                    // world.add(make_shared<sphere>(center, 0.2, sphere_material));
                    let albedo = random_vec3(0.0, 0.5).into();
                    let fuzz = random_f32(0.0, 0.5);
                    material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                }
            }

            world.push(sphere);
        }
    }

    // Set up camera
    let camera = CameraBuilder::default().build();

    // Render
    camera.render(&world).wrap_err("Failed to render image.")?;

    Ok(())
}
