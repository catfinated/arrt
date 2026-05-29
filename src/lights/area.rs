use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::math::{Vec3, Vec4};
use crate::objects::transform::Transform;
use crate::render::ColorRGB;

use super::Light;

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaLightConfig {
    #[serde(default)]
    pub transform: Transform,
    #[serde(default = "default_samples")]
    pub samples: u32,
    #[serde(default = "ColorRGB::white")]
    pub color: ColorRGB,
}

fn default_samples() -> u32 {
    25
}

pub struct AreaLight {
    /// Bottom-left corner of the rectangle in world space.
    corner: Vec3,
    /// World-space vector spanning the u-axis of the rectangle.
    edge_u: Vec3,
    /// World-space vector spanning the v-axis of the rectangle.
    edge_v: Vec3,
    /// Grid side length; total samples = n * n.
    n: u32,
    color: ColorRGB,
}

impl AreaLight {
    pub fn new(config: &AreaLightConfig) -> Self {
        let m = config.transform.mat4();

        // Unit XZ-plane rectangle before transform:
        //   corner (-0.5, 0, -0.5), edge_u = (1,0,0), edge_v = (0,0,1)
        let corner = (&m * Vec4::from_vec3(Vec3::new(-0.5, 0.0, -0.5), 1.0)).to_vec3();
        let edge_u = (&m * Vec4::from_vec3(Vec3::new(1.0, 0.0, 0.0), 0.0)).to_vec3();
        let edge_v = (&m * Vec4::from_vec3(Vec3::new(0.0, 0.0, 1.0), 0.0)).to_vec3();

        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = (config.samples as f32).sqrt() as u32;
        assert!(n * n == config.samples, "area light samples must be a perfect square");

        AreaLight {
            corner,
            edge_u,
            edge_v,
            n,
            color: config.color,
        }
    }

    fn center(&self) -> Vec3 {
        self.corner + 0.5 * self.edge_u + 0.5 * self.edge_v
    }
}

impl Light for AreaLight {
    fn direction_from(&self, from: Vec3) -> Vec3 {
        self.center() - from
    }

    fn intensity_at(&self, _at: Vec3) -> f32 {
        1.0
    }

    fn diffuse(&self) -> ColorRGB {
        self.color
    }

    fn specular(&self) -> ColorRGB {
        self.color
    }

    #[allow(clippy::cast_precision_loss)]
    fn sample_directions_from(&self, from: Vec3) -> Vec<Vec3> {
        let mut rng = rand::thread_rng();
        let n = self.n as usize;
        let nf = self.n as f32;
        let mut dirs = Vec::with_capacity(n * n);

        for i in 0..n {
            for j in 0..n {
                let u = (i as f32 + rng.gen::<f32>()) / nf;
                let v = (j as f32 + rng.gen::<f32>()) / nf;
                let point = self.corner + u * self.edge_u + v * self.edge_v;
                dirs.push(point - from);
            }
        }

        dirs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;
    use crate::objects::transform::Transform;
    use crate::render::ColorRGB;

    fn unit_light() -> AreaLight {
        AreaLight::new(&AreaLightConfig {
            transform: Transform::default(),
            samples: 25,
            color: ColorRGB::white(),
        })
    }

    #[test]
    fn intensity_is_always_one() {
        assert_eq!(unit_light().intensity_at(Vec3::new(1.0, 0.0, 0.0)), 1.0);
    }

    #[test]
    fn color_returned_for_diffuse_and_specular() {
        let light = AreaLight::new(&AreaLightConfig {
            transform: Transform::default(),
            samples: 25,
            color: ColorRGB::new(0.5, 0.3, 0.1),
        });
        let d = light.diffuse();
        assert!((d.r - 0.5).abs() < 1e-5);
        assert!((d.g - 0.3).abs() < 1e-5);
        assert!((d.b - 0.1).abs() < 1e-5);
        assert!((light.specular().r - 0.5).abs() < 1e-5);
    }

    #[test]
    fn direction_from_points_to_center() {
        // Identity transform: rectangle lies in the XZ plane, center at origin.
        let dir = unit_light().direction_from(Vec3::new(0.0, -5.0, 0.0));
        assert!((dir.x() - 0.0).abs() < 1e-5);
        assert!((dir.y() - 5.0).abs() < 1e-5);
        assert!((dir.z() - 0.0).abs() < 1e-5);
    }

    #[test]
    fn translate_moves_center() {
        let light = AreaLight::new(&AreaLightConfig {
            transform: Transform {
                translate: Vec3::new(0.0, 3.0, 0.0),
                ..Transform::default()
            },
            samples: 25,
            color: ColorRGB::white(),
        });
        // center moves to (0,3,0); direction from origin should be (0,3,0)
        let dir = light.direction_from(Vec3::zeros());
        assert!((dir.x() - 0.0).abs() < 1e-4);
        assert!((dir.y() - 3.0).abs() < 1e-4);
        assert!((dir.z() - 0.0).abs() < 1e-4);
    }

    #[test]
    fn sample_count_matches_config() {
        assert_eq!(unit_light().sample_directions_from(Vec3::zeros()).len(), 25);
    }

    #[test]
    fn larger_sample_count() {
        let light = AreaLight::new(&AreaLightConfig {
            transform: Transform::default(),
            samples: 100,
            color: ColorRGB::white(),
        });
        assert_eq!(light.sample_directions_from(Vec3::zeros()).len(), 100);
    }

    #[test]
    fn samples_land_on_rectangle_surface() {
        // Identity transform: rectangle is the XZ plane at y=0.
        // From a point below (y=-5), every sample direction endpoint must have y=0.
        let from = Vec3::new(0.0, -5.0, 0.0);
        for dir in unit_light().sample_directions_from(from) {
            let endpoint_y = from.y() + dir.y();
            assert!(
                endpoint_y.abs() < 1e-4,
                "expected endpoint y=0, got {endpoint_y}"
            );
        }
    }

    #[test]
    fn samples_stay_within_rectangle_bounds() {
        // Identity transform: rectangle spans x ∈ [-0.5, 0.5], z ∈ [-0.5, 0.5].
        let from = Vec3::new(0.0, -5.0, 0.0);
        for dir in unit_light().sample_directions_from(from) {
            let x = from.x() + dir.x();
            let z = from.z() + dir.z();
            assert!(x >= -0.5 && x <= 0.5, "sample x={x} out of [-0.5, 0.5]");
            assert!(z >= -0.5 && z <= 0.5, "sample z={z} out of [-0.5, 0.5]");
        }
    }
}
