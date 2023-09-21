use crate::BoundingBox;
use crate::BoundingVolume;
use crate::Face;
use crate::Intersection;
use crate::Node;
use crate::Ray;
use crate::Texture;
use crate::Vec2;
use crate::Vec3;
use crate::Vertex;

pub struct Mesh<'a> {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    texture: &'a Texture,
    emission: f32,
    bounding_box: BoundingBox,

}

impl<'a> Mesh<'a> {
    pub fn new_cube(texture: &'a Texture, translation: Vec3, scale: Vec3, emission: f32) -> Mesh<'a> {
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
            vertex.position.x *= scale.x;
            vertex.position.y *= scale.y;
            vertex.position.z *= scale.z;
        }
        for vertex in vertices.iter_mut() {
            vertex.position += translation;
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

    fn face_intersects_ray(&self, face: &Face, ray: &Ray) -> Option<Intersection> {
        let p0 = self.vertices[face.vertex_ids[0] as usize];
        let p1 = self.vertices[face.vertex_ids[1] as usize];
        let p2 = self.vertices[face.vertex_ids[2] as usize];

        let v0v1 = p1.position - p0.position;
        let v0v2 = p2.position - p0.position;
        let n = v0v1.cross(v0v2);

        let denom = n.dot(n);

        let n_dot_ray_dir = n.dot(ray.direction);
        if n_dot_ray_dir.abs() < 0.00001 {
            return None;
        }
        let d = -n.dot(p0.position);
        let t = -(n.dot(ray.origin) + d) / n_dot_ray_dir;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction*t;
        let edge0 = p1.position - p0.position;
        let vp0 = p - p0.position;
        let c = edge0.cross(vp0);
        if n.dot(c) < 0.0 {
            return None;
        }

        let w = n.dot(c)/denom;

        let edge1 = p2.position - p1.position;
        let vp1 = p - p1.position;
        let c = edge1.cross(vp1);
        if n.dot(c) < 0.0 {
            return None;
        }

        let u = n.dot(c)/denom;

        let edge2 = p0.position - p2.position;
        let vp2 = p - p2.position;
        let c = edge2.cross(vp2);
        if n.dot(c) < 0.0 {
            return None;
        }

        let v = n.dot(c)/denom;

        if t > ray.t_max || t < ray.t_min {
            return None;
        }

        let mut normal = n.normalize();
        if normal.dot(ray.direction) > 0.0 {
            normal *= -1.0;
        }

        let texture_coords = p0.texture_coords*u + p1.texture_coords*v + p2.texture_coords*w;

        Some(Intersection {
            position: p,
            normal,
            color: self.texture.sample(texture_coords),
            t: t,
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
            if let Some(intersection) = self.face_intersects_ray(face, ray) {
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