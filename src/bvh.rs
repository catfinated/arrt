use crate::math::*;
use crate::aabb::AABB;
use crate::scene::Surfel;

use std::cmp::Ordering;

pub trait BvhNode {
    fn centroid(&self) -> Vec3;
    fn bbox(&self) -> AABB;
    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel>;
}

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
    pub fn new(mut objects: Vec<T>, axis: usize) -> BVH<T>
    {
        objects.sort_unstable_by(|a, b| centroid_cmp(a, b, axis));

        if objects.len() <= 1 {
            objects.shrink_to_fit();
            let bbox = compute_bbox(&objects[..]);
            println!("added BVH leaf with {} objects. bbox: {:?}", objects.len(), bbox);
            BVH{ left: None, right: None, objects, bbox }
        }
        else {
            let next_axis = (axis + 1) % 3;
            let mid = objects.len() / 2;
            let rhs = objects.split_off(mid);
            let left = Box::new(BVH::new(objects, next_axis));
            let right = Box::new(BVH::new(rhs, next_axis));
            let bbox = left.bbox.merge(&right.bbox);
            BVH{ left: Some(left), right: Some(right), objects: Vec::new(), bbox }
        }
    }

    pub fn centroid(&self) -> Vec3 {
        self.bbox.center()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Surfel> {
        let range = Range{ min: 1e-6, max: f32::MAX };
        self.intersect_with_range(ray, range)
    }

    pub fn intersect_with_range(&self, ray: &Ray, mut range: Range) -> Option<Surfel>
    {
        let mut ret = None;

        if let Some(t) = self.bbox.intersect(ray, range) {
            range.min = t;

            if !self.objects.is_empty() {
                for object in self.objects.iter() {
                    if let Some(surf) = object.intersect(ray, range) {
                        range.max = surf.t;
                        ret = Some(surf);
                    }
                }
            }
            else {
                let maybe_l_surf = self.left.as_ref().and_then(|node| node.intersect_with_range(ray, range));
                let maybe_r_surf = self.right.as_ref().and_then(|node| node.intersect_with_range(ray, range));

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

fn compute_bbox<T: BvhNode>(objects: &[T]) -> AABB {
    let mut bbox = AABB::maxmin();

    for object in objects {
        bbox = bbox.merge(&object.bbox());
    }

    bbox
}

fn centroid_cmp<T: BvhNode>(lhs: &T, rhs: &T, axis: usize) ->  Ordering {
    let lhs_centroid = lhs.centroid();
    let rhs_centroid = rhs.centroid();
    lhs_centroid[axis].partial_cmp(&rhs_centroid[axis]).unwrap()
}
