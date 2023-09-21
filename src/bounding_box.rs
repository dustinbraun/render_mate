use crate::BoundingVolume;
use crate::Ray;
use crate::Vec3;

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    #[inline]
    pub fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox {
            min,
            max,
        }
    }
}

impl BoundingVolume for BoundingBox {
    #[inline]
    fn center(&self) -> Vec3 {
        (self.min + self.max)*0.5
    }

    fn intersects_ray(&self, ray: &Ray) -> bool {
        let dir_frac = Vec3::new(
            1.0/ray.direction.x,
            1.0/ray.direction.y,
            1.0/ray.direction.z,
        );
        let t1 = (self.min.x - ray.origin.x)*dir_frac.x;
        let t2 = (self.max.x - ray.origin.x)*dir_frac.x;
        let t3 = (self.min.y - ray.origin.y)*dir_frac.y;
        let t4 = (self.max.y - ray.origin.y)*dir_frac.y;
        let t5 = (self.min.z - ray.origin.z)*dir_frac.z;
        let t6 = (self.max.z - ray.origin.z)*dir_frac.z;
        let tmin = max(max(min(t1, t2), min(t3, t4)), min(t5, t6));
        let tmax = min(min(max(t1, t2), max(t3, t4)), max(t5, t6));
        if tmax < 0.0 || tmin > tmax {
            false
        }
        else {
            true
        }
    }

    #[inline]
    fn translate(&mut self, translation: Vec3) {
        self.min += translation;
        self.max += translation;
    }
}

#[inline]
fn min(a: f32, b: f32) -> f32 {
    if a <= b {
        a
    }
    else {
        b
    }
}

#[inline]
fn max(a: f32, b: f32) -> f32 {
    if a >= b {
        a
    }
    else {
        b
    }
}