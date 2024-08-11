
use crate::math::Vec3;
use crate::render::ColorRGB;

pub trait Light: Send + Sync {
    fn direction_from(&self, from: Vec3) -> Vec3;
    fn intensity_at(&self, at: Vec3) -> f32;
    fn diffuse(&self) -> ColorRGB;
    fn specular(&self) -> ColorRGB;
}
