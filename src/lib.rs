use std::f32;

use rand::{thread_rng, Rng};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
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
