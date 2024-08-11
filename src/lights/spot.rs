use serde::{Serialize, Deserialize};

use crate::math::{Vec3, Degree, dot, to_radians};
use crate::render::ColorRGB;

use super::Light;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotLight {
    pub color: ColorRGB,
    pub position: Vec3,
    pub direction: Vec3,
    pub angle: Degree,
    pub sharpness: f32,
}

impl Light for SpotLight {
    fn direction_from(&self, from: Vec3) -> Vec3 {
        self.position - from
    }

    fn diffuse(&self) -> ColorRGB {
        self.color
    }

    fn specular(&self) -> ColorRGB {
        self.color
    }

    fn intensity_at(&self, at: Vec3) -> f32 {

        let r = to_radians(self.angle).0;
        let phi = dot(-at, self.direction).acos();
        
        if phi > r {
            return 0.0_f32
        }

        let n = std::f32::consts::PI / 2.0;
        let d = phi / r;
        let f = (n * d).cos();
        f.powf(self.sharpness)
    }

}

