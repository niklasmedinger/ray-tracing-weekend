//! A Rust implementation of ``Ray Tracing in One Weekend''.

#![forbid(unused_must_use)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod point;
pub mod quad;
pub mod ray;
pub mod texture;
pub mod vec3;

use std::f32;

use rand::{thread_rng, Rng};
use vec3::{Unit3, Vec3};

/// Positive infinity for f32.
pub const INFINITY: f32 = f32::INFINITY;
/// Negative infinity for f32.
pub const NEG_INFINITY: f32 = f32::NEG_INFINITY;
/// Pi
pub const PI: f32 = f32::consts::PI;

#[inline]
/// Converts `degrees` to radians.
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
/// Generates a random f32 in `[0.0, 1.0]`.
pub fn random_0_1_f32() -> f32 {
    thread_rng().gen_range(0.0..1.0)
}

#[inline]
/// Generates a random f32 in `[min, max]`.
pub fn random_f32(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min..max)
}

#[inline]
/// Generates a random vector where each component is in `[0, 0]`.
pub fn random_0_1_vec3() -> Vec3 {
    Vec3::new(random_0_1_f32(), random_0_1_f32(), random_0_1_f32())
}

#[inline]
/// Generates a random vector where each component is in `[min, max]`.
pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_f32(min, max),
        random_f32(min, max),
        random_f32(min, max),
    )
}

/// Generates a random vector in the sphere with radius 1.0.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
/// Generates a random unit vector.
pub fn random_unit_vector() -> Unit3 {
    random_in_unit_sphere().unit()
}

/// Generates a random unit vector in the disk with radius 1.0. Note that the
/// disk lies in the x and y plane.
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
