//! This module contains an implementation of RGB color, which is a thin wrapper
//! around [Vec3].

use std::{
    f32,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use crate::{interval::COLOR_INTENSITY, vec3::Vec3};

#[derive(Copy, Clone, Debug)]
/// A struct that represents RGB colors.
pub struct Color(Vec3);

impl Color {
    /// Create a new color from `r`, `g`, and `b` values in [0, 1].
    /// The final value is then, for instance, computed as `r * 255`.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color(Vec3::new(r, g, b))
    }

    /// The black color.
    pub fn black() -> Self {
        Color(Vec3::new(0.0, 0.0, 0.0))
    }

    /// The white color.
    pub fn white() -> Self {
        Color(Vec3::new(1.0, 1.0, 1.0))
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(rhs.0 * self)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color(value)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Linear to gamma space
        fn linear_to_gamma(linear_component: f32) -> f32 {
            if linear_component > 0.0 {
                linear_component.sqrt()
            } else {
                0.0
            }
        }

        let r = linear_to_gamma(self.0.x());
        let g = linear_to_gamma(self.0.y());
        let b = linear_to_gamma(self.0.z());
        let rbyte: u32 = (256.0 * COLOR_INTENSITY.clamp(r)) as u32;
        let gbyte: u32 = (256.0 * COLOR_INTENSITY.clamp(g)) as u32;
        let bbyte: u32 = (256.0 * COLOR_INTENSITY.clamp(b)) as u32;
        write!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}
