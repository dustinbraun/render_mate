use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3, t_min: f32, t_max: f32) -> Ray {
        Ray {
            origin,
            direction,
            t_min,
            t_max,
        }
    }
}