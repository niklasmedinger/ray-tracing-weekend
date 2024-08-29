//! This module contains the axis aligned bounding box code.

use std::{cmp::Ordering, ops::Index};

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
        let mut res = Self { x, y, z };
        res.pad_to_minimums();
        res
    }

    /// Create an empty bounding box.
    pub const fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    /// Create a bounding box that spans the whole scene.
    pub const fn universe() -> Self {
        Self {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
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
        let mut res = Self { x, y, z };
        res.pad_to_minimums();
        res
    }

    /// Create a new bounding box that contains both `box1` and `box2`.
    pub fn from_aabbs(box1: &AABB, box2: &AABB) -> Self {
        let x = Interval::enclosing(&box1.x, &box2.x);
        let y = Interval::enclosing(&box1.y, &box2.y);
        let z = Interval::enclosing(&box1.z, &box2.z);
        Self { x, y, z }
    }

    /// Get the x interval.
    pub fn x(&self) -> &Interval {
        &self.x
    }

    /// Get the y interval.
    pub fn y(&self) -> &Interval {
        &self.y
    }

    /// Get the z interval.
    pub fn z(&self) -> &Interval {
        &self.z
    }

    /// Returns the dimension with the longest axis.
    pub fn longest_axis(&self) -> Dimension {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Dimension::X
            } else {
                Dimension::Z
            }
        } else if self.y.size() > self.z.size() {
            Dimension::Y
        } else {
            Dimension::Z
        }
    }

    /// Determine whether `ray` hits the bounding box in interval `ray_t`.
    /// If so, returns a new interval where the ray and the box intersect.
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Interval> {
        let mut res = ray_t;
        let ray_origin = ray.origin();
        let ray_dir = ray.direction();

        for dimension in Dimension::iter() {
            let ax = self[dimension];
            let adinv = 1.0 / ray_dir[dimension];

            let t0 = (ax.min() - ray_origin[dimension]) * adinv;
            let t1 = (ax.max() - ray_origin[dimension]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min() {
                    res.set_min(t0);
                }
                if t1 < ray_t.max() {
                    res.set_max(t1);
                }
            } else {
                if t1 > ray_t.min() {
                    res.set_min(t1);
                }
                if t0 < ray_t.max() {
                    res.set_max(t0);
                }
            }

            if res.max() <= res.min() {
                return None;
            }
        }

        Some(res)
    }

    /// Returns `Ordering::Less` iff the minimum value of self's interval in
    /// the dimension is smaller than other's interval in the dimension.
    pub fn box_compare(&self, other: &AABB, dimension: Dimension) -> Ordering {
        let self_axis_interval = self[dimension];
        let other_axis_interval = other[dimension];
        self_axis_interval
            .min()
            .total_cmp(&other_axis_interval.min())
    }

    /// Compare self and other in the x-dimension.
    pub fn box_x_compare(&self, other: &AABB) -> Ordering {
        self.box_compare(other, Dimension::X)
    }

    /// Compare self and other in the y-dimension.
    pub fn box_y_compare(&self, other: &AABB) -> Ordering {
        self.box_compare(other, Dimension::Y)
    }

    /// Compare self and other in the z-dimension.
    pub fn box_z_compare(&self, other: &AABB) -> Ordering {
        self.box_compare(other, Dimension::Z)
    }

    /// Pad the bounding box such that no side is narrower than some delta.
    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
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
