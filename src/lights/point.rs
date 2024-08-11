use serde::{Serialize, Deserialize};

use super::Light;
use crate::math::Vec3;
use crate::render::ColorRGB;

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLight {
    pub position: Vec3,
    pub ambient: ColorRGB,
    pub diffuse: ColorRGB,
    pub specular: ColorRGB,
}

impl Light for PointLight {
    fn direction_from(&self, from: Vec3) -> Vec3 {
        self.position - from
    }

    fn intensity_at(&self, _at: Vec3) -> f32 {
        1.0_f32
    }

    fn diffuse(&self) -> ColorRGB {
        self.diffuse
    }

    fn specular(&self) -> ColorRGB {
        self.specular
    }
}
