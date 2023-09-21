use crate::BoundingVolume;
use crate::Ray;
use crate::Vec3;

#[derive(Copy, Clone)]
pub struct BoundingSphere {
    pub position: Vec3,
    pub radius: f32,
}

impl BoundingSphere {
    #[inline]
    pub fn new(position: Vec3, radius: f32) -> BoundingSphere {
        BoundingSphere {
            position,
            radius,
        }
    }
}

impl BoundingVolume for BoundingSphere {
    #[inline]
    fn center(&self) -> Vec3 {
        self.position
    }

    fn intersects_ray(&self, _ray: &Ray) -> bool {
        unimplemented!()
    }

    #[inline]
    fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }
}
