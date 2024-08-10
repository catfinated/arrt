use serde::{Serialize, Deserialize};

use crate::objects::model::ModelConfig;
use crate::objects::sphere::SphereConfig;
use crate::objects::plane::PlaneConfig;

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Model(ModelConfig),
    Plane(PlaneConfig),
}

