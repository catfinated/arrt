use std::sync::Arc;

use serde;
use serde::{Serialize, Deserialize};

use crate::math::*;
use crate::aabb::AABB;

use super::mesh::Mesh;
use super::material::{Surfel, MaterialID};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Vec3,
    pub scale: Vec3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub mesh: String,
    pub material: String,
    #[serde(default)]
    pub transform: Transform
}

pub struct Model {
    mesh: Mesh,
    material_id: MaterialID,
    normals: Vec<Vec3>,
    pub bbox: AABB
}

pub struct ModelInstance {
    model: Arc<Model>,
    material_id: MaterialID,
    pub bbox: AABB,
    transform: Mat4,
    inverse: Mat4
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

    /// Create combined transformation matrix
    pub fn mat4(&self) -> Mat4 {
        let t = Mat4::translate(&self.translate);
        let s = Mat4::scale(&self.scale);
        let rx = Mat4::rotate_x(Degree(self.rotate.x()));
        let ry = Mat4::rotate_y(Degree(self.rotate.y()));
        let rz = Mat4::rotate_z(Degree(self.rotate.z()));
        let r = &(&rx * &ry) * &rz;
        &(&t * &r) * &s
    }

    /// Create inverse transformation matrix
    pub fn inverse(&self) -> Mat4 {
        let t = Mat4::itranslate(&self.translate);
        let s = Mat4::iscale(&self.scale);
        let rx = Mat4::irotate_x(Degree(self.rotate.x()));
        let ry = Mat4::irotate_y(Degree(self.rotate.y()));
        let rz = Mat4::irotate_z(Degree(self.rotate.z()));
        let r = &(&rz * &ry) * &rx;
        &(&s * &r) * &t
    }
}

impl Model {
    pub fn new(fpath: &String, dpath: &String, material_id: MaterialID) -> Model { 
        let mesh = Mesh::new(fpath, dpath);
        let mut normals = Vec::with_capacity(mesh.vertices.len());
        let mut counts = Vec::with_capacity(mesh.vertices.len());
        normals.resize(mesh.vertices.len(), Vec3::zeros());
        counts.resize(mesh.vertices.len(), 0);

        let mut box_min = Vec3::fill(f32::MAX);
        let mut box_max = Vec3::fill(f32::MIN);

        // todo: vertices/normals/bbox now redundant between model and mesh.
        // combine the two since transformations are now handled with instancing.
        for v in &mesh.vertices {
            box_min.set_x(v.x().min(box_min.x()));
            box_min.set_y(v.y().min(box_min.y()));
            box_min.set_z(v.z().min(box_min.z()));

            box_max.set_x(v.x().max(box_max.x()));
            box_max.set_y(v.y().max(box_max.y()));
            box_max.set_z(v.z().max(box_max.z()));
        }

        // compute normals
        for tri in &mesh.triangles {
            let v0 = mesh.vertices[tri.i];
            let v1 = mesh.vertices[tri.j];
            let v2 = mesh.vertices[tri.k];

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
        println!("model bbox: {:?}", bbox);
        Model{ mesh, material_id, normals, bbox }
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {

        let mut t_range = range;
        let mut surfel = None;

        for tri in &self.mesh.triangles {
            if let Some(surf) = tri.intersect(ray, t_range, &self.mesh.vertices, &self.normals) {
                t_range.max = surf.t;
                surfel = Some(Surfel{material_id: self.material_id, ..surf});
            }
        }

        surfel
    }

    pub fn centroid(&self) -> Vec3 {
        self.bbox.center()
    }
}

impl ModelInstance {
    pub fn new(model: Arc<Model>, material_id: MaterialID, transformations: &Transform) -> Self {
        let transform = transformations.mat4();
        let inverse = transformations.inverse();
        let bbox = model.bbox.transform(&transform);
        println!("instance bbox: {:?} center: {:?}", bbox, bbox.center());
        ModelInstance{model, material_id, bbox, transform, inverse}
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let o = (&self.inverse * Vec4::from_vec3(ray.origin, 1.0_f32)).to_vec3();
        let d = (&self.inverse * Vec4::from_vec3(ray.direction, 0.0_f32)).to_vec3();
        let r = Ray{origin: o, direction: normalize(d)};
        let mut surfel = None;

        if let Some(surf) = self.model.intersect(&r, range) {
            let hit_point = (&self.transform * Vec4::from_vec3(surf.hit_point, 1.0_f32)).to_vec3();
            // original c++ impl had a note about using the t value computed from model space 
            // intersection here being incorrect and this seems true. however, suffern text says it should
            // be passed back unmodified but this leads to incorrect clipping
            //println!("hit point: {:?} t {} tpoint: {:?}", hit_point, t, ray.point_at(t));
            //let t = surf.t;
            let t = (hit_point - ray.origin).x() / ray.direction.x();
            let it = self.inverse.transpose();
            let v4 = &it * Vec4::from_vec3(surf.normal, 0.0_f32);
            let normal = normalize(v4.to_vec3());
            let material_id = self.material_id;
            surfel = Some(Surfel{t, hit_point, normal, material_id})
        }
        surfel
    }

    pub fn centroid(&self) -> Vec3 {
        self.bbox.center()
    }
}