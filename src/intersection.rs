use crate::Face;
use crate::Mesh;
use crate::Sphere;

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub payload: IntersectionPayload<'a>,
}

#[derive(Copy, Clone)]
pub enum IntersectionPayload<'a> {
    MeshIntersectionPayload {
        mesh: &'a Mesh<'a>,
        face: &'a Face,
        u: f32,
        v: f32,
    },
    SphereIntersectionPayload {
        sphere: &'a Sphere,
    },
}