pub mod vec3;
pub mod vec4;
pub mod mat3;
pub mod mat4;
pub mod range;
pub mod ray;

pub use range::{Range, in_range};
pub use ray::Ray;
pub use vec3::{Vec3, normalize, cross, dot, reflect, refract};
pub use vec4::Vec4;
pub use mat3::{Mat3, determinant};
pub use mat4::Mat4;

use serde::{Serialize, Deserialize};

const PI_OVER_ONE_EIGHTY : f32 = std::f32::consts::PI / 180.0_f32;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Degree(pub f32);
pub struct Radian(pub f32);

pub fn to_radians(d: Degree) -> Radian {
    Radian(d.0 * PI_OVER_ONE_EIGHTY)
}

impl Radian {
    fn sin(&self) -> f32 { self.0.sin() }
    fn cos(&self) -> f32 { self.0.cos() }
    fn tan(&self) -> f32 { self.0.tan() }
}

impl Degree {
    pub fn sin(&self) -> f32 { to_radians(*self).sin() }
    pub fn cos(&self) -> f32 { to_radians(*self).cos() }
    pub fn tan(&self) -> f32 { to_radians(*self).tan() }
}
