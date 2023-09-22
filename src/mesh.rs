use crate::BoundingBox;
use crate::BoundingVolume;
use crate::Face;
use crate::Intersection;
use crate::Mat4;
use crate::Node;
use crate::Ray;
use crate::Texture;
use crate::Vec2;
use crate::Vec3;
use crate::Vec4;
use crate::Vertex;

pub struct Mesh<'a> {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    texture: &'a Texture,
    emission: f32,
    bounding_box: BoundingBox,

}

impl<'a> Mesh<'a> {
    pub fn new_cube(texture: &'a Texture, transformation: Mat4, emission: f32) -> Mesh<'a> {
        let mut vertices = vec![
            //------------------------------------------------------------
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0,  0.0), Vec3::new( 0.0,  0.0, -1.0)),
            Vertex::new(Vec3::new( 0.5, -0.5, -0.5), Vec2::new(1.0,  0.0), Vec3::new( 0.0,  0.0, -1.0)),
            Vertex::new(Vec3::new( 0.5,  0.5, -0.5), Vec2::new(1.0,  1.0), Vec3::new( 0.0,  0.0, -1.0)),
            Vertex::new(Vec3::new( 0.5,  0.5, -0.5), Vec2::new(1.0,  1.0), Vec3::new( 0.0,  0.0, -1.0)),
            Vertex::new(Vec3::new(-0.5,  0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 0.0,  0.0, -1.0)),
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0,  0.0), Vec3::new( 0.0,  0.0, -1.0)),
            //------------------------------------------------------------
            Vertex::new(Vec3::new(-0.5, -0.5,  0.5), Vec2::new(0.0,  0.0), Vec3::new( 0.0,  0.0,  1.0)),
            Vertex::new(Vec3::new( 0.5, -0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 0.0,  0.0,  1.0)),
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0,  1.0), Vec3::new( 0.0,  0.0,  1.0)),
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0,  1.0), Vec3::new( 0.0,  0.0,  1.0)),
            Vertex::new(Vec3::new(-0.5,  0.5,  0.5), Vec2::new(0.0,  1.0), Vec3::new( 0.0,  0.0,  1.0)),
            Vertex::new(Vec3::new(-0.5, -0.5,  0.5), Vec2::new(0.0,  0.0), Vec3::new( 0.0,  0.0,  1.0)),
            //------------------------------------------------------------
            Vertex::new(Vec3::new(-0.5,  0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new(-1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new(-0.5,  0.5, -0.5), Vec2::new(1.0,  1.0), Vec3::new(-1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new(-1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new(-1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new(-0.5, -0.5,  0.5), Vec2::new(0.0,  0.0), Vec3::new(-1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new(-0.5,  0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new(-1.0,  0.0,  0.0)),
            //------------------------------------------------------------
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new( 0.5,  0.5, -0.5), Vec2::new(1.0,  1.0), Vec3::new( 1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5,  0.5), Vec2::new(0.0,  0.0), Vec3::new( 1.0,  0.0,  0.0)),
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 1.0,  0.0,  0.0)),
            //------------------------------------------------------------
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 0.0, -1.0,  0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5, -0.5), Vec2::new(1.0,  1.0), Vec3::new( 0.0, -1.0,  0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 0.0, -1.0,  0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 0.0, -1.0,  0.0)),
            Vertex::new(Vec3::new(-0.5, -0.5,  0.5), Vec2::new(0.0,  0.0), Vec3::new( 0.0, -1.0,  0.0)),
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 0.0, -1.0,  0.0)),
            //------------------------------------------------------------
            Vertex::new(Vec3::new(-0.5,  0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 0.0,  1.0,  0.0)),
            Vertex::new(Vec3::new( 0.5,  0.5, -0.5), Vec2::new(1.0,  1.0), Vec3::new( 0.0,  1.0,  0.0)),
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 0.0,  1.0,  0.0)),
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0,  0.0), Vec3::new( 0.0,  1.0,  0.0)),
            Vertex::new(Vec3::new(-0.5,  0.5,  0.5), Vec2::new(0.0,  0.0), Vec3::new( 0.0,  1.0,  0.0)),
            Vertex::new(Vec3::new(-0.5,  0.5, -0.5), Vec2::new(0.0,  1.0), Vec3::new( 0.0,  1.0,  0.0)),
        ];
        let faces = vec![
            //---------------------
            Face::new([ 0,  1,  2]),
            Face::new([ 3,  4,  5]),
            //---------------------
            Face::new([ 6,  7,  8]),
            Face::new([ 9, 10, 11]),
            //---------------------
            Face::new([12, 13, 14]),
            Face::new([15, 16, 17]),
            //---------------------
            Face::new([18, 19, 20]),
            Face::new([21, 22, 23]),
            //---------------------
            Face::new([24, 25, 26]),
            Face::new([27, 28, 29]),
            //---------------------
            Face::new([30, 31, 32]),
            Face::new([33, 34, 35]),
        ];
        for vertex in vertices.iter_mut() {
            let mut v = Vec4::new(
                vertex.position.x,
                vertex.position.y,
                vertex.position.z,
                1.0,
            );
            v = transformation*v;
            vertex.position.x = v.x;
            vertex.position.y = v.y;
            vertex.position.z = v.z;
        }
        let mut min = vertices[0].position;
        let mut max = vertices[0].position;
        for vertex in vertices.iter() {
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
        min.x -= 0.001;
        min.y -= 0.001;
        min.z -= 0.001;
        max.x += 0.001;
        max.y += 0.001;
        max.z += 0.001;
        let bounding_box = BoundingBox::new(
            min,
            max,
        );
        Mesh {
            vertices,
            faces,
            texture,
            emission,
            bounding_box,
        }

    }

    // See https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    // for implementation details.
    pub fn ray_intersects_face(&self, ray: &Ray, face: &Face) -> Option<Intersection> {
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
        let texture_coords = v0.texture_coords*w + v1.texture_coords*u + v2.texture_coords*v;
        let mut normal = v0.normal*w + v1.normal*u + v2.normal*v;
        if normal.dot(ray.direction) > 0.0 {
            normal *= -1.0;
        }
        Some(Intersection {
            position: ray.origin + ray.direction*t,
            normal,
            color: self.texture.sample(texture_coords),
            t,
            emission: self.emission,
        })
    }
}

impl<'a> Node for Mesh<'a> {
    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounding_box.intersects_ray(ray) {
            return None;
        }
        let mut best: Option<Intersection> = None;
        for face in self.faces.iter() {
            if let Some(intersection) = self.ray_intersects_face(ray, face) {
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