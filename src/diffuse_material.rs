use rand::Rng;

use crate::Material;
use crate::Color;
use crate::Scene;
use crate::Ray;
use crate::Intersection;
use crate::Texture;
use crate::Vec3;

pub struct DiffuseMaterial<'a> {
    pub texture: &'a Texture,
}

impl<'a> Material for DiffuseMaterial<'a> {
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
        let t = v0.texture_coords*w + v1.texture_coords*u + v2.texture_coords*v;
        let color = self.texture.sample(t);
        let next_ray_direction = compute_random_hemisphere_point(&n);
        let next_ray_origin = p;
        let next_ray = Ray {
            origin: next_ray_origin,
            direction: next_ray_direction,
            t_min: 0.00001,
            t_max: 10000.0,
        };
        let incoming = scene.trace(&next_ray, depth - 1);
        let cos_theta = next_ray.direction.dot(n);
        color*incoming*cos_theta

    }
}

fn compute_random_hemisphere_point(n: &Vec3) -> Vec3 {
    let mut direction = Vec3::new(
        rand::thread_rng().gen::<f32>()  - 0.5,
        rand::thread_rng().gen::<f32>()  - 0.5,
        rand::thread_rng().gen::<f32>()  - 0.5,
    ).normalize();
    if n.dot(direction) < 0.0 {
        direction.x *= -1.0;
        direction.y *= -1.0;
        direction.z *= -1.0;
    }
    direction
}