//! A module that implements intervals of f32.

use crate::{INFINITY, NEG_INFINITY};

/// The interval of valid color intensities to be used inside of [Color].
pub const COLOR_INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

#[derive(Debug, Copy, Clone)]
/// A interval of real-numbers `[min, max]` represented by [f32].
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    /// Create a new interval `[min, max]`.
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    /// Create a new interval which encloses both intervals `a` and `b`.
    pub fn enclosing(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval { min, max }
    }

    /// Get `min` of the interval `[min, max]`.
    pub fn min(&self) -> f32 {
        self.min
    }

    /// Get `max` of the interval `[min, max]`.
    pub fn max(&self) -> f32 {
        self.max
    }

    /// Override the minimum of this interval.
    pub fn set_min(&mut self, v: f32) {
        self.min = v;
    }

    /// Override the maximum of this interval.
    pub fn set_max(&mut self, v: f32) {
        self.max = v;
    }

    /// Get the size of `[min, max]`, i.e., `max - min`.
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    /// Check whether `v` is contained inside of `[min, max]`. I.e., `min <= v <= max`.
    pub fn contains(&self, v: f32) -> bool {
        self.min <= v && v <= self.max
    }

    /// Check whether `v` is surrounded by `[min, max]`. I.e., `min < v < max`.
    pub fn surrounds(&self, v: f32) -> bool {
        self.min < v && v < self.max
    }

    /// Returns `x` if it is contained by `[min, max]`. If `x` is not contained,
    /// and returns `min` if `x` is smaller than `min`; otherwise `max`.
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// Create a new interval that is the same as `self` but padded by
    /// `delta / 2.0` on both sides.
    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// The empty interval.
    pub const fn empty() -> Self {
        Self {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }

    /// The interval from negative infinity to positive infinity.
    pub const fn universe() -> Self {
        Self {
            min: NEG_INFINITY,
            max: INFINITY,
        }
    }

    /// The unit interval from `0.0` to `1.0`.
    pub const fn unit() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}
