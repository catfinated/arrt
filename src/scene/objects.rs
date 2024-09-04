use serde::{Serialize, Deserialize};

use crate::objects::mesh::MeshConfig;
use crate::objects::sphere::SphereConfig;
use crate::objects::plane::PlaneConfig;
use crate::objects::superquadric::SuperQuadricConfig;
use crate::objects::bpatch::BPatchConfig;

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Model(MeshConfig),
    Plane(PlaneConfig),
    SuperQuadric(SuperQuadricConfig),
    BPatch(BPatchConfig),
}

