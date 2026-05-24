use crate::math::{Range, Ray, Vec3};

use super::aabb::Aabb;
use super::material::Surfel;

pub trait Object: Send + Sync {
    fn bbox(&self) -> Option<Aabb>;
    fn centroid(&self) -> Vec3;
    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel>;
}
