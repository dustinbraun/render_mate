use crate::Color;
use crate::Intersection;
use crate::Node;
use crate::Ray;
use crate::Vec3;

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Color,
    pub emission: f32,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, color: Color, emission: f32) -> Sphere {
        Sphere {
            position,
            radius,
            color,
            emission,
        }
    }
}

impl Node for Sphere {
    fn intersects_ray(&self, _ray: &Ray) -> Option<Intersection> {
        unimplemented!()
    }
}