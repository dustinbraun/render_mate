use crate::Face;
use crate::Mesh;
use crate::Sphere;
use crate::AccelerationStructure;

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub payload: IntersectionPayload<'a>,
}

#[derive(Copy, Clone)]
pub enum IntersectionPayload<'a> {
    Mesh {
        mesh: &'a Mesh,
        face: &'a Face,
        u: f32,
        v: f32,
    },
    Sphere {
        sphere: &'a Sphere,
    },
    AccelerationStructure {
        acceleration_structure: &'a AccelerationStructure,
        face: &'a Face,
        u: f32,
        v: f32,
    },
}