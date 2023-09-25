use crate::Face;
use crate::Mesh;

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub mesh: &'a Mesh<'a>,
    pub face: &'a Face,
    pub t: f32,
    pub u: f32,
    pub v: f32,
}