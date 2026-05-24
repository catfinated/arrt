pub mod mat3;
pub mod mat4;
pub mod range;
pub mod ray;
pub mod vec3;
pub mod vec4;

pub use mat3::{determinant, Mat3};
pub use mat4::Mat4;
pub use range::{in_range, Range};
pub use ray::Ray;
pub use vec3::{cross, dot, normalize, reflect, refract, Vec3};
pub use vec4::Vec4;

use serde::{Deserialize, Serialize};

const PI_OVER_ONE_EIGHTY: f32 = std::f32::consts::PI / 180.0_f32;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Degree(pub f32);
pub struct Radian(pub f32);

pub fn to_radians(d: Degree) -> Radian {
    Radian(d.0 * PI_OVER_ONE_EIGHTY)
}

impl Radian {
    fn sin(&self) -> f32 {
        self.0.sin()
    }
    fn cos(&self) -> f32 {
        self.0.cos()
    }
    fn tan(&self) -> f32 {
        self.0.tan()
    }
}

impl Degree {
    pub fn sin(self) -> f32 {
        to_radians(self).sin()
    }
    pub fn cos(self) -> f32 {
        to_radians(self).cos()
    }
    pub fn tan(self) -> f32 {
        to_radians(self).tan()
    }
}
