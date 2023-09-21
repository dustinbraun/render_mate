use crate::Intersection;
use crate::Ray;

pub trait Node: Sync {
    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection>;
}