pub mod camera;
pub mod lights;
pub mod objects;
pub mod material;
pub mod mesh;
pub mod sphere;
pub mod model;

use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde;
use serde_yaml;
use serde::{Serialize, Deserialize};

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
pub use material::{Material, MaterialID, Surfel};

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
    materials_map: HashMap<String, MaterialID>,
}

impl Scene {
    pub fn new(fpath: &PathBuf) -> Scene {
        let yaml = fs::read_to_string(&fpath).unwrap();
        let config: SceneConfig = serde_yaml::from_str(&yaml).unwrap();

        let mut materials_map = HashMap::new();
        for (i, mat) in config.materials.iter().enumerate() {
            materials_map.insert(mat.name.clone(), MaterialID(i));
        }

        Scene{config, materials_map}
    }

    pub fn make_bvh(&self) -> BVH<Object>
    {
        let mut objs = Vec::new();
        let mesh_dir = &self.config.mesh_dir;
        let mut meshes = HashMap::new();

        for obj in &self.config.objects {
            match obj {
                ObjectConfig::Sphere(s) => {
                    objs.push(Object::Sphere(Sphere::new(s, self.materials_map[&s.material])));
                }
                ObjectConfig::Model(m) => {
                    let material_id = self.materials_map[&m.material];
                    let mesh: &Arc<Mesh> = meshes.entry(m.mesh.clone())
                        .or_insert_with(|| Arc::new(Mesh::new(&m.mesh, mesh_dir)));
                    objs.push(Object::Model(Model::new(Arc::clone(mesh),
                                                       material_id,
                                                       &m.transform)));
                }
            }
        }

        BVH::new(objs, 0)
    }

    pub fn make_camera(&self) -> Camera
    {
        Camera::new(&self.config.camera, self.width() as f32, self.height() as f32)
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

    pub fn ambient(&self) -> ColorRGB {
        self.config.ambient
    }

    pub fn bgcolor(&self) -> ColorRGB {
        self.config.bgcolor
    }

    pub fn material_for_surfel(&self, surfel: &Surfel) -> &Material
    {
        &self.config.materials[surfel.material_id.0]
    }
}
