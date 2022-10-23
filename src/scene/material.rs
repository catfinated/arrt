use serde;
use serde::{Serialize, Deserialize};

use crate::framebuffer::ColorRGB;
use crate::math::Vec3;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Material {
    pub name: String,
    pub ambient: ColorRGB,
    pub diffuse: ColorRGB,
    pub specular: ColorRGB,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub shininess: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct MaterialID(pub usize);

pub struct Surfel {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub (super) material_id: MaterialID,
}

impl Default for Material {
    fn default() -> Self {
        Self { name: "".to_string(),
               ambient: ColorRGB::black(),
               diffuse: ColorRGB::black(),
               specular: ColorRGB::black(),
               ka: 1.0_f32,
               kd: 1.0_f32,
               ks: 1.0_f32,
               shininess: 1.0_f32 }
    }
}


