use std::{
    f32,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
    },
};

use rand::{thread_rng, Rng};

// pub mod ppm;
pub mod camera;
pub mod hittable;
pub mod ray;

pub const INFINITY: f32 = std::f32::INFINITY;
pub const NEG_INFINITY: f32 = std::f32::NEG_INFINITY;
pub const PI: f32 = 3.1415926535897932385;
pub const COLOR_INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_0_1_f32() -> f32 {
    thread_rng().gen_range(0.0..1.0)
}

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(Vec3);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point(Vec3::new(x, y, z))
    }
}

impl Deref for Point {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for Point {
    fn from(value: Vec3) -> Self {
        Point(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color(Vec3(r, g, b))
    }
    pub fn black() -> Self {
        Color(Vec3(0.0, 0.0, 0.0))
    }

    pub fn white() -> Self {
        Color(Vec3(1.0, 1.0, 1.0))
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color(value)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.0.x();
        let g = self.0.y();
        let b = self.0.z();
        let rbyte: u32 = (256.0 * COLOR_INTENSITY.clamp(r)) as u32;
        let gbyte: u32 = (256.0 * COLOR_INTENSITY.clamp(g)) as u32;
        let bbyte: u32 = (256.0 * COLOR_INTENSITY.clamp(b)) as u32;
        write!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index {} out of bounds", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index {} out of bounds", index),
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn negate() {
        let t = Vec3::new(1.0, 1.0, 1.0);
        let negated = -t;
        let expected = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(expected, negated);
        assert_eq!(t, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn div() {
        let t = Vec3::new(2.0, 2.0, 2.0);
        let division = t / 2.0;
        let expected = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(expected, division);
        assert_eq!(t, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn mul() {
        let t = Vec3::new(2.0, 2.0, 2.0);
        let mul = t * 2.0;
        let expected = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(expected, mul);
        assert_eq!(t, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn sub() {
        let t = Vec3::new(2.0, 2.0, 2.0);
        let sub = t - Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(expected, sub);
        assert_eq!(t, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn add() {
        let t = Vec3::new(2.0, 2.0, 2.0);
        let add = t + Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(expected, add);
        assert_eq!(t, Vec3::new(2.0, 2.0, 2.0));
    }
}
