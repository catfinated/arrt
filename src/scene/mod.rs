pub mod camera;
pub mod lights;
pub mod objects;
pub mod material;
pub mod mesh;
pub mod sphere;
pub mod model;

use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::PathBuf;

use serde;
use serde_yaml;
use serde::{Serialize, Deserialize};

use crate::math::*;
use crate::bvh::BVH;
use crate::framebuffer::ColorRGB;

use camera::CameraConfig;
use lights::PointLight;
use objects::ObjectConfig;

pub use camera::Camera;
pub use lights::Light;
pub use sphere::Sphere;
pub use model::Model;
pub use objects::Object;
pub use material::{Material, Surfel};

use mesh::Mesh;

#[derive(Serialize, Deserialize)]
struct SceneConfig {
    bgcolor: ColorRGB,
    width: u32,
    height: u32,
    #[serde(default)]
    mesh_dir: String,
    #[serde(default = "ColorRGB::white")]
    ambient: ColorRGB,
    camera: CameraConfig,
    #[serde(default)]
    materials: Vec<Material>,
    #[serde(default)]
    objects: Vec<ObjectConfig>,
    #[serde(default)]
    lights: Vec<PointLight>,
}

pub struct Scene {
    config: SceneConfig,
    pub bvh: BVH<Object>,
}

impl Scene {
    pub fn new(fpath: &PathBuf) -> Scene {
        let yaml = fs::read_to_string(&fpath).unwrap();
        let config: SceneConfig = serde_yaml::from_str(&yaml).unwrap();

        let mut map = HashMap::new();
        for (i, mat) in config.materials.iter().enumerate() {
            map.insert(&mat.name, i);
        }

        let mut objs = Vec::new();
        let mesh_dir = &config.mesh_dir;
        let mut meshes = HashMap::new();

        for obj in &config.objects {
            match obj {
                ObjectConfig::Sphere(s) => {
                    objs.push(Object::Sphere(Sphere::new(s, map[&s.material])));
                }
                ObjectConfig::Model(m) => {
                    let material_id = map[&m.material];
                    let mesh: &Rc<Mesh> = meshes.entry(m.mesh.clone())
                        .or_insert_with(|| Rc::new(Mesh::new(&m.mesh, mesh_dir)));
                    objs.push(Object::Model(Model::new(Rc::clone(mesh),
                                                       material_id,
                                                       &m.transform)));
                }
            }
        }

        let bvh = BVH::new(objs, 0);
        Scene{config, bvh}
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Surfel> {
        let range = Range{ min: 1e-6, max: std::f32::MAX };
        self.bvh.intersect(ray, range)
    }

    pub fn width(&self) -> u32 {
        self.config.width
    }

    pub fn height(&self) -> u32 {
        self.config.height
    }

    pub fn lights(&self) -> &Vec<PointLight> {
        &self.config.lights
    }

    pub fn camera(&self) -> &CameraConfig {
        &self.config.camera
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.config.materials
    }

    pub fn ambient(&self) -> ColorRGB {
        self.config.ambient
    }

    pub fn bgcolor(&self) -> ColorRGB {
        self.config.bgcolor
    }


}
