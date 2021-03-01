#![allow(dead_code)]

use super::{Vec3f, Position};

/// A ray is defined as the function
/// `p(t) = A + tB`, where `A` is the origin of the ray
/// and `B` it's direction.
pub struct Ray {
    pub a: Vec3f<Position>,
    pub b: Vec3f<Position>,
}

impl Ray {
    /// Origin of the ray
    pub fn origin(&self) -> Vec3f<Position> {
        self.a
    }
    /// Direction of the ray
    pub fn direction(&self) -> Vec3f<Position> {
        self.b
    }
    /// Ray as the function `p(t) = A + tB`
    pub fn point_at_parameter(&self, t: f32) -> Vec3f<Position> {
        self.a + t * self.b
    }
}