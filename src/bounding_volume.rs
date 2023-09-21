use crate::Ray;
use crate::Vec3;

pub trait BoundingVolume {
    fn center(&self) -> Vec3;

    fn intersects_ray(&self, ray: &Ray) -> bool;

    fn translate(&mut self, translation: Vec3);
}