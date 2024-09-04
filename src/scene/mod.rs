pub mod camera;

mod objects;
mod lights;

use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Serialize, Deserialize};

use crate::render::ColorRGB;
use crate::objects::{bpatch, superquadric, Bvh, Material, MaterialMap, Mesh, Instance, Object, Plane, Sphere, Surfel};
use crate::lights::{Light, PointLight, SpotLight};

use camera::CameraConfig;
use objects::ObjectConfig;
use lights::LightsConfig;

pub use camera::Camera;

#[derive(Serialize, Deserialize)]
struct SceneConfig {
    bgcolor: ColorRGB,
    width: u32,
    height: u32,
    #[serde(default)]
    mesh_dir: String,
    #[serde(default)]
    patch_dir: String,    
    #[serde(default = "ColorRGB::white")]
    ambient: ColorRGB,
    camera: CameraConfig,
    #[serde(default)]
    objects: Vec<ObjectConfig>,
    #[serde(default)]
    lights: Vec<LightsConfig>,
}

pub struct Scene {
    config: SceneConfig,
    materials_map: MaterialMap,
    lights: Vec<Arc<dyn Light>>,
}

impl Scene {
    pub fn new(fpath: &PathBuf) -> Scene {
        let mut mat_path = fpath.clone();
        mat_path.set_file_name("materials.yaml");
        let yaml = fs::read_to_string(fpath).unwrap();
        let config: SceneConfig = serde_yaml::from_str(&yaml).unwrap();
        let materials_map = MaterialMap::new(&mat_path);
        let mut lights: Vec<Arc<dyn Light>> = Vec::new();

        for light in &config.lights {
            match light {
                LightsConfig::Point(pl) => {
                    lights.push(Arc::new(PointLight{..*pl}));
                }
                LightsConfig::Spot(sl) => {
                    lights.push(Arc::new(SpotLight{..*sl}));
                }
            }
        }

        Scene{config, materials_map, lights}
    }

    pub fn make_objects(&self) -> Vec<Arc<dyn Object>> {
        let mut all_objs: Vec<Arc<dyn Object>> = Vec::new();
        let mut bounded_objs: Vec<Arc<dyn Object>> = Vec::new();
        let mesh_dir = &self.config.mesh_dir;
        let patch_dir = &self.config.patch_dir;
        let mut meshes = HashMap::new();

        for obj in &self.config.objects {
            match obj {
                ObjectConfig::Sphere(s) => {
                    bounded_objs.push(Arc::new(Sphere::new(s, self.materials_map.get_material_id(&s.material))));
                }
                ObjectConfig::Model(m) => {
                    let material_id = self.materials_map.get_material_id(&m.material);
                    let mesh: &Arc<Mesh> = meshes.entry(m.mesh.clone())
                    .or_insert_with(|| Arc::new(Mesh::fromSMF(&m.mesh, mesh_dir)));
                    bounded_objs.push(Arc::new(Instance::new(mesh.clone(),
                                                       material_id,
                                                       &m.transform)));
                },
                ObjectConfig::Plane(p) => {
                    all_objs.push(Arc::new(Plane::new(p, self.materials_map.get_material_id(&p.material))));
                },
                ObjectConfig::SuperQuadric(sqc) => {
                    let material_id = self.materials_map.get_material_id(&sqc.material);
                    let se = Arc::new(superquadric::tessellate_superquadric(sqc));
                    bounded_objs.push(Arc::new(Instance::new(se, material_id, &sqc.transform)));
                },
                ObjectConfig::BPatch(bpc) => {
                    let material_id = self.materials_map.get_material_id(&bpc.material);
                    let bp = Arc::new(bpatch::tessellate_bpatch(patch_dir, bpc));
                    bounded_objs.push(Arc::new(Instance::new(bp, material_id, &bpc.transform)));
                }
            }
        }

        all_objs.push(Arc::new(Bvh::new(bounded_objs, 0)));
        println!("all objects {}", all_objs.len());
        all_objs
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

    pub fn lights(&self) -> &Vec<Arc<dyn Light>> {
        &self.lights
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
