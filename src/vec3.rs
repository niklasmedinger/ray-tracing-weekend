//! This module implements a three dimensional vector [Vec3] and many linear
//! algebra operations on it.

use std::{
    f32,
    ops::{
        Add, AddAssign, Deref, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
    },
};

use strum_macros::EnumIter;

use crate::point::Point;

#[derive(Copy, Clone, Debug, EnumIter)]
/// The three dimensions of 3D space.
pub enum Dimension {
    /// The `X` dimension.
    X,
    /// The `Y` dimension.
    Y,
    /// The `Z` dimension.
    Z,
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
/// A three dimensional vector in space.
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    #[inline]
    /// Create a new vector from its x, y, and z component.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    #[inline]
    /// Get the x component.
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    /// Get the y component.
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    /// Get the z component.
    pub fn z(&self) -> f32 {
        self.2
    }

    #[inline]
    /// Compute the length of the vector. I.e., `sqrt(x^2 + y^2 + z^2)`.
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    #[inline]
    /// Compute the square of the length of the vector. I.e., `x^2 + y^2 + z^2`.
    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    /// Returns true iff the vector is _near zero_. That is, each component
    /// of the vector is smaller than `10e-8`.
    pub fn near_zero(&self) -> bool {
        let s = 1.0 / 10.0_f32.powi(8);
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    #[inline]
    /// Compute the dot product of `self` with `other`.
    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    /// Compute the cross product of `self` and `other`.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    /// Return the unit vector that points in the same direction as `self`.
    pub fn unit(&self) -> Unit3 {
        Unit3::new_normalize(*self)
    }
}

impl From<Point> for Vec3 {
    fn from(value: Point) -> Self {
        *value
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
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

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl DivAssign for Vec3 {
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

impl Index<Dimension> for Vec3 {
    type Output = f32;

    fn index(&self, index: Dimension) -> &Self::Output {
        match index {
            Dimension::X => &self.0,
            Dimension::Y => &self.1,
            Dimension::Z => &self.2,
        }
    }
}

impl IndexMut<Dimension> for Vec3 {
    fn index_mut(&mut self, index: Dimension) -> &mut Self::Output {
        match index {
            Dimension::X => &mut self.0,
            Dimension::Y => &mut self.1,
            Dimension::Z => &mut self.2,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

/// A three dimensional vector with unit length.
#[derive(Copy, Debug, Clone)]
pub struct Unit3(Vec3);

impl Deref for Unit3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Unit3 {
    #[inline]
    /// Creates a new normal vector pointing in the same direction as `v`.
    pub fn new_normalize(v: Vec3) -> Self {
        Unit3(v / v.length())
    }

    #[inline]
    /// Creates a new normal vector pointing in the same direction as `v`.
    /// Does not normalize `v` but assumes that it is already a unit vector.
    pub fn new_unchecked(v: Vec3) -> Self {
        Unit3(v)
    }

    #[inline]
    /// Consumes the unit vector and retrieves the underlying [Vec3].
    pub fn into_inner(self) -> Vec3 {
        self.0
    }
}

impl Neg for Unit3 {
    type Output = Unit3;

    fn neg(self) -> Self::Output {
        Unit3(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

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
