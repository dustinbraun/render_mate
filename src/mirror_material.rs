use crate::Material;
use crate::Color;
use crate::Scene;
use crate::Ray;
use crate::Intersection;

pub struct MirrorMaterial {

}

impl Material for MirrorMaterial {
    fn trace(&self, scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(
                0.0,
                0.0,
                0.0,
                1.0,
            );
        }
        let u = intersection.u;
        let v = intersection.v;
        let w = 1.0 - u - v;
        let v0 = intersection.mesh.vertices()[intersection.face.vertex_ids[0] as usize];
        let v1 = intersection.mesh.vertices()[intersection.face.vertex_ids[1] as usize];
        let v2 = intersection.mesh.vertices()[intersection.face.vertex_ids[2] as usize];
        let p = ray.origin + ray.direction * intersection.t;
        let mut n = v0.normal*w + v1.normal*u + v2.normal*v;
        if n.dot(ray.direction) > 0.0 {
            n *= -1.0;
        }
        let next_ray_direction = ray.direction.reflect(n);
        let next_ray_origin = p;
        let next_ray = Ray {
            origin: next_ray_origin,
            direction: next_ray_direction,
            t_min: 0.00001,
            t_max: 10000.0,
        };
        let incoming = scene.trace(&next_ray, depth - 1);
        let cos_theta = next_ray.direction.dot(n);
        incoming*cos_theta

    }
}
