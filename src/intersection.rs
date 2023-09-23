use crate::Color;
use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Color,
    pub t: f32,
    pub emission: f32,
    pub scatter: f32,
}