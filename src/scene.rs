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
        let sample_count = 2000;
        for y in 0..framebuffer.get_extent().height {
            for x in 0..framebuffer.get_extent().width {
                let mut average_color = Color::new(0.0, 0.0, 0.0, 1.0);
                for _ in 0..sample_count {
                    let ray = camera.get_ray(x, y);
                    average_color = average_color + self.cast_ray(&ray, 5);
                }
                average_color = average_color*(1.0/sample_count as f32);
                framebuffer.put_pixel(x, y, average_color);
            }
            println!("{}", y);
        }
    }

    // See https://en.wikipedia.org/wiki/Path_tracing
    // for implementation details.
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
                    if intersection.scatter > 0.5 {
                        let ray_direction = compute_random_hemisphere_point(&intersection.normal);
                        let ray_origin = intersection.position;
                        let next_ray = Ray {
                            origin: ray_origin,
                            direction: ray_direction,
                            t_min: 0.00001,
                            t_max: 10000.0,
                        };

                        let p = 1.0/(3.14*2.0);
                        let cos_theta = next_ray.direction.dot(intersection.normal);
                        let brdf = intersection.color/3.14;
                        let incoming = self.cast_ray(&next_ray, depth - 1);
                        //brdf * incoming * cos_theta / p
                        incoming*intersection.color*cos_theta
                        
                    }
                    else {
                        let ray_direction = ray.direction.reflect(intersection.normal);
                        let ray_origin = intersection.position;
                        let next_ray = Ray {
                            origin: ray_origin,
                            direction: ray_direction,
                            t_min: 0.00001,
                            t_max: 10000.0,
                        };

                        
                        let incoming = self.cast_ray(&next_ray, depth - 1);
                        incoming * intersection.color
                    }
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