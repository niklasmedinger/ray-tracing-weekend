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

    /// Get `min` of the interval `[min, max]`.
    pub fn min(&self) -> f32 {
        self.min
    }

    /// Get `max` of the interval `[min, max]`.
    pub fn max(&self) -> f32 {
        self.max
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
}
