//! This module contains the code for textures.

use std::fmt::Debug;
use std::sync::Arc;

use crate::{color::Color, point::Point};

/// Defines the methods a texture object needs to implement.
pub trait Texture: Debug + Sync + Send {
    /// Computes the color of this texture at coordinates `u` and `v`.
    /// We also pass the [Point] that corresponds to these coordinates.
    fn value(&self, u: f32, v: f32, p: Point) -> Color;
}

#[derive(Copy, Debug, Clone)]
/// A texture that returns a color
pub struct SolidColor {
    /// The solid color of this texture
    albedo: Color,
}

impl SolidColor {
    /// Creates a new solid color texture given a color.
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }

    /// Creates a new solid color from rgb values.
    pub fn from_rbg(r: f32, g: f32, b: f32) -> Self {
        SolidColor {
            albedo: Color::new(r, g, b),
        }
    }

    /// Returns the color of this texture.
    pub fn color(&self) -> Color {
        self.albedo
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Point) -> Color {
        self.albedo
    }
}

#[derive(Clone, Debug)]
/// A checkered texture.
pub struct CheckeredTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckeredTexture {
    /// Creates a new checkered texture given textures for the even and odd tiles.
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    /// Creates a new checkered texture given colors for the even and odd tiles.
    pub fn from_color(scale: f32, even: Color, odd: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, u: f32, v: f32, p: Point) -> Color {
        let x_int = f32::floor(self.inv_scale * p.x()) as i32;
        let y_int = f32::floor(self.inv_scale * p.y()) as i32;
        let z_int = f32::floor(self.inv_scale * p.z()) as i32;
        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
