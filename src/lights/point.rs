use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;
    use crate::render::ColorRGB;

    fn white_point_light() -> PointLight {
        PointLight {
            position: Vec3::new(0.0, 5.0, 0.0),
            ambient: ColorRGB::black(),
            diffuse: ColorRGB::white(),
            specular: ColorRGB::white(),
        }
    }

    #[test]
    fn direction_from_is_position_minus_from() {
        let light = white_point_light();
        let d = light.direction_from(Vec3::new(0.0, 0.0, 0.0));
        assert!((d.x() - 0.0).abs() < 1e-5);
        assert!((d.y() - 5.0).abs() < 1e-5);
        assert!((d.z() - 0.0).abs() < 1e-5);
    }

    #[test]
    fn intensity_is_always_one() {
        let light = white_point_light();
        assert_eq!(light.intensity_at(Vec3::new(1.0, 0.0, 0.0)), 1.0);
    }

    #[test]
    fn diffuse_returns_configured_color() {
        let light = white_point_light();
        let d = light.diffuse();
        assert!((d.r - 1.0).abs() < 1e-5);
        assert!((d.g - 1.0).abs() < 1e-5);
        assert!((d.b - 1.0).abs() < 1e-5);
    }
}
