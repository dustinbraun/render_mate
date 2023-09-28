use crate::Intersection;
use crate::Ray;
use crate::Vec3;
use crate::IntersectionPayload;
use crate::Geometry;

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32) -> Sphere {
        Sphere {
            position,
            radius,
        }
    }
}

impl Geometry for Sphere {
    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.origin - self.position;
        let b = 2.0*ray.direction.dot(a);
        let c = a.sq_len() - self.radius*self.radius;
        if b*b - 4.0*c < 0.0 {
            return None
        }
        let t = (-b - (b*b - 4.0*c).sqrt())/2.0;
        if t < ray.t_min || t > ray.t_max {
            return None
        }
        let position = ray.origin + ray.direction*t;
        let mut normal = (position - self.position).normalize();
        if normal.dot(ray.direction) > 0.0 {
            normal *= -1.0;
        }
        Some(Intersection {
            t,
            payload: IntersectionPayload::SphereIntersectionPayload {
                sphere: self,
            },
        })
    }
}