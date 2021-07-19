use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cmp::Ordering;

use serde_yaml;
use serde::{Serialize, Deserialize};

use super::math::*;
use super::objects::{Sphere, SphereConfig, Material, Model2, ModelConfig, Mesh, Surfel};
use super::framebuffer::ColorRGB;
use super::lights::PointLight;
use super::aabb::{AABB, BvhNode};
use super::bvh::BVH;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub eye: Vec3,
    pub up: Vec3,
    pub look_at: Vec3,
    pub dist: f32,
    pub fov: Degree
}

pub struct Camera {
    eye: Vec3,
    top_left: Vec3,
    xv: Vec3,
    yv: Vec3,
    sj: f32,
    sk: f32,
    hres: f32,
    vres: f32
}

impl Camera {
    pub fn new(config: CameraConfig, hres: f32, vres: f32) -> Camera {
        let zv = normalize(config.look_at - config.eye);
        let vup = normalize(config.up);
        // right handed coordinates
        let xv = normalize(cross(vup, zv));
        let yv = normalize(cross(zv, xv));

        let theta = Degree(config.fov.0 / 2.0_f32);
        let h = config.dist * theta.tan();
        let sj = 2.0_f32 * h;
        let sk = sj * (vres / hres);

        let top_left = config.eye + config.dist * zv + (sj / 2.0_f32) * xv + (sk / 2.0_f32) * yv;

        println!("{:?}", config.eye);
        println!("{:?}", zv);
        println!("{:?}", vup);
        println!("{:?}", xv);
        println!("{:?}", yv);
        println!("{}", sj);
        println!("{}", sk);
        println!("{:?}", top_left);
        println!("{}", config.fov.0);
        println!("{}", theta.0);
        println!("{}", h);
        println!("{}", config.dist);

        Camera {
            eye: config.eye,
            top_left,
            xv,
            yv,
            sj,
            sk,
            hres,
            vres
        }
    }

    pub fn ray_at(&self, j: u32, k: u32) -> Ray {
        let jf = j as f32;
        let kf = k as f32;

        let v = (self.top_left -
            self.sj * (jf / (self.hres - 1.0_f32)) * self.xv -
            self.sk * (kf / (self.vres - 1.0_f32)) * self.yv) -
            self.eye;

        Ray{origin: self.eye, direction: normalize(v)}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Model(ModelConfig)
}

pub enum Object {
    Sphere(Sphere),
    Model(Model2)
}

impl BvhNode for Object {

}

impl Object {

    fn centroid(&self) -> Vec3 {
        match self {
            Object::Sphere(sphere) => {
                sphere.center
            }
            Object::Model(model) => {
                model.centroid()
            }
        }
    }

    fn bbox(&self) -> AABB {
        match self {
            Object::Sphere(sphere) => {
                sphere.bbox
            }
            Object::Model(model) => {
                model.bbox
            }
        }
    }

    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        match self {
            Object::Sphere(sphere) => {
                sphere.intersect(&ray, range)
            }
            Object::Model(model) => {
                model.intersect(&ray, range)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub bgcolor: ColorRGB,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub mesh_dir: String,
    #[serde(default = "ColorRGB::white")]
    pub ambient: ColorRGB,
    pub camera: CameraConfig,
    #[serde(default)]
    pub materials: Vec<Material>,
    #[serde(default)]
    pub objects: Vec<ObjectConfig>,
    #[serde(default)]
    pub lights: Vec<PointLight>,
    #[serde(skip)]
    meshes: HashMap<String, Rc<Mesh>>,
    #[serde(skip)]
    pub bvh: BVH<Object>,
}

fn centroid_cmp(lhs: &Object, rhs: &Object, axis: usize) ->  Ordering {
    let lhs_centroid = lhs.centroid();
    let rhs_centroid = rhs.centroid();
    lhs_centroid[axis].partial_cmp(&rhs_centroid[axis]).unwrap()
}

fn compute_bbox(objects: &[Object]) -> AABB {
    let mut bbox = AABB::maxmin();

    for object in objects {
        bbox = bbox.merge(&object.bbox());
    }

    bbox
}

fn intersect(ray: &Ray, range: Range, obj: &Object) -> Option<Surfel> {
    obj.intersect(ray, range)
}

impl Scene {
    pub fn new(config: &Config) -> Scene {

        let yaml = fs::read_to_string(&config.filename).unwrap();
        let mut scene: Scene = serde_yaml::from_str(&yaml).unwrap();

        let mut map = HashMap::new();

        for (i, mat) in scene.materials.iter().enumerate() {
            map.insert(&mat.name, i);
        }

        let mut objs = Vec::new();
        let mesh_dir = &scene.mesh_dir;

        for obj in &mut scene.objects {
            match obj {
                ObjectConfig::Sphere(cfg) => {
                    objs.push(Object::Sphere(Sphere::new(cfg, map[&cfg.material])));
                }
                ObjectConfig::Model(cfg) => {
                    let material_id = map[&cfg.material];
                    let mesh: &Rc<Mesh> = scene.meshes.entry(cfg.mesh.clone())
                        .or_insert_with(|| Rc::new(Mesh::new(&cfg.mesh, mesh_dir)));
                    objs.push(Object::Model(Model2::new(Rc::clone(mesh),
                                                              material_id,
                                                              &cfg.transform)));
                }
            }
        }

        scene.bvh = BVH::new(objs, 0, &centroid_cmp, &compute_bbox);
        scene
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Surfel> {
        let range = Range{ min: 1e-6, max: std::f32::MAX };
        self.bvh.intersect(ray, range, &intersect)
    }
}
