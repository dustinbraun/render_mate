use crate::Geometry;
use crate::Material;

#[derive(Copy, Clone)]
pub struct Node<'a> {
    geometry: &'a dyn Geometry,
    material: &'a dyn Material,
}

impl<'a> Node<'a> {
    #[inline(always)]
    pub fn new(geometry: &'a dyn Geometry, material: &'a dyn Material) -> Node<'a> {
        Node {
            geometry,
            material,
        }
    }

    #[inline(always)]
    pub fn geometry(&'a self) -> &'a dyn Geometry {
        self.geometry
    }

    #[inline(always)]
    pub fn material(&'a self) -> &'a dyn Material {
        self.material
    }
}