use serde::{Serialize, Deserialize};

use crate::math::{Ray, Range, Vec3, dot, normalize, in_range};

use super::material::{Surfel, MaterialID};
use super::object::Object;
use super::aabb::Aabb;

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
    pub bbox: Aabb
}

impl Sphere {
    pub fn new(config: &SphereConfig, material: MaterialID) -> Sphere {

        let bbox = Aabb::new(config.center - config.radius,
                             config.center + config.radius);

        Sphere { center: config.center,
                 radius: config.radius,
                 material_id: material,
                 bbox}
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        normalize(point - self.center)
    }
}

impl Object for Sphere {

    fn bbox(&self) -> Option<Aabb>
    {
        Some(self.bbox)
    }

    fn centroid(&self) -> Vec3
    {
        self.center
    }

    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let a = dot(ray.direction, ray.direction);
        let v = ray.origin - self.center;
        let b = 2.0_f32 * dot(ray.direction, v);
        let c = dot(v, v) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0_f32 * a * c);

        if discriminant < 0.0_f32 {
            return None;
        }

        let f = discriminant.sqrt();
        let mut t = (-b - f) / (2.0_f32 * a);

        if t < 0.0_f32 {
            t = (-b + f) / (2.0_f32 * a);
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
