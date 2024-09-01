use serde::{Serialize, Deserialize};

use crate::math::{dot, in_range, Range, Ray, Vec3};

use super::aabb::Aabb;
use super::material::{MaterialID, Surfel};
use super::object::Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaneConfig {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: String
}

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material_id: MaterialID,
}

impl Plane {
    pub fn new(config: &PlaneConfig, material_id: MaterialID) -> Self {
        Plane{point: config.point, normal: config.normal, material_id}
    }
}

impl Object for Plane {
    fn bbox(&self) -> Option<Aabb> {
        None
    }

    fn centroid(&self) -> Vec3 {
        self.point
    }

    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        //println("plane intersect!");
        let ndotrd = dot(self.normal, ray.direction);
        let mut surf = None;

        if ndotrd != 0.0_f32 {
            let ndotro = dot(self.normal, ray.origin);
            let ndotp = dot(self.normal, self.point);
            let t = -((ndotro - ndotp) / ndotrd);
            if in_range(range, t) {
                let hit_point = ray.point_at(t);
                let mut normal = self.normal; // todo
                if ndotrd > 0.0_f32 {
                    normal = -normal;
                }
                surf = Some(Surfel{t, hit_point, normal, material_id: self.material_id, n_offset: 0.0_f32});

            }
        }
        surf

    }
}