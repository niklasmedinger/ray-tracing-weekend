//! This module contains the axis aligned bounding box code.

use std::ops::Index;

use strum::IntoEnumIterator;

use crate::{interval::Interval, point::Point, ray::Ray, vec3::Dimension};

#[derive(Copy, Clone, Debug)]
/// An axis aligned bounding box.
pub struct AABB {
    /// The x interval this box covers.
    x: Interval,
    /// The y interval this box covers.
    y: Interval,
    /// The z interval this box covers.
    z: Interval,
}

impl Default for AABB {
    /// The default bounding box is empty.
    fn default() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }
}

impl AABB {
    /// Create a new bounding box from the given intervals.
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// Create a new bounding box from two points.
    pub fn from_points(a: Point, b: Point) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        Self { x, y, z }
    }

    /// Create a new bounding box that contains both `box1` and `box2`.
    pub fn from_aabbs(box1: AABB, box2: AABB) -> Self {
        let x = Interval::enclosing(&box1.x, &box2.x);
        let y = Interval::enclosing(&box1.y, &box2.y);
        let z = Interval::enclosing(&box1.z, &box2.z);
        Self { x, y, z }
    }

    /// Get the x interval
    pub fn x(&self) -> &Interval {
        &self.x
    }

    /// Get the y interval
    pub fn y(&self) -> &Interval {
        &self.y
    }

    /// Get the z interval
    pub fn z(&self) -> &Interval {
        &self.z
    }

    /// Determine whether `ray` hits
    pub fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_dir = ray.direction();

        for dimension in Dimension::iter() {
            let ax = self[dimension];
            let adinv = 1.0 / ray_dir[dimension];

            let t0 = (ax.min() - ray_origin[dimension]) * adinv;
            let t1 = (ax.max() - ray_origin[dimension]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min() {
                    ray_t.set_min(t0);
                }
                if t1 < ray_t.max() {
                    ray_t.set_max(t1);
                }
            } else {
                if t1 > ray_t.min() {
                    ray_t.set_min(t1);
                }
                if t0 < ray_t.max() {
                    ray_t.set_max(t0);
                }
            }

            if ray_t.max() <= ray_t.min() {
                return false;
            }
        }

        true
    }
}

impl Index<Dimension> for AABB {
    type Output = Interval;

    fn index(&self, index: Dimension) -> &Self::Output {
        match index {
            Dimension::X => &self.x,
            Dimension::Y => &self.y,
            Dimension::Z => &self.z,
        }
    }
}
