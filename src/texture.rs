//! This module contains the code for textures.

use std::sync::Arc;
use std::{fmt::Debug, path::Path};

use image::{ImageBuffer, Rgb};

use crate::interval::Interval;
use crate::perlin::Perlin;
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

#[derive(Debug)]
/// A texture backed by an image.
pub struct ImageTexture {
    /// The underlying image.
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageTexture {
    /// Loads a new image from the given path. Panics if the loading fails.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            image: image::open(path)
                .expect("Failed to load image.")
                .into_rgb8(),
        }
    }

    /// Clamp x into [low, high].
    fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            low
        } else if x >= high {
            high - 1
        } else {
            x
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Point) -> Color {
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f32) as u32;
        let j = (v * self.image.height() as f32) as u32;
        let x = Self::clamp(i, 0, self.image.width());
        let y = Self::clamp(j, 0, self.image.height());
        let color = self.image.get_pixel(x, y);
        let color = Color::new(color[0] as f32, color[1] as f32, color[2] as f32);
        let color_scale = 1.0 / 255.0;
        color * color_scale
    }
}

#[derive(Debug, Copy, Clone)]
/// A noise texture backed by perlin noise.
pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    /// Create a new texture from perlin noise.
    pub fn new(noise: Perlin) -> Self {
        Self { noise }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Point) -> Color {
        Color::white() * self.noise.noise(p)
    }
}
