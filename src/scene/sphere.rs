use serde::{Serialize, Deserialize};

use crate::math::*;
use crate::aabb::AABB;

use super::material::{Surfel, MaterialID};

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereConfig {
    pub center: Vec3,
    pub radius: f32,
    pub material: String
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material_id: MaterialID,
    pub bbox: AABB
}

impl Sphere {
    pub fn new(config: &SphereConfig, material: MaterialID) -> Sphere {

        let bbox = AABB::new(config.center - config.radius,
                             config.center + config.radius);

        Sphere { center: config.center,
                 radius: config.radius,
                 material_id: material,
                 bbox}
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        normalize(point - self.center)
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let a = sum(square(ray.direction));
        let v = ray.origin - self.center;
        let b = 2.0_f32 * sum(ray.direction * v);
        let c = sum(square(v)) - self.radius * self.radius;
        let discriminant = b * b - 4.0_f32 * a * c;

        if discriminant < 0.0_f32 {
            return None;
        }

        let mut t = (-b - discriminant) / 2.0_f32;

        if t < 0.0_f32 {
            t = (-b + discriminant) / 2.0_f32;
        }

        if t < 0.0_f32 {
            return None;
        }

        if in_range(range, t) {
            let hit_point = ray.point_at(t);
            let normal = self.normal_at(hit_point);

            return Some(Surfel{t, hit_point, normal, material_id: self.material_id});
        }

        None
    }
}
