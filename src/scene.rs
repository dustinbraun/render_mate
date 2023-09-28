use crate::Camera;
use crate::Color;
use crate::Framebuffer;
use crate::Intersection;
use crate::Ray;
use crate::Node;

pub struct Scene<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node<'a>) {
        self.nodes.push(node);
    }

    pub fn render(&self, camera: &Camera, framebuffer: &mut Framebuffer) {
        let sample_count = 500;
        for y in 0..framebuffer.get_extent().height {
            for x in 0..framebuffer.get_extent().width {
                let mut average_color = Color::new(0.0, 0.0, 0.0, 1.0);
                for _ in 0..sample_count {
                    let ray = camera.get_ray(x, y);
                    average_color = average_color + self.trace(&ray, 10);
                }
                average_color = average_color*(1.0/sample_count as f32);
                framebuffer.put_pixel(x, y, average_color);
            }
            println!("{}", y);
        }
    }

    pub fn trace(&self, ray: &Ray, depth: u32) -> Color {
        let mut best: Option<Intersection> = None;
        let mut best_mesh: Option<Node> = None;
        for node in self.nodes.iter() {
            if let Some(intersection) = node.geometry.intersects_ray(ray) {
                if let Some(best_intersection) = best {
                    if intersection.t < best_intersection.t {
                        best = Some(intersection);
                        best_mesh = Some(*node);
                    }
                }
                else {
                    best = Some(intersection);
                    best_mesh = Some(*node);
                }
            }
        }
        if let Some(best_intersection) = best {
            if let Some(best_mesh) = best_mesh {
                best_mesh.material.trace(self, ray, &best_intersection, depth)
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
