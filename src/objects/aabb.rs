use crate::math::Vec4;
use crate::math::{Ray, Range, Vec3, Mat4};

use std::mem;
use std::f32;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

fn nearly_zero(f: f32) -> bool {
    let abs_diff = (f - 0.0_f32).abs();
    abs_diff <= (2.0_f32 * f32::EPSILON)
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb{ min, max }
    }

    pub fn zero() -> Aabb {
        Aabb::new(Vec3::zeros(), Vec3::zeros())
    }

    pub fn maxmin() -> Self {
        Aabb::new(Vec3::fill(f32::MAX), Vec3::fill(f32::MIN))
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

        Aabb{ min, max }
    }

    fn vertices(&self) -> [Vec3; 8] {
        [
            Vec3::new(self.min.x(), self.min.y(), self.min.z()),
            Vec3::new(self.max.x(), self.min.y(), self.min.z()),
            Vec3::new(self.min.x(), self.max.y(), self.min.z()),
            Vec3::new(self.min.x(), self.min.y(), self.max.z()),
            Vec3::new(self.min.x(), self.max.y(), self.max.z()),
            Vec3::new(self.max.x(), self.min.y(), self.max.z()),
            Vec3::new(self.max.x(), self.max.y(), self.min.z()),
            Vec3::new(self.max.x(), self.max.y(), self.max.z()),
        ]
    }

    /// Transform bounding box 
    pub fn transform(&self, m: &Mat4) -> Self {
        let mut mins = [f32::MAX, f32::MAX, f32::MAX];
        let mut maxs = [f32::MIN, f32::MIN, f32::MIN];

        // Convert to cube, transform cube, re-align to axes
        for vertex in self.vertices().iter() {
            let v = (m * Vec4::from_vec3(*vertex, 1.0_f32)).to_vec3();
            mins[0] = mins[0].min(v.x());
            mins[1] = mins[1].min(v.y());
            mins[2] = mins[2].min(v.z());

            maxs[0] = maxs[0].max(v.x());
            maxs[1] = maxs[1].max(v.y());
            maxs[2] = maxs[2].max(v.z());            
        }
        
        Aabb{ min: Vec3::new(mins[0], mins[1], mins[2]), 
          max: Vec3::new(maxs[0], maxs[1], maxs[2]) }
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

        Some(t_near)
    }
}
