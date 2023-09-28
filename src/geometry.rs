use crate::Intersection;
use crate::Ray;

pub trait Geometry: Sync {
    fn intersects_ray(&self, _ray: &Ray) -> Option<Intersection> {
        None
    }
}