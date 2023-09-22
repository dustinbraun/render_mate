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
    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction.sq_len();
        let b = ray.origin - self.position;
        let c = 2.0*ray.direction.dot(b);
        let d = b.sq_len() - self.radius*self.radius;
        if c*c - 4.0*a*d < 0.0 {
            return None
        }
        let t = (-c - (c*c - 4.0*a*d).sqrt())/(2.0*a);
        if t < ray.t_min || t > ray.t_max {
            return None
        }
        let position = ray.origin + ray.direction*t;
        let mut normal = (position - self.position).normalize();
        if normal.dot(ray.direction) > 0.0 {
            normal *= -1.0;
        }
        Some(Intersection {
            position,
            normal,
            color: self.color,
            t,
            emission: self.emission,
        })
    }
}