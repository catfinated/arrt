use crate::math::{Range, Ray, Vec3};

use std::cmp::Ordering;
use std::sync::Arc;

use super::aabb::Aabb;
use super::material::Surfel;
use super::object::Object;

pub struct Bvh {
    left: Option<Arc<dyn Object>>,
    right: Option<Arc<dyn Object>>,
    objects: Vec<Arc<dyn Object>>,
    pub bbox: Aabb,
}

impl Default for Bvh {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            objects: Vec::new(),
            bbox: Aabb::maxmin(),
        }
    }
}

impl Bvh {
    pub fn new(mut objects: Vec<Arc<dyn Object>>, axis: usize) -> Self {
        objects.sort_unstable_by(|a, b| centroid_cmp(&**a, &**b, axis));

        if objects.len() <= 1 {
            objects.shrink_to_fit();
            let bbox = compute_bbox(&objects);
            println!(
                "added BVH leaf with {} objects. bbox: {:?}",
                objects.len(),
                bbox
            );
            Bvh {
                left: None,
                right: None,
                objects,
                bbox,
            }
        } else {
            let next_axis = (axis + 1) % 3;
            let mid = objects.len() / 2;
            let rhs = objects.split_off(mid);
            let left = Arc::new(Bvh::new(objects, next_axis));
            let right = Arc::new(Bvh::new(rhs, next_axis));
            let bbox = left.bbox.merge(&right.bbox);
            Bvh {
                left: Some(left),
                right: Some(right),
                objects: Vec::new(),
                bbox,
            }
        }
    }
}

impl Object for Bvh {
    fn bbox(&self) -> Option<Aabb> {
        Some(self.bbox)
    }

    fn centroid(&self) -> Vec3 {
        self.bbox.center()
    }

    #[allow(clippy::similar_names)]
    fn intersect(&self, ray: &Ray, range: Range) -> Option<Surfel> {
        let mut ret = None;
        let mut trange = range;

        if self.bbox.intersect(ray, range).is_some() {
            if self.objects.is_empty() {
                let maybe_l_surf = self
                    .left
                    .as_ref()
                    .and_then(|node| node.intersect(ray, trange));
                let maybe_r_surf = self
                    .right
                    .as_ref()
                    .and_then(|node| node.intersect(ray, trange));

                match (maybe_l_surf, maybe_r_surf) {
                    (Some(l_surf), Some(r_surf)) => {
                        if l_surf.t <= r_surf.t {
                            ret = Some(l_surf);
                        } else {
                            ret = Some(r_surf);
                        }
                    }
                    (Some(surf), None) | (None, Some(surf)) => {
                        ret = Some(surf);
                    }
                    (None, None) => {}
                }
            } else {
                for object in &self.objects {
                    if let Some(surf) = object.intersect(ray, trange) {
                        trange.max = surf.t;
                        ret = Some(surf);
                    }
                }
            }
        }

        ret
    }
}

fn compute_bbox(objects: &Vec<Arc<dyn Object>>) -> Aabb {
    let mut bbox = Aabb::maxmin();

    for object in objects {
        bbox = bbox.merge(&object.bbox().unwrap());
    }

    bbox
}

fn centroid_cmp(lhs: &dyn Object, rhs: &dyn Object, axis: usize) -> Ordering {
    let lhs_centroid = lhs.centroid();
    let rhs_centroid = rhs.centroid();
    lhs_centroid[axis].partial_cmp(&rhs_centroid[axis]).unwrap()
}
