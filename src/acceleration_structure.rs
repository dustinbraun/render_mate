use std::ops::Bound;

use crate::BoundingBox;
use crate::BoundingVolume;
use crate::Face;
use crate::Intersection;
use crate::Vertex;
use crate::Ray;
use crate::IntersectionPayload;
use crate::Mesh;
use crate::Geometry;

#[derive(Clone)]
pub struct AsNode {
    faces: Vec<Face>,
    nodes: Option<Box<[AsNode; 2]>>,
    bounding_box: BoundingBox,
}

pub struct AccelerationStructure {
    root_node: AsNode,
    vertices: Vec<Vertex>,
}

impl<'a> AccelerationStructure {
    pub fn new(vertices: &[Vertex], faces: &[Face]) -> AccelerationStructure {

        let mut root_node = AsNode {
            faces: faces.to_vec(),
            nodes: None,
            bounding_box: compute_bounding_box(vertices, faces),
        };

        split_x_rec(vertices, &mut root_node, 9);

        let mut accs = AccelerationStructure {
            root_node,
            vertices: vertices.to_vec(),
        };

        accs
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }


    

    pub fn intersects_ray_rec(&'a self, ray: &Ray, node: &'a AsNode, stack: &mut Vec<&'a AsNode>) {
        if node.bounding_box.intersects_ray(ray) {
            if let Some(nodes) = &node.nodes {
                self.intersects_ray_rec(ray, &nodes[0], stack);
                self.intersects_ray_rec(ray, &nodes[1], stack);
            }
            else {
                stack.push(node);
            }
        }
    }


    pub fn node_intersects_ray(&'a self, node: &'a AsNode, ray: &Ray) -> Option<Intersection> {
        let mut best: Option<Intersection> = None;
        for face in node.faces.iter() {
            if let Some(intersection) = self.face_intersects_ray(ray, face) {
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

    pub fn face_intersects_ray(&'a self, ray: &Ray, face: &'a Face) -> Option<Intersection<'a>> {
        let v0 = self.vertices[face.vertex_ids[0] as usize];
        let v1 = self.vertices[face.vertex_ids[1] as usize];
        let v2 = self.vertices[face.vertex_ids[2] as usize];
        let edge1 = v1.position - v0.position;
        let edge2 = v2.position - v0.position;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);
        if a.abs() < 0.00001 {
            return None;
        }
        let f = 1.0/a;
        let s = ray.origin - v0.position;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }
        let t = f * edge2.dot(q);
        if t > ray.t_max || t < ray.t_min {
            return None;
        }
        let w = 1.0 - u - v;
        let _texture_coords = v0.texture_coords*w + v1.texture_coords*u + v2.texture_coords*v;
        let mut normal = v0.normal*w + v1.normal*u + v2.normal*v;
        if normal.dot(ray.direction) > 0.0 {
            normal *= -1.0;
        }
        Some(Intersection {
            t,
            payload: IntersectionPayload::AccelerationStructure {
                acceleration_structure: self,
                face,
                u,
                v,
            },
        })
    }
}


impl Geometry for AccelerationStructure {
    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        // ToDo: Avoid heap-alloc
        let mut stack: Vec<&AsNode> = vec![];
        self.intersects_ray_rec(ray, &self.root_node, &mut stack);

        let mut best: Option<Intersection> = None;
        for node in stack.iter() {
            if let Some(intersection) = self.node_intersects_ray(node, ray) {
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


fn compute_bounding_box(vertices: &[Vertex], faces: &[Face]) -> BoundingBox {
    let mut min = vertices[faces[0].vertex_ids()[0] as usize].position;
    let mut max = vertices[faces[0].vertex_ids()[0] as usize].position;
    for face in faces.iter() {
        for vertex_id in face.vertex_ids().iter() {
            let vertex = vertices[*vertex_id as usize];
            if vertex.position.x < min.x {
                min.x = vertex.position.x;
            }
            if vertex.position.y < min.y {
                min.y = vertex.position.y;
            }
            if vertex.position.z < min.z {
                min.z = vertex.position.z;
            }
            if vertex.position.x > max.x {
                max.x = vertex.position.x;
            }
            if vertex.position.y > max.y {
                max.y = vertex.position.y;
            }
            if vertex.position.z > max.z {
                max.z = vertex.position.z;
            }
        }
    }
    BoundingBox::new(
        min,
        max,
    )
}


fn compute_x_median(vertices: &[Vertex], node: &AsNode) -> f32 {
    let mut median = 0.0;
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        median += v0.position.x + v1.position.x + v2.position.x;
    }
    median /= (node.faces.len()*3) as f32;
    median
}

fn compute_y_median(vertices: &[Vertex], node: &AsNode) -> f32 {
    let mut median = 0.0;
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        median += v0.position.y + v1.position.y + v2.position.y;
    }
    median /= (node.faces.len()*3) as f32;
    median
}

fn compute_z_median(vertices: &[Vertex], node: &AsNode) -> f32 {
    let mut median = 0.0;
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        median += v0.position.z + v1.position.z + v2.position.z;
    }
    median /= (node.faces.len()*3) as f32;
    median
}

