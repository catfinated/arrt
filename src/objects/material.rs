use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use serde;
use serde_yaml;
use serde::{Serialize, Deserialize};

use crate::render::ColorRGB;
use crate::math::Vec3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Material {
    pub name: String,
    pub ambient: ColorRGB,
    pub diffuse: ColorRGB,
    pub specular: ColorRGB,
    pub transmissive: ColorRGB,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub kr: f32,
    pub kt: f32,
    pub ior: f32,
    pub shininess: f32,
    pub highlight: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct MaterialID(pub usize);

pub struct MaterialMap {
    materials: Vec<Material>,
    name_to_id: HashMap<String, MaterialID>,
}

pub struct Surfel {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material_id: MaterialID,
    pub n_offset: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self { name: "".to_string(),
               ambient: ColorRGB::black(),
               diffuse: ColorRGB::black(),
               specular: ColorRGB::black(),
               transmissive: ColorRGB::black(),
               ka: 1.0_f32,
               kd: 1.0_f32,
               ks: 1.0_f32,
               kr: 0.0_f32,
               kt: 0.0_f32,
               ior: 0.0_f32,
               shininess: 1.0_f32,
               highlight: 0.0_f32, }
    }
}

impl MaterialMap {
    pub fn new(fpath: &PathBuf) -> MaterialMap {
        println!("loading materials from: {:#?}", fpath);
        let inf = File::open(fpath).unwrap();
        let materials: Vec<Material> =  serde_yaml::from_reader(&inf).unwrap();

        for mat in &materials {
            println!("{:?}", mat);
        }

        let mut name_to_id = HashMap::new();
        for (i, mat) in materials.iter().enumerate() {
            name_to_id.insert(mat.name.clone(), MaterialID(i));
        }
        MaterialMap{materials, name_to_id}
    }
     
    pub fn get_material_id(&self, name: &str) -> MaterialID {
        self.name_to_id[name]
    }

    pub fn get_material(&self, id: MaterialID) -> &Material {
        &self.materials[id.0]
    }
}

