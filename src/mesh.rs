use rand::seq::index;

use crate::BoundingBox;
use crate::BoundingVolume;
use crate::Face;
use crate::Mat4;
use crate::Ray;
use crate::Vec2;
use crate::Vec3;
use crate::Vec4;
use crate::Vertex;
use crate::Intersection;
use crate::IntersectionPayload;
use crate::Geometry;
use crate::AccelerationStructure;

pub struct Mesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    bounding_box: BoundingBox,
    acceleration_structure: AccelerationStructure,
}

impl<'a> Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    pub fn from_obj(path: &str, transformation: Mat4) -> Mesh {
        
        let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).unwrap();
        
        let mut vertices: Vec<Vertex> = vec![];
        let mut faces: Vec<Face> = vec![];
        for model in models.iter() {
            for vertex_chunk in model.mesh.positions.chunks_exact(3) {
                vertices.push(Vertex::new(
                    Vec3::new(vertex_chunk[0], vertex_chunk[1], vertex_chunk[2]),
                    Vec2::new(0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                ));
            }
            for index_chunk in model.mesh.indices.chunks_exact(3) {
                faces.push(Face::new([
                    index_chunk[0],
                    index_chunk[1],
                    index_chunk[2],
                ]));
            }
        }


        /*let obj = obj::Obj::load(path).unwrap();
        let mut vertices: Vec<Vertex> = vec![];
        for vertex in obj.data.position.iter() {
            vertices.push(Vertex::new(
                Vec3::new(vertex[0], vertex[1], vertex[2]),
                Vec2::new(0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ));
        }
        let mut faces: Vec<Face> = vec![];
        for object in obj.data.objects.iter() {
            for group in object.groups.iter() {
                for poly in group.polys.iter() {
                    for simple_poly in poly.0.iter() {
                        if let Some(s1) = simple_poly.1 {
                            if let Some(s2) = simple_poly.2 {
                                faces.push(Face::new([
                                    simple_poly.0 as u32,
                                    s1 as u32,
                                    s2 as u32,
                                ]));
                            }
                        }
                    }
                }
            }
        }*/


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
            //vertex.normal = Mat3::from(transformation.inverse().transpose())*vertex.normal;
            //vertex.normal = vertex.normal.normalize();
        }

        for vertex in vertices.iter_mut() {
            vertex.normal.x = 0.0;
            vertex.normal.y = 0.0;
            vertex.normal.z = 0.0;
        }
        for face in faces.iter() {
            let edge1 = vertices[face.vertex_ids[1] as usize].position - vertices[face.vertex_ids[0] as usize].position;
            let edge2 = vertices[face.vertex_ids[2] as usize].position - vertices[face.vertex_ids[0] as usize].position;
            let normal = edge1.cross(edge2).normalize();
            vertices[face.vertex_ids[0] as usize].normal += normal;
            vertices[face.vertex_ids[1] as usize].normal += normal;
            vertices[face.vertex_ids[2] as usize].normal += normal;
        }
        for vertex in vertices.iter_mut() {
            vertex.normal = vertex.normal.normalize();
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
        let acceleration_structure = AccelerationStructure::new(&vertices, &faces);
        Mesh {
            vertices,
            faces,
            bounding_box,
            acceleration_structure,
        }
    }

    pub fn new_cube(transformation: Mat4) -> Mesh {
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
            //vertex.normal = Mat3::from(transformation.inverse().transpose())*vertex.normal;
            //vertex.normal = vertex.normal.normalize();
        }

        for vertex in vertices.iter_mut() {
            vertex.normal.x = 0.0;
            vertex.normal.y = 0.0;
            vertex.normal.z = 0.0;
        }
        for face in faces.iter() {
            let edge1 = vertices[face.vertex_ids[1] as usize].position - vertices[face.vertex_ids[0] as usize].position;
            let edge2 = vertices[face.vertex_ids[2] as usize].position - vertices[face.vertex_ids[0] as usize].position;
            let normal = edge1.cross(edge2).normalize();
            vertices[face.vertex_ids[0] as usize].normal += normal;
            vertices[face.vertex_ids[1] as usize].normal += normal;
            vertices[face.vertex_ids[2] as usize].normal += normal;
        }
        for vertex in vertices.iter_mut() {
            vertex.normal = vertex.normal.normalize();
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
        let acceleration_structure = AccelerationStructure::new(&vertices, &faces);
        Mesh {
            vertices,
            faces,
            bounding_box,
            acceleration_structure,
        }

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
            payload: IntersectionPayload::Mesh {
                mesh: self,
                face,
                u,
                v,
            },
        })
    }
}

impl Geometry for Mesh {
    fn intersects_ray(&self, ray: &Ray) -> Option<Intersection> {
        
        if !self.bounding_box.intersects_ray(ray) {
            return None;
        }
        let mut best: Option<Intersection> = None;
        for face in self.faces.iter() {
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
}