fn split_x(vertices: &[Vertex], node: &mut AsNode) {
    let mut faces0: Vec<Face> = vec![];
    let mut faces1: Vec<Face> = vec![];
    let median = compute_x_median(vertices, node);
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        if v0.position.x <= median || v1.position.x <= median || v2.position.x <= median {
            faces0.push(*face);
        }
    }
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        if v0.position.x >= median || v1.position.x >= median || v2.position.x >= median {
            faces1.push(*face);
        }
    }
    let nodes = [
        AsNode { faces: faces0.clone(), nodes: None, bounding_box: compute_bounding_box(&vertices, &faces0) },
        AsNode { faces: faces1.clone(), nodes: None, bounding_box: compute_bounding_box(&vertices, &faces1) },
    ];
    node.nodes = Some(Box::new(nodes));
}

fn split_y(vertices: &[Vertex], node: &mut AsNode) {
    let mut faces0: Vec<Face> = vec![];
    let mut faces1: Vec<Face> = vec![];
    let median = compute_y_median(vertices, node);
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        if v0.position.y <= median || v1.position.y <= median || v2.position.y <= median {
            faces0.push(*face);
        }
    }
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        if v0.position.y >= median || v1.position.y >= median || v2.position.y >= median {
            faces1.push(*face);
        }
    }
    let nodes = [
        AsNode { faces: faces0.clone(), nodes: None, bounding_box: compute_bounding_box(&vertices, &faces0) },
        AsNode { faces: faces1.clone(), nodes: None, bounding_box: compute_bounding_box(&vertices, &faces1) },
    ];
    node.nodes = Some(Box::new(nodes));
}

fn split_z(vertices: &[Vertex], node: &mut AsNode) {
    let mut faces0: Vec<Face> = vec![];
    let mut faces1: Vec<Face> = vec![];
    let median = compute_z_median(vertices, node);
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        if v0.position.z <= median || v1.position.z <= median || v2.position.z <= median {
            faces0.push(*face);
        }
    }
    for face in node.faces.iter() {
        let v0 = vertices[face.vertex_ids()[0] as usize];
        let v1 = vertices[face.vertex_ids()[1] as usize];
        let v2 = vertices[face.vertex_ids()[2] as usize];
        if v0.position.z >= median || v1.position.z >= median || v2.position.z >= median {
            faces1.push(*face);
        }
    }
    let nodes = [
        AsNode { faces: faces0.clone(), nodes: None, bounding_box: compute_bounding_box(&vertices, &faces0) },
        AsNode { faces: faces1.clone(), nodes: None, bounding_box: compute_bounding_box(&vertices, &faces1) },
    ];
    node.nodes = Some(Box::new(nodes));
}


fn split_x_rec(vertices: &[Vertex], node: &mut AsNode, depth: u32) {
    if depth == 0 {
        return;
    }
    split_x(vertices, node);
    if let Some(nodes) = &mut node.nodes {
        split_y_rec(vertices, &mut nodes[0], depth - 1);
        split_y_rec(vertices, &mut nodes[1], depth - 1);
    }
}

fn split_y_rec(vertices: &[Vertex], node: &mut AsNode, depth: u32) {
    if depth == 0 {
        return;
    }
    split_y(vertices, node);
    if let Some(nodes) = &mut node.nodes {
        split_z_rec(vertices, &mut nodes[0], depth - 1);
        split_z_rec(vertices, &mut nodes[1], depth - 1);
    }
}

fn split_z_rec(vertices: &[Vertex], node: &mut AsNode, depth: u32) {
    if depth == 0 {
        return;
    }
    split_z(vertices, node);
    if let Some(nodes) = &mut node.nodes {
        split_x_rec(vertices, &mut nodes[0], depth - 1);
        split_x_rec(vertices, &mut nodes[1], depth - 1);
    }
}