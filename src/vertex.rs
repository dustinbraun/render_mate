use crate::Vec2;
use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub texture_coords: Vec2,
    pub normal: Vec3,
}

impl Vertex {
    #[inline]
    pub fn new(position: Vec3, texture_coords: Vec2, normal: Vec3) -> Vertex {
        Vertex {
            position,
            texture_coords,
            normal,
        }
    }
}