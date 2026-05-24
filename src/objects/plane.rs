use serde::{Deserialize, Serialize};

use crate::math::{dot, in_range, Range, Ray, Vec3};

use super::aabb::Aabb;
use super::material::{MaterialID, Surfel};
use super::object::Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaneConfig {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: String,
}

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material_id: MaterialID,
}

impl Plane {
    pub fn new(config: &PlaneConfig, material_id: MaterialID) -> Self {
        Plane {
            point: config.point,
            normal: config.normal,
            material_id,
        }
    }
}

impl Object for Plane {
    fn bbox(&self) -> Option<Aabb> {
        None
    }

    fn centroid(&self) -> Vec3 {
        self.point
    }

    #[allow(clippy::similar_names)]
    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
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
                surf = Some(Surfel {
                    t,
                    hit_point,
                    normal,
                    material_id: self.material_id,
                    n_offset: 0.0_f32,
                });
            }
        }
        surf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Range, Ray, Vec3};

    fn xz_plane() -> Plane {
        let cfg = PlaneConfig {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: String::new(),
        };
        Plane::new(&cfg, MaterialID(0))
    }

    fn range() -> Range {
        Range {
            min: 0.001,
            max: f32::MAX,
        }
    }

    #[test]
    fn hit_from_above() {
        let ray = Ray {
            origin: Vec3::new(0.0, 5.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
            depth: 0,
        };
        assert!(xz_plane().intersect(&ray, range()).is_some());
    }

    #[test]
    fn hit_t_value() {
        let ray = Ray {
            origin: Vec3::new(0.0, 5.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
            depth: 0,
        };
        let t = xz_plane().intersect(&ray, range()).unwrap().t;
        assert!((t - 5.0).abs() < 1e-5);
    }

    #[test]
    fn miss_parallel_ray() {
        let ray = Ray {
            origin: Vec3::new(0.0, 1.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
            depth: 0,
        };
        assert!(xz_plane().intersect(&ray, range()).is_none());
    }

    #[test]
    fn miss_ray_points_away() {
        let ray = Ray {
            origin: Vec3::new(0.0, -1.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
            depth: 0,
        };
        assert!(xz_plane().intersect(&ray, range()).is_none());
    }

    #[test]
    fn normal_flips_for_back_face() {
        // ray from below: normal should point downward (toward the ray)
        let ray = Ray {
            origin: Vec3::new(0.0, -5.0, 0.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
            depth: 0,
        };
        let normal = xz_plane().intersect(&ray, range()).unwrap().normal;
        assert!(normal.y() < 0.0);
    }
}
