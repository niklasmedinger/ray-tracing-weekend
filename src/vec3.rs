use std::{
    f32,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::point::Point;

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

    pub fn near_zero(&self) -> bool {
        let s = 1.0 / 10.0_f32.powi(8);
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
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

impl From<Point> for Vec3 {
    fn from(value: Point) -> Self {
        value.as_vec3()
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

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
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
