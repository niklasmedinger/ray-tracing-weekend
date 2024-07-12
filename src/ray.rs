use crate::{point::Point, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
/// A ray from an origin with a specific direction.
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    /// Create a new ray from an origin and a direction.
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get the origin of the point.
    pub fn origin(&self) -> &Point {
        &self.origin
    }

    /// Get the direction of the point.
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Compute `ray.origin + t * ray.direction`. I.e., follow the direction of
    /// the ray from the origin scaled by `t`.
    pub fn at(&self, t: f32) -> Point {
        self.origin + (t * self.direction)
    }
}
