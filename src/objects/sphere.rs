use serde::{Deserialize, Serialize};

use crate::math::{dot, in_range, normalize, Range, Ray, Vec3};

use super::aabb::Aabb;
use super::material::{MaterialID, Surfel};
use super::object::Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereConfig {
    pub center: Vec3,
    pub radius: f32,
    pub material: String,
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material_id: MaterialID,
    pub bbox: Aabb,
}

impl Sphere {
    pub fn new(config: &SphereConfig, material: MaterialID) -> Sphere {
        let bbox = Aabb::new(config.center - config.radius, config.center + config.radius);

        Sphere {
            center: config.center,
            radius: config.radius,
            material_id: material,
            bbox,
        }
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        normalize(point - self.center)
    }
}

impl Object for Sphere {
    fn bbox(&self) -> Option<Aabb> {
        Some(self.bbox)
    }

    fn centroid(&self) -> Vec3 {
        self.center
    }

    #[allow(clippy::many_single_char_names)]
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

            return Some(Surfel {
                t,
                hit_point,
                normal,
                material_id: self.material_id,
                n_offset: 0.0001,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Range, Ray, Vec3};

    fn unit_sphere() -> Sphere {
        let cfg = SphereConfig {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: String::new(),
        };
        Sphere::new(&cfg, MaterialID(0))
    }

    fn range() -> Range {
        Range {
            min: 0.001,
            max: f32::MAX,
        }
    }

    #[test]
    fn hit_along_z_axis() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            depth: 0,
        };
        assert!(unit_sphere().intersect(&ray, range()).is_some());
    }

    #[test]
    fn hit_t_is_front_face() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            depth: 0,
        };
        let t = unit_sphere().intersect(&ray, range()).unwrap().t;
        assert!((t - 4.0).abs() < 1e-5); // front face at z=-1, origin at z=-5 => t=4
    }

    #[test]
    fn miss_ray_passes_beside() {
        let ray = Ray {
            origin: Vec3::new(2.0, 0.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            depth: 0,
        };
        assert!(unit_sphere().intersect(&ray, range()).is_none());
    }

    #[test]
    fn miss_ray_points_away() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            depth: 0,
        };
        assert!(unit_sphere().intersect(&ray, range()).is_none());
    }

    #[test]
    fn normal_points_outward() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            depth: 0,
        };
        let normal = unit_sphere().intersect(&ray, range()).unwrap().normal;
        assert!(normal.z() < 0.0); // front face normal points toward the ray
    }

    #[test]
    fn offset_sphere_hit() {
        let cfg = SphereConfig {
            center: Vec3::new(3.0, 0.0, 0.0),
            radius: 1.0,
            material: String::new(),
        };
        let sphere = Sphere::new(&cfg, MaterialID(0));
        let ray = Ray {
            origin: Vec3::new(3.0, 5.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
            depth: 0,
        };
        let t = sphere.intersect(&ray, range()).unwrap().t;
        assert!((t - 4.0).abs() < 1e-5);
    }
}
