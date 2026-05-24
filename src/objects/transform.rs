use serde::{Deserialize, Serialize};

use crate::math::{Degree, Mat4, Vec3};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Vec3,
    pub scale: Vec3,
}

impl Transform {
    /// Create combined transformation matrix
    pub fn mat4(&self) -> Mat4 {
        let t = Mat4::translate(&self.translate);
        let s = Mat4::scale(&self.scale);
        let rx = Mat4::rotate_x(Degree(self.rotate.x()));
        let ry = Mat4::rotate_y(Degree(self.rotate.y()));
        let rz = Mat4::rotate_z(Degree(self.rotate.z()));
        let r = &(&rx * &ry) * &rz;
        &(&t * &r) * &s
    }

    /// Create inverse transformation matrix
    pub fn inverse(&self) -> Mat4 {
        let t = Mat4::itranslate(&self.translate);
        let s = Mat4::iscale(&self.scale);
        let rx = Mat4::irotate_x(Degree(self.rotate.x()));
        let ry = Mat4::irotate_y(Degree(self.rotate.y()));
        let rz = Mat4::irotate_z(Degree(self.rotate.z()));
        let r = &(&rz * &ry) * &rx;
        &(&s * &r) * &t
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translate: Vec3::zeros(),
            rotate: Vec3::zeros(),
            scale: Vec3::ones(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Vec3, Vec4};

    fn apply(t: &Transform, v: Vec3) -> Vec3 {
        (&t.mat4() * Vec4::from_vec3(v, 1.0)).to_vec3()
    }

    #[test]
    fn default_transform_is_identity() {
        let t = Transform::default();
        let result = apply(&t, Vec3::new(1.0, 2.0, 3.0));
        assert!((result.x() - 1.0).abs() < 1e-4);
        assert!((result.y() - 2.0).abs() < 1e-4);
        assert!((result.z() - 3.0).abs() < 1e-4);
    }

    #[test]
    fn translate_moves_point() {
        let t = Transform {
            translate: Vec3::new(1.0, 2.0, 3.0),
            ..Transform::default()
        };
        let result = apply(&t, Vec3::zeros());
        assert!((result.x() - 1.0).abs() < 1e-4);
        assert!((result.y() - 2.0).abs() < 1e-4);
        assert!((result.z() - 3.0).abs() < 1e-4);
    }

    #[test]
    fn scale_stretches_vector() {
        let t = Transform {
            scale: Vec3::new(2.0, 3.0, 4.0),
            ..Transform::default()
        };
        let result = apply(&t, Vec3::new(1.0, 1.0, 1.0));
        assert!((result.x() - 2.0).abs() < 1e-4);
        assert!((result.y() - 3.0).abs() < 1e-4);
        assert!((result.z() - 4.0).abs() < 1e-4);
    }
}
