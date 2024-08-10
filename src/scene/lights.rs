use serde::{Serialize, Deserialize};

use crate::render::ColorRGB;
use crate::math::*;

pub trait Light {
    fn direction_from(&self, from: Vec3) -> Vec3;
    fn intensity_at(&self, at: Vec3) -> f32;
    fn diffuse(&self) -> ColorRGB;
    fn specular(&self) -> ColorRGB;
}

#[derive(Serialize, Deserialize)]
pub struct PointLight {
    position: Vec3,
    ambient: ColorRGB,
    diffuse: ColorRGB,
    specular: ColorRGB,
}

impl PointLight {
    pub fn new(position: Vec3,
               ambient: ColorRGB,
               diffuse: ColorRGB,
               specular: ColorRGB) -> PointLight {
        PointLight{ position, ambient, diffuse, specular }
    }
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
