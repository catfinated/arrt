use serde::{Deserialize, Serialize};

use crate::math::{dot, to_radians, Degree, Vec3};
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
            return 0.0_f32;
        }

        let n = std::f32::consts::PI / 2.0;
        let d = phi / r;
        let f = (n * d).cos();
        f.powf(self.sharpness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{normalize, Vec3};
    use crate::render::ColorRGB;

    fn downward_spot(angle_deg: f32) -> SpotLight {
        SpotLight {
            color: ColorRGB::white(),
            position: Vec3::new(0.0, 5.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
            angle: Degree(angle_deg),
            sharpness: 1.0,
        }
    }

    #[test]
    fn on_axis_intensity_is_one() {
        let light = downward_spot(45.0);
        // hit point directly below: direction_from → (0,5,0), normalized → (0,1,0)
        let at = normalize(light.direction_from(Vec3::new(0.0, 0.0, 0.0)));
        assert!((light.intensity_at(at) - 1.0).abs() < 1e-4);
    }

    #[test]
    fn outside_cone_is_zero() {
        let light = downward_spot(30.0);
        // hit point far off-axis: angle to axis >> 30°
        let at = normalize(light.direction_from(Vec3::new(10.0, 0.0, 0.0)));
        assert_eq!(light.intensity_at(at), 0.0);
    }

    #[test]
    fn inside_cone_is_positive() {
        let light = downward_spot(60.0);
        // hit point slightly off-axis: angle < 60°
        let at = normalize(light.direction_from(Vec3::new(1.0, 0.0, 0.0)));
        assert!(light.intensity_at(at) > 0.0);
    }
}
