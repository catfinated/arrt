use std::rc::Rc;

use serde;
use serde::{Serialize, Deserialize};

use crate::math::*;
use crate::aabb::AABB;

use super::mesh::Mesh;
use super::material::Surfel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub mesh: String,
    pub material: String,
    #[serde(default)]
    pub transform: Transform
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Vec3,
    pub scale: Vec3,
}

pub struct Model {
    mesh: Rc<Mesh>,
    material: usize,
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    pub bbox: AABB
}

impl Default for Transform {
    fn default() -> Self {
        Self { translate: Vec3::zeros(),
               rotate: Vec3::zeros(),
               scale: Vec3::ones() }
    }
}

impl Transform {
    pub fn new(translate: Vec3, rotate: Vec3, scale: Vec3) -> Transform {
        Transform{ translate, rotate, scale }
    }

    pub fn mat4(&self) -> Mat4 {
        let t = Mat4::translate(&self.translate);
        let s = Mat4::scale(&self.scale);
        let rx = Mat4::rotate_x(Degree(self.rotate.x()));
        let ry = Mat4::rotate_y(Degree(self.rotate.y()));
        let rz = Mat4::rotate_z(Degree(self.rotate.z()));
        let r = &(&rx * &ry) * &rz;
        &(&t * &r) * &s
    }
}

impl Model {
    pub fn new(mesh: Rc<Mesh>, material: usize, transform: &Transform) -> Model {
        let m = transform.mat4();

        let mut vertices = Vec::with_capacity(mesh.vertices.len());
        let mut normals = Vec::with_capacity(mesh.vertices.len());
        let mut counts = Vec::with_capacity(mesh.vertices.len());
        normals.resize(mesh.vertices.len(), Vec3::zeros());
        counts.resize(mesh.vertices.len(), 0);

        let mut box_min = Vec3::fill(f32::MAX);
        let mut box_max = Vec3::fill(f32::MIN);

        // transform vertices and compute bbox vertexes
        for vert in &mesh.vertices {
            let v = (&m * Vec4::from_vec3(*vert, 1.0_f32)).to_vec3();
            vertices.push(v);

            box_min.set_x(v.x().min(box_min.x()));
            box_min.set_y(v.y().min(box_min.y()));
            box_min.set_z(v.z().min(box_min.z()));

            box_max.set_x(v.x().max(box_max.x()));
            box_max.set_y(v.y().max(box_max.y()));
            box_max.set_z(v.z().max(box_max.z()));
        }

        // compute normals
        for tri in &mesh.triangles {
            let v0 = vertices[tri.i];
            let v1 = vertices[tri.j];
            let v2 = vertices[tri.k];

            let a = v1 - v0;
            let b = v2 - v0;
            let c = cross(a, b);
            let n = normalize(c);

            normals[tri.i] = normals[tri.i] + n;
            normals[tri.j] = normals[tri.j] + n;
            normals[tri.k] = normals[tri.k] + n;
            counts[tri.i] += 1;
            counts[tri.j] += 1;
            counts[tri.k] += 1;
        }

        // average normals
        for (idx, norm) in normals.iter_mut().enumerate() {
            let count = counts[idx] as f32;
            *norm = normalize(*norm / count);
        }

        let bbox = AABB::new(box_min, box_max);
        //normals = mesh.normals.clone();
        Model{ mesh, material, vertices, normals, bbox }
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {

        let mut t_range = range;
        let mut surfel = None;

        for tri in &self.mesh.triangles {
            if let Some(surf) = tri.intersect(ray, t_range, &self.vertices, &self.normals) {
                t_range.max = surf.t;
                surfel = Some(Surfel{material_id: self.material, ..surf});
            }
        }

        surfel
    }

    pub fn centroid(&self) -> Vec3 {
        self.bbox.center()
    }
}
