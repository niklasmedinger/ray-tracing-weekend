//! This module defines the [Ray]s which are cast into the world and rendered
//! onto the image.

use crate::{point::Point, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
/// A ray from an origin with a specific direction.
pub struct Ray {
    origin: Point,
    direction: Vec3,
    time: f32,
}

impl Ray {
    /// Creates a new ray from an origin and a direction.
    pub fn new(origin: Point, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Get the origin of the point.
    pub fn origin(&self) -> &Point {
        &self.origin
    }

    /// Get the direction of the point.
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Get the time of the ray.
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Compute `ray.origin + t * ray.direction`. I.e., follow the direction of
    /// the ray from the origin scaled by `t`.
    pub fn at(&self, t: f32) -> Point {
        self.origin + (t * self.direction)
    }
}
