#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;
use std::f32;

use serde;
use serde::{Serialize, Deserialize};

use super::math::*;
use super::framebuffer::ColorRGB;
use super::aabb::AABB;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Material {
    pub name: String,
    pub ambient: ColorRGB,
    pub diffuse: ColorRGB,
    pub specular: ColorRGB,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub shininess: f32,
}

 impl Default for Material {
    fn default() -> Self {
        Self { name: "".to_string(),
               ambient: ColorRGB::black(),
               diffuse: ColorRGB::black(),
               specular: ColorRGB::black(),
               ka: 1.0_f32,
               kd: 1.0_f32,
               ks: 1.0_f32,
               shininess: 1.0_f32 }
    }
}

pub struct Surfel {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material_id: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereConfig {
    center: Vec3,
    radius: f32,
    pub material: String
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    material_id: usize,
    pub bbox: AABB
}

#[derive(Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub mesh: String,
    pub material: String,
    #[serde(default = "Vec3::zeros")]
    pub translate: Vec3,
    #[serde(default = "Vec3::zeros")]
    pub rotate: Vec3,
    #[serde(default = "Vec3::ones")]
    pub scale: Vec3,
    #[serde(skip)]
    pub material_id: usize,
    #[serde(skip)]
    triangles: Vec<Triangle>
}

pub struct IndexedTriangle {
    i: usize,
    j: usize,
    k: usize
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<IndexedTriangle>,
    pub normals: Vec<Vec3>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self { translate: Vec3::zeros(),
               rotate: Vec3::zeros(),
               scale: Vec3::ones() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub mesh: String,
    pub material: String,
    #[serde(default)]
    pub transform: Transform
}

pub struct Model2 {
    mesh: Rc<Mesh>,
    material: usize,
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    pub bbox: AABB
}

impl Sphere {
    pub fn new(config: &SphereConfig, material: usize) -> Sphere {

        let bbox = AABB::new(config.center - config.radius,
                             config.center + config.radius);

        Sphere { center: config.center,
                 radius: config.radius,
                 material_id: material,
                 bbox: bbox}
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        normalize(point - self.center)
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let a = sum(square(ray.direction));
        let v = ray.origin - self.center;
        let b = 2.0_f32 * sum(ray.direction * v);
        let c = sum(square(v)) - self.radius * self.radius;
        let discriminant = b * b - 4.0_f32 * a * c;

        if discriminant < 0.0_f32 {
            return None;
        }

        let mut t = (-b - discriminant) / 2.0_f32;

        if t < 0.0_f32 {
            t = (-b + discriminant) / 2.0_f32;
        }

        if t < 0.0_f32 {
            return None;
        }

        if in_range(range, t) {
            let hit_point = ray.point_at(t);
            let normal = self.normal_at(hit_point);

            return Some(Surfel{t, hit_point, normal, material_id: self.material_id});
        }

        None
    }
}

impl Triangle {

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let a = self.v0.x() - self.v1.x();
        let b = self.v0.x() - self.v2.x();
        let c = ray.direction.x();
        let d = self.v0.x() - ray.origin.x();

        let e = self.v0.y() - self.v1.y();
        let f = self.v0.y() - self.v2.y();
        let g = ray.direction.y();
        let h = self.v0.y() - ray.origin.y();

        let i = self.v0.z() - self.v1.z();
        let j = self.v0.z() - self.v2.z();
        let k = ray.direction.z();
        let l = self.v0.z() - ray.origin.z();

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

        let denom = 1.0 / determinant(&A);
        let beta = determinant(&B) * denom;

        if beta < 0.0 { return None; }

        let gamma = determinant(&Y) * denom;

        if gamma < 0.0 { return None; }

        if beta + gamma > 1.0 { return None; }

        let t = determinant(&T) * denom;

        if !in_range(range, t) {
            return None;
        }

        let hit_point = ray.point_at(t);
        let normal = Vec3::new(0.0, 0.0, 0.0); // TODO
        let material_id = 0;

        return Some(Surfel{t, hit_point, normal, material_id});
    }
}

impl IndexedTriangle {

