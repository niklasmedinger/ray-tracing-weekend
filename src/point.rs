use std::{
    f32,
    ops::{Add, Sub},
};

use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(Vec3);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point(Vec3::new(x, y, z))
    }

    pub fn as_vec3(&self) -> Vec3 {
        self.0
    }
}

impl From<Vec3> for Point {
    fn from(value: Vec3) -> Self {
        Point(value)
    }
}

impl Add for Point {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Add<Vec3> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub for Point {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<Vec3> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0 - rhs)
    }
}
