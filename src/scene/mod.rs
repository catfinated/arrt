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
pub use model::{Model, ModelInstance, Transform};
pub use objects::Object;
pub use material::{Material, MaterialID, MaterialMap, Surfel};

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
    objects: Vec<ObjectConfig>,
    #[serde(default)]
    lights: Vec<PointLight>,
}

pub struct Scene {
    config: SceneConfig,
    materials_map: MaterialMap,
}

impl Scene {
    pub fn new(fpath: &PathBuf) -> Scene {
        let mut mat_path = fpath.clone();
        mat_path.set_file_name("materials.yaml");
        let yaml = fs::read_to_string(fpath).unwrap();
        let config: SceneConfig = serde_yaml::from_str(&yaml).unwrap();
        let materials_map = MaterialMap::new(&mat_path);
        Scene{config, materials_map}
    }

    pub fn make_bvh(&self) -> BVH<Object>
    {
        let mut objs = Vec::new();
        let mesh_dir = &self.config.mesh_dir;
        let mut models = HashMap::new();

        for obj in &self.config.objects {
            match obj {
                ObjectConfig::Sphere(s) => {
                    objs.push(Object::Sphere(Sphere::new(s, self.materials_map.get_material_id(&s.material))));
                }
                ObjectConfig::Model(m) => {
                    let material_id = self.materials_map.get_material_id(&m.material);
                    let model: &Arc<Model> = models.entry(m.mesh.clone())
                    .or_insert_with(|| Arc::new(Model::new(&m.mesh, mesh_dir, material_id)));
                    objs.push(Object::ModelInstance(ModelInstance::new(model.clone(),
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

    pub fn material_for_surfel(&self, surfel: &Surfel) -> &Material {
        self.materials_map.get_material(surfel.material_id)
    }
}
