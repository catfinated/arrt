use super::math::*;
use super::aabb::{AABB, BvhNode};
use super::objects::Surfel;

use std::cmp::Ordering;

pub struct BVH<T: BvhNode> {
    left: Option<Box<BVH<T>>>,
    right: Option<Box<BVH<T>>>,
    objects: Vec<T>,
    pub bbox: AABB,
}

impl<T: BvhNode> Default for BVH<T> {
    fn default() -> Self {
        Self{ left: None, right: None, objects: Vec::new(), bbox: AABB::maxmin() }
    }
}

impl<T: BvhNode> BVH<T> {

    pub fn new<S, B>(mut objects: Vec<T>, axis: usize, sort: &S, bbox: &B) -> BVH<T>
    where
        S: Fn(&T, &T, usize) -> Ordering,
        B: Fn(&[T]) -> AABB

    {
        objects.sort_unstable_by(|a, b| sort(a, b, axis));

        if objects.len() <= 1 {
            println!("added BVH leaf with {} objects", objects.len());
            objects.shrink_to_fit();
            let bbox = bbox(&objects[..]);
            BVH{ left: None, right: None, objects: objects, bbox: bbox }
        }
        else {
            let next_axis = (axis + 1) % 3;
            let mid = objects.len() / 2;
            let rhs = objects.split_off(mid);
            let left = Box::new(BVH::new(objects, next_axis, sort, bbox));
            let right = Box::new(BVH::new(rhs, next_axis, sort, bbox));
            let bbox = left.bbox.merge(&right.bbox);
            BVH{ left: Some(left), right: Some(right), objects: Vec::new(), bbox: bbox }
        }
    }

    pub fn centroid(&self) -> Vec3 {
        self.bbox.center()
    }

    /*
    pub fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        self.intersect_with(ray, range,
                            &|ray: &Ray, range: Range, obj: &T| -> Option<Surfel>
                            { obj.intersect(ray, range) })
    }
    */

    pub fn intersect<F>(&self, ray: &Ray, mut range: Range, f: &F) -> Option<Surfel>
    where
        F: Fn(&Ray, Range, &T) -> Option<Surfel>
    {

        let mut ret = None;

        if let Some(t) = self.bbox.intersect(ray, range) {
            range.min = t;

            if !self.objects.is_empty() {
                for object in self.objects.iter() {
                    if let Some(surf) = f(ray, range, object) /*object.intersect(ray, range)*/ {
                        range.max = surf.t;
                        ret = Some(surf);
                    }
                }
            }
            else {
                let maybe_l_surf = self.left.as_ref().map_or(None, |node| node.intersect(ray, range, f));
                let maybe_r_surf = self.right.as_ref().map_or(None, |node| node.intersect(ray, range, f));

                match (maybe_l_surf, maybe_r_surf) {
                    (Some(l_surf), Some(r_surf)) => {
                        if l_surf.t <= r_surf.t {
                            ret = Some(l_surf);
                        }
                        else {
                            ret = Some(r_surf);
                        }
                    },
                    (Some(surf), None) => { ret = Some(surf); }
                    (None, Some(surf)) => { ret = Some(surf); }
                    (None, None) => {}
                }
            }
        }

        ret
    }

}