    pub fn intersect(&self, ray: &Ray,
                     range: Range,
                     vertices: &Vec<Vec3>,
                     normals: &Vec<Vec3>) -> Option<Surfel> {
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

        if beta < 0.0_f32 { return None; }

        let gamma = determinant(&Y) * denom;

        if gamma < 0.0_f32 { return None; }

        if beta + gamma > 1.0_f32 { return None; }

        let t = determinant(&T) * denom;

        if !in_range(range, t) {
            return None;
        }

        let hit_point = ray.point_at(t);
        let alpha = (1.0_f32 - beta - gamma).max(0.0_f32);
        let normal = (alpha * normals[self.i]) +
                     (beta * normals[self.j]) +
                     (gamma * normals[self.k]);
        let material_id = 0;

        return Some(Surfel{t, hit_point, normal, material_id});
    }
}

impl Mesh {
    pub fn new(fpath: &String, dpath: &String) -> Mesh {
        let mut mesh = Mesh{ vertices: Vec::new(), triangles: Vec::new(), normals: Vec::new() };
        let path = Path::new(dpath).join(fpath);

        let file = match File::open(&path) {
            Err(why) => panic!("failed to open {}: {}", path.display(), why.to_string()),
            Ok(file) => file,
        };

        let lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(data) = line {
                if data.is_empty() || data.starts_with("#") {
                    continue;
                }
                let split = data.trim().split_whitespace();
                let vec: Vec<&str> = split.collect();
                if vec[0] == "v" {
                    let x = vec[1].parse::<f32>().unwrap();
                    let y = vec[2].parse::<f32>().unwrap();
                    let z = vec[3].parse::<f32>().unwrap();
                    mesh.vertices.push(Vec3::new(x, y, z));
                }
                else if vec[0] == "f" {
                    let i = vec[1].parse::<usize>().unwrap() - 1;
                    let j = vec[2].parse::<usize>().unwrap() - 1;
                    let k = vec[3].parse::<usize>().unwrap() - 1;
                    mesh.triangles.push(IndexedTriangle{ i, j, k });
                }
                else if vec[0] == "n" {
                    let a = vec[1].parse::<f32>().unwrap();
                    let b = vec[2].parse::<f32>().unwrap();
                    let c = vec[3].parse::<f32>().unwrap();
                    mesh.normals.push(Vec3::new(a, b, c));
                }
            }
        }

        mesh
    }
}

impl Transform {
    pub fn new(translate: Vec3, rotate: Vec3, scale: Vec3) -> Transform {
        Transform{ translate, rotate, scale }
    }

    pub fn mat4(&self) -> Mat4 {
        let t = Mat4::translate(&self.translate);
        //println!("t {:?}", t);
        let s = Mat4::scale(&self.scale);
        //println!("s {:?}", s);
        let rx = Mat4::rotate_x(Degree(self.rotate.x()));
        let ry = Mat4::rotate_y(Degree(self.rotate.y()));
        let rz = Mat4::rotate_z(Degree(self.rotate.z()));
        //println!("rx {:?}", rx);
        //println!("ry {:?}", ry);
        //println!("rz {:?}", rz);
        let r = &(&rx * &ry) * &rz;
        //println!("r {:?}", r);
        let m = &(&t * &r) * &s;
        m
    }
}

impl Model2 {
    pub fn new(mesh: Rc<Mesh>, material: usize, transform: &Transform) -> Model2 {
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
        Model2{ mesh, material, vertices, normals, bbox }
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

impl Model {
    pub fn init(&mut self, mesh: &Mesh) -> () {

        /*
        let t = Mat4::translate(&self.translate);
        let s = Mat4::scale(&self.scale);
        let rx = Mat4::rotate_x(Degree(self.rotate.x));
        let ry = Mat4::rotate_y(Degree(self.rotate.y));
        let rz = Mat4::rotate_z(Degree(self.rotate.z));
        let r = &(&rx * &ry) * &rz;
        let m = &(&t * &r) * &s;
        */

        for tri in &mesh.triangles {
            //let v0 = (&m * Vec4::from_vec3(mesh.vertices[tri.i], 1.0)).to_vec3();
            //let v1 = (&m * Vec4::from_vec3(mesh.vertices[tri.j], 1.0)).to_vec3();
            //let v2 = (&m * Vec4::from_vec3(mesh.vertices[tri.k], 1.0)).to_vec3();
            let v0 = mesh.vertices[tri.i];
            let v1 = mesh.vertices[tri.j];
            let v2 = mesh.vertices[tri.k];
            self.triangles.push(Triangle{ v0, v1, v2 });
        }
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {

        let mut t_range = range;
        let mut surfel = None;

        for tri in &self.triangles {
            if let Some(surf) = tri.intersect(ray, t_range) {
                t_range.max = surf.t;
                surfel = Some(Surfel{material_id: self.material_id, ..surf});
            }
        }

        surfel
    }

}
