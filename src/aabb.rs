use super::math::{Ray, Range, Vec3};

use std::mem;
use std::f32;


#[derive(Copy, Clone, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

pub trait BvhNode {
    //fn centroid(&self) -> Vec3;

    //fn bbox(&self) -> AABB;

    //fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel>;
}

fn nearly_zero(f: f32) -> bool {
    let abs_diff = (f - 0.0_f32).abs();
    abs_diff <= (2.0_f32 * f32::EPSILON)
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB{ min, max }
    }

    pub fn zero() -> AABB {
        AABB::new(Vec3::zeros(), Vec3::zeros())
    }

    pub fn maxmin() -> AABB {
        AABB::new(Vec3::fill(f32::MAX), Vec3::fill(f32::MIN))
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.0_f32
    }

    pub fn merge(&self, other: &Self) -> Self {
        let min = Vec3::new(self.min.x().min(other.min.x()),
                            self.min.y().min(other.min.y()),
                            self.min.z().min(other.min.z()));

        let max = Vec3::new(self.max.x().max(other.max.x()),
                            self.max.y().max(other.max.y()),
                            self.max.z().max(other.max.z()));

        AABB{ min, max }
    }

    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<f32> {

        let mut t_near = range.min;
        let mut t_far = range.max;

        for i in 0..3 { // for each axis
            let d = ray.direction[i];
            let o = ray.origin[i];

            if nearly_zero(d) { // parallel to axis
                if o < self.min[i] || o > self.max[i] {
                    // box is intersected by the plane
                    // defined AXIS = o
                    return None;
                }
                else {
                    // never going to hit the plane of this
                    // axis with this ray.
                    continue;
                }
            }

            let f = 1.0_f32 / d;
            let mut t1 = (self.min[i] - o) * f;
            let mut t2 = (self.max[i] - o) * f;

            if t1 > t2 {
                mem::swap(&mut t1, &mut t2);
            }

            t_near = t1.max(t_near);
            t_far = t2.min(t_far);

            if t_near > t_far {
                return None;
            }

            if t_far < 0.0_f32 {
                return None;
            }
        }

        if t_near < 0.0_f32 {
            return Some(t_far);
        }

        return Some(t_near);
    }
}
