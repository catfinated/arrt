#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::math::*;
use super::Surfel;

#[derive(Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3
}

pub struct IndexedTriangle {
    pub i: usize,
    pub j: usize,
    pub k: usize
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<IndexedTriangle>,
    pub normals: Vec<Vec3>,
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

        Some(Surfel{t, hit_point, normal, material_id})
    }
}

impl IndexedTriangle {

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

        Some(Surfel{t, hit_point, normal, material_id})
    }
}

impl Mesh {
    pub fn new(fpath: &String, dpath: &String) -> Mesh {
        let mut mesh = Mesh{ vertices: Vec::new(), triangles: Vec::new(), normals: Vec::new() };
        let path = Path::new(dpath).join(fpath);

        let file = match File::open(&path) {
            Err(why) => panic!("failed to open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let split = line.split_whitespace();
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

        mesh
    }
}
