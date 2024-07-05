use ray_tracing_weekend::{
    camera::Camera,
    hittable::{Sphere, World},
    Point,
};

fn main() -> color_eyre::Result<()> {
    // TODO: Continue with Chapter 7.0
    // TODO: A serialization format for scenes. Otherwise, we have to manually
    // change `main` all the time.
    // Note on a type for normal length vectors: Create a type for normal length
    // vectors? I.e., NormalVec3(Vec3)
    // Problem: Calculating a normal can be done more efficient than dividing
    // a vector by its length, which is costly to compute. E.g., for a sphere
    // we can compute its surface normal by diving the vector from its center
    // to the ray's intersection by the sphere's radius instead of computing
    // the ray's length by squaring.
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 1240;

    // Camera

    let camera = Camera::new(aspect_ratio, image_width);

    // Objects in the world
    let sphere1 = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);

    // World
    let mut world = World::new();
    world.push(&sphere1);
    world.push(&sphere2);

    camera.render(&world)?;

    // TODO: Impl math ops for Point such that they return Point? Currently, we have
    // to 'downcast' Points, Colors, etc to Triple and then manually 'remember' what
    // the result type is.

    Ok(())
}
