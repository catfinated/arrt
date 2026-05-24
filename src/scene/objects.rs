use serde::{Deserialize, Serialize};

use crate::objects::bpatch::BPatchConfig;
use crate::objects::mesh::MeshConfig;
use crate::objects::plane::PlaneConfig;
use crate::objects::sphere::SphereConfig;
use crate::objects::superquadric::SuperQuadricConfig;

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectConfig {
    Sphere(SphereConfig),
    Model(MeshConfig),
    Plane(PlaneConfig),
    SuperQuadric(SuperQuadricConfig),
    BPatch(BPatchConfig),
}
