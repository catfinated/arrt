#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::math::*;
use super::material::{Surfel, MaterialID};

pub struct Triangle {
    pub i: usize,
    pub j: usize,
    pub k: usize
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub normals: Vec<Vec3>,
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

impl Mesh {
    pub fn new(fpath: &String, dpath: &String) -> Mesh {
        let mut mesh = Mesh{ vertices: Vec::new(), triangles: Vec::new(), normals: Vec::new() };
        let path = Path::new(dpath).join(fpath);
        println!("loading model mesh from: {:#?}", path);

        let file = match File::open(&path) {
            Err(why) => panic!("failed to open {}: {}", path.display(), why),
            Ok(file) => file,
        };

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
                    mesh.vertices.push(Vec3::new(x, y, z));
                }
                else if vec[0] == "f" {
                    let i = vec[1].parse::<usize>().unwrap() - 1;
                    let j = vec[2].parse::<usize>().unwrap() - 1;
                    let k = vec[3].parse::<usize>().unwrap() - 1;
                    mesh.triangles.push(Triangle{ i, j, k });
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
