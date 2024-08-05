use serde::{Serialize, Deserialize};

use crate::aabb::AABB;
use crate::bvh::BvhNode;
use crate::math::*;

use super::material::Surfel;
use super::model::{ModelConfig, ModelInstance};
use super::sphere::{Sphere, SphereConfig};

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Model(ModelConfig)
}

pub enum Object {
    Sphere(Sphere),
    ModelInstance(ModelInstance)
}

impl BvhNode for Object {
    fn centroid(&self) -> Vec3 {
        match self {
            Object::Sphere(sphere) => {
                sphere.center
            }
            Object::ModelInstance(instance) => {
                instance.centroid()
            }
        }
    }

    fn bbox(&self) -> AABB {
        match self {
            Object::Sphere(sphere) => {
                sphere.bbox
            }
            Object::ModelInstance(instance) => {
                instance.bbox
            }
        }
    }
    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        match self {
            Object::Sphere(sphere) => {
                sphere.intersect(ray, range)
            }
            Object::ModelInstance(instance) => {
                instance.intersect(ray, range)
            }
        }
    }
}
