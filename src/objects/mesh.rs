#![allow(non_snake_case)]
use std::sync::Arc;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde;
use serde::{Serialize, Deserialize};

use crate::math::{Mat3, Mat4, Range, Ray, Vec3, Vec4, cross, determinant, in_range, normalize};
use super::aabb::Aabb;
use super::material::{Surfel, MaterialID};
use super::object::Object;
use super::transform::Transform;

pub struct Triangle {
    pub i: usize,
    pub j: usize,
    pub k: usize
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub normals: Vec<Vec3>,
    pub bbox: Aabb,
}

pub struct Instance {
    model: Arc<Mesh>,
    material_id: MaterialID,
    pub bbox: Aabb,
    transform: Mat4,
    inverse: Mat4
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MeshConfig {
    pub mesh: String,
    pub material: String,
    #[serde(default)]
    pub transform: Transform
}


impl Triangle {

    pub fn intersect(&self, ray: &Ray,
                     range: Range,
                     vertices: &[Vec3],
                     normals: &[Vec3]) -> Option<Surfel> {
        let v0 = vertices[self.i];
        let v1 = vertices[self.j];
        let v2 = vertices[self.k];

        let a = v0.x() - v1.x();
        let b = v0.x() - v2.x();
        let c = ray.direction.x();
        let d = v0.x() - ray.origin.x();

        let e = v0.y() - v1.y();
        let f = v0.y() - v2.y();
        let g = ray.direction.y();
        let h = v0.y() - ray.origin.y();

        let i = v0.z() - v1.z();
        let j = v0.z() - v2.z();
        let k = ray.direction.z();
        let l = v0.z() - ray.origin.z();

        let A = Mat3::new(a, b, c,
                          e, f, g,
                          i, j, k);

        let B = Mat3::new(d, b, c,
                          h, f, g,
                          l, j, k);

        let Y = Mat3::new(a, d, c,
                          e, h, g,
                          i, l, k);

        let T = Mat3::new(a, b, d,
                          e, f, h,
                          i, j, l);

        let denom = 1.0_f32 / determinant(&A);
        let beta = determinant(&B) * denom;

        let spf = 1e-6_f32;

        if beta < spf { return None; }

        let gamma = determinant(&Y) * denom;

        if gamma < spf { return None; }

        if beta + gamma > 1.0_f32 { return None; }

        let t = determinant(&T) * denom;

        if !in_range(range, t) {
            return None;
        }

        let hit_point = ray.point_at(t);
        let alpha = (1.0_f32 - beta - gamma).max(0.0_f32);
        let normal = normalize((alpha * normals[self.i]) +
                     (beta * normals[self.j]) +
                     (gamma * normals[self.k]));
        let material_id = MaterialID(0);

        Some(Surfel{t, hit_point, normal, material_id, n_offset: 0.0_f32})
    }

}

pub fn compute_normals(vertices: &[Vec3], triangles: &[Triangle], flip: bool) -> Vec<Vec3> {
    let mut counts = Vec::with_capacity(vertices.len());
    let mut normals = Vec::new();
    normals.resize(vertices.len(), Vec3::zeros());
    counts.resize(vertices.len(), 0);

    // compute normals
    for tri in triangles {
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
        if flip {
            *norm = -*norm;
        }
    }  
    normals
}

impl Mesh {
    pub fn fromSMF(fpath: &String, dpath: &String) -> Mesh {
        let mut vertices= Vec::new();
        let mut triangles = Vec::new();
        let mut normals = Vec::new();
        let path = Path::new(dpath).join(fpath);
        println!("loading model mesh from: {:#?}", path);

        let file = match File::open(&path) {
            Err(why) => panic!("failed to open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut box_min = Vec3::fill(f32::MAX);
        let mut box_max = Vec3::fill(f32::MIN);
        let lines = io::BufReader::new(file).lines();

        for line in lines.map_while(Result::ok) {
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                let split = line.split_whitespace();
                let vec: Vec<&str> = split.collect();

                if vec.len() < 4 {
                    continue; // dragon.smf has a single line with 'f\n'
                }

                if vec[0] == "v" {
                    let x = vec[1].parse::<f32>().unwrap();
                    let y = vec[2].parse::<f32>().unwrap();
                    let z = vec[3].parse::<f32>().unwrap();
                    let v = Vec3::new(x, y, z);

                    box_min.set_x(v.x().min(box_min.x()));
                    box_min.set_y(v.y().min(box_min.y()));
                    box_min.set_z(v.z().min(box_min.z()));

                    box_max.set_x(v.x().max(box_max.x()));
                    box_max.set_y(v.y().max(box_max.y()));
                    box_max.set_z(v.z().max(box_max.z()));

                    vertices.push(v);
                }
                else if vec[0] == "f" {
                    let i = vec[1].parse::<usize>().unwrap() - 1;
                    let j = vec[2].parse::<usize>().unwrap() - 1;
                    let k = vec[3].parse::<usize>().unwrap() - 1;
                    triangles.push(Triangle{ i, j, k });
                }
                else if vec[0] == "n" {
                    let a = vec[1].parse::<f32>().unwrap();
                    let b = vec[2].parse::<f32>().unwrap();
                    let c = vec[3].parse::<f32>().unwrap();
                    normals.push(Vec3::new(a, b, c));
                }
        }

        if normals.is_empty() {
            normals = compute_normals(&vertices, &triangles, false);
        }

        let bbox = Aabb::new(box_min, box_max);
        println!("mesh bbox: {:?}", bbox);
        Mesh{ vertices, triangles, normals, bbox }
    }
}

impl Object for Mesh {

    fn bbox(&self) -> Option<Aabb>
    {
        Some(self.bbox)
    }

    fn centroid(&self) -> Vec3
    {
        self.bbox.center()
    }

    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {

        let mut t_range = range;
        let mut surfel = None;

        for tri in &self.triangles {
            if let Some(surf) = tri.intersect(ray, t_range, &self.vertices, &self.normals) {
                t_range.max = surf.t;
                surfel = Some(Surfel{..surf});
            }
        }

        surfel
    }
}

impl Instance {
    pub fn new(model: Arc<Mesh>, material_id: MaterialID, transformations: &Transform) -> Self {
        let transform = transformations.mat4();
        let inverse = transformations.inverse();
        let bbox = model.bbox.transform(&transform);
        println!("instance bbox: {:?} center: {:?}", bbox, bbox.center());
        Instance{model, material_id, bbox, transform, inverse}
    }
}

impl Object for Instance {

    fn bbox(&self) -> Option<Aabb>
    {
        Some(self.bbox)
    }

    fn centroid(&self) -> Vec3
    {
        self.bbox.center()
    }

    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let o = (&self.inverse * Vec4::from_vec3(ray.origin, 1.0_f32)).to_vec3();
        let d = (&self.inverse * Vec4::from_vec3(ray.direction, 0.0_f32)).to_vec3();
        let r = Ray{origin: o, direction: normalize(d), depth: ray.depth};
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
            surfel = Some(Surfel{t, hit_point, normal, material_id, n_offset: 0.0000000001})
        }
        surfel
    }
}