use serde::{Serialize, Deserialize};

use crate::aabb::AABB;
use crate::bvh::BvhNode;
use crate::math::*;

use super::material::Surfel;
use super::model::{Model, ModelConfig};
use super::sphere::{Sphere, SphereConfig};

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Model(ModelConfig)
}

pub enum Object {
    Sphere(Sphere),
    Model(Model)
}

impl BvhNode for Object {
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
                sphere.intersect(ray, range)
            }
            Object::Model(model) => {
                model.intersect(ray, range)
            }
        }
    }
}
