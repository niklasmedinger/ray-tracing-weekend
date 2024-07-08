use std::f32;

use rand::{thread_rng, Rng};
use vec3::Vec3;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod point;
pub mod ray;
pub mod vec3;

pub const INFINITY: f32 = std::f32::INFINITY;
pub const NEG_INFINITY: f32 = std::f32::NEG_INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_0_1_f32() -> f32 {
    thread_rng().gen_range(0.0..1.0)
}

pub fn random_f32(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min..max)
}

pub fn random_0_1_vec3() -> Vec3 {
    Vec3::new(random_0_1_f32(), random_0_1_f32(), random_0_1_f32())
}

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_f32(min, max),
        random_f32(min, max),
        random_f32(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
