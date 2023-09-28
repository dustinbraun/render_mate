use crate::Material;
use crate::Color;
use crate::Scene;
use crate::Ray;
use crate::Intersection;

pub struct LightMaterial {
    color: Color,
    intensity: f32,
}

impl LightMaterial {
    pub fn new(color: Color, intensity: f32) -> LightMaterial {
        LightMaterial {
            color,
            intensity,
        }
    }
}

impl<'a> Material for LightMaterial {
    fn trace(&self, _scene: &Scene, _ray: &Ray, _intersection: &Intersection, _depth: u32) -> Color {
        //let u = intersection.u;
        //let v = intersection.v;
        //let w = 1.0 - u - v;
        //let v0 = mesh.vertices()[intersection.face.vertex_ids[0] as usize];
        //let v1 = mesh.vertices()[intersection.face.vertex_ids[1] as usize];
        //let v2 = mesh.vertices()[intersection.face.vertex_ids[2] as usize];
        //let p = ray.origin + ray.direction * intersection.t;
        //let n = v0.normal*w + v1.normal*u + v2.normal*v;
        //let t = v0.texture_coords*w + v1.texture_coords*u + v2.texture_coords*v;
        self.color*self.intensity
    }
}