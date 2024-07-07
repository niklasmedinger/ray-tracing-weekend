use rand::{thread_rng, Rng};

use crate::{INFINITY, NEG_INFINITY};

pub const COLOR_INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

pub fn random_interval_f32(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min..max)
}

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, v: f32) -> bool {
        self.min <= v && v <= self.max
    }

    pub fn surrounds(&self, v: f32) -> bool {
        self.min < v && v < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub const fn empty() -> Self {
        Self {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }

    pub const fn universe() -> Self {
        Self {
            min: NEG_INFINITY,
            max: INFINITY,
        }
    }
}
