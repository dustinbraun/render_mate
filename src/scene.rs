use rand::Rng;

use crate::Camera;
use crate::Color;
use crate::Framebuffer;
use crate::Intersection;
use crate::Node;
use crate::PointLight;
use crate::Ray;
use crate::Vec3;

pub struct Scene<'a> {
    nodes: Vec<&'a dyn Node>,
    point_lights: Vec<PointLight>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            nodes: vec![],
            point_lights: vec![],
        }
    }

    pub fn add_node(&mut self, node: &'a dyn Node) {
        self.nodes.push(node);
    }

    pub fn add_point_light(&mut self, point_light: PointLight) {
        self.point_lights.push(point_light);
    }

    pub fn render(&self, camera: &Camera, framebuffer: &mut Framebuffer) {
        let sample_count = 200;
        for y in 0..framebuffer.get_extent().height {
            for x in 0..framebuffer.get_extent().width {
                let ray = camera.get_ray(x, y);
                let mut average_color = Color::new(0.0, 0.0, 0.0, 1.0);
                for _ in 0..sample_count {
                    average_color = average_color + self.cast_ray(&ray, 5);
                }
                average_color = average_color*(1.0/sample_count as f32);
                framebuffer.put_pixel(x, y, average_color);
            }
            println!("{}", y);
        }
    }

    fn cast_ray(&self, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            Color::new(
                0.0,
                0.0,
                0.0,
                1.0,
            )
        }
        else {
            if let Some(intersection) = self.intersects_ray(ray) {
                if intersection.emission > 0.0 {
                    intersection.color*intersection.emission
                }
                else {
                    let ray_direction = compute_random_hemisphere_point(&intersection.normal);
                    let ray_origin = intersection.position;
                    let next_ray = Ray {
                        origin: ray_origin,
                        direction: ray_direction,
                        t_min: 0.001,
                        t_max: 10000.0,
                    };
                    intersection.color*self.cast_ray(&next_ray, depth - 1)
                }
            }
            else {
                Color::new(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                )
            }
        }
    }

    /*fn cast_ray2(&self, ray: &Ray, depth: u32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0, 1.0);
        if let Some(intersection) = self.intersects_ray(ray) {

            

            for point_light in self.point_lights.iter() {
                let shadow_ray_direction = (point_light.position - intersection.position).normalize();
                let shadow_ray_origin = intersection.position;
                let shadow_ray = Ray {
                    origin: shadow_ray_origin,
                    direction: shadow_ray_direction,
                    t_min: 0.001,
                    t_max: (point_light.position - intersection.position).len(),
                };
                if self.intersects_ray(&shadow_ray).is_none() {
                    color = color + intersection.color*point_light.color*intersection.normal.dot(shadow_ray_direction);
                }
            }
        }
        color
    }*/

    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        let mut best: Option<Intersection> = None;
        for node in self.nodes.iter() {
            if let Some(intersection) = node.intersects_ray(ray) {
                if let Some(best_intersection) = best {
                    if intersection.t < best_intersection.t {
                        best = Some(intersection);
                    }
                }
                else {
                    best = Some(intersection);
                }
            }
        }
        best
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