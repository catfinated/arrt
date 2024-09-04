use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use serde;
use serde::{Serialize, Deserialize};

use crate::math::Vec3;
use super::aabb::Aabb;
use super::mesh::{compute_normals, Mesh, Triangle};
use super::transform::Transform;

#[derive(Debug, Serialize, Deserialize)]
pub struct BPatchConfig {
    pub fpath: String,
    pub material: String,
    pub slices: u32,
    pub flip_normals: bool,
    #[serde(default)]
    pub transform: Transform
}

#[derive(Debug)]
struct Patch {
    points: [Vec3; 16]
}

impl Default for Patch {
    fn default() -> Self {
        Patch{ points: [Vec3::zeros(); 16] }
    }
}

fn read_bpt(fpath: &Path) -> Vec<Patch> {
    println!("loading patch from: {:#?}", fpath);
    
    let file = match File::open(fpath) {
        Err(why) => panic!("failed to open {}: {}", fpath.display(), why),
        Ok(file) => file,
    };

    let mut patches = Vec::new();
    let mut lines = io::BufReader::new(file).lines();
    let num_patches = lines.next().unwrap().unwrap().parse::<u32>().unwrap();

    for _ in 0..num_patches {
        let mut line = lines.next().unwrap().unwrap();
        assert_eq!(line, "3 3");
        let mut patch = Patch::default();

        for j in 0..16 {
            line = lines.next().unwrap().unwrap();
            let vec: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(vec.len(), 3);
            patch.points[j].set_x(vec[0].parse::<f32>().unwrap());
            patch.points[j].set_y(vec[1].parse::<f32>().unwrap());
            patch.points[j].set_z(vec[2].parse::<f32>().unwrap());
        }

        patches.push(patch);
    }

    patches
}

fn blend(u: f32, points: &[Vec3]) -> Vec3
{
  let one_minus_u = 1.0_f32 - u;

  let b0 = one_minus_u.powf(3.0_f32);
  let b1 = (3.0_f32 * u) * one_minus_u.powf(2.0_f32);
  let b2 = (3.0_f32 * u.powf(2.0_f32)) * one_minus_u;
  let b3 = u.powf(3.0_f32);
  
  points[0] * b0 + points[1] * b1 + points[2] * b2 + points[3] * b3
}

fn interpolate(u: f32, v: f32, patch: &Patch) -> Vec3
{
    let mut curve = [Vec3::zeros(); 4];

    for (i, elem) in curve.iter_mut().enumerate() {
        *elem = blend(u, &patch.points[4*i..(4*i)+4]);
    }

    blend(v, &curve)
}


pub fn tessellate_bpatch(dpath: &String, config: &BPatchConfig) -> Mesh {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    let path = Path::new(dpath).join(&config.fpath);

    let mut box_min = Vec3::fill(f32::MAX);
    let mut box_max = Vec3::fill(f32::MIN);

    let patches = read_bpt(&path);

    for patch in &patches {
        let offset = vertices.len();
        for i in 0..=config.slices {
            let u = i as f32 / config.slices as f32;
            for j in 0..=config.slices {
                let v = j as f32 / config.slices as f32;
                let point = interpolate(u, v, patch);
                vertices.push(point);

                box_min.set_x(point.x().min(box_min.x()));
                box_min.set_y(point.y().min(box_min.y()));
                box_min.set_z(point.z().min(box_min.z()));
    
                box_max.set_x(point.x().max(box_max.x()));
                box_max.set_y(point.y().max(box_max.y()));
                box_max.set_z(point.z().max(box_max.z()));
            }
        }

        let s = config.slices + 1;

        for i in 0..config.slices {
            for j in 0..config.slices {
                    // todo: might have top/bottom reversed here
                    let i0 = offset + (i * s + j) as usize; // bottom left
                    let i1 = offset + (i * s + (j + 1)) as usize; // bottom right
                    let i2 = offset + ((i + 1) * s + (j + 1)) as usize; // top right
                    let i3 = offset + ((i + 1) * s + j) as usize; // top left

                    triangles.push(Triangle{ i: i0, j: i1, k: i2 });
                    triangles.push(Triangle{ i: i2, j: i3, k: i0 });
            }
        }
    }

    let bbox = Aabb::new(box_min, box_max);
    let normals = compute_normals(&vertices, &triangles, config.flip_normals);
    println!("bpatch bbox: {:?}", bbox);
    Mesh{ vertices, normals, triangles, bbox }
}