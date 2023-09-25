use crate::Color;
use crate::Scene;
use crate::Intersection;
use crate::Ray;

pub trait Material: Sync {
    fn trace(&self, _scene: &Scene, _ray: &Ray, _intersection: &Intersection, _depth: u32) -> Color {
        Color::new(
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}