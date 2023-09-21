use crate::Color;
use crate::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Vec3, color: Color) -> PointLight {
        PointLight {
            position,
            color,
        }
    }
}