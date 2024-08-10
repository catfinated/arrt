use crate::math::{Vec3, Ray, Range};

use super::aabb::AABB;
use super::material::Surfel;

pub trait Object: Send + Sync {
    fn bbox(&self) -> Option<AABB>;
    fn centroid(&self) -> Vec3;
    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel>;
}