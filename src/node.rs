use crate::Geometry;
use crate::Material;

#[derive(Copy, Clone)]
pub struct Node<'a> {
    pub geometry: &'a dyn Geometry,
    pub material: &'a dyn Material,
}