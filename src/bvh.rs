//! This module contains the code for the BVH nodes

use std::sync::Arc;

use crate::{aabb::AABB, hittable::Hittable, interval::Interval};

#[derive(Clone, Debug)]
/// A BVH node with its bounding box, and left and right children.
pub struct BVHNode {
    /// The bounding box for this node.
    bounding_box: AABB,
    /// The left child.
    left: Arc<dyn Hittable>,
    /// The right child.
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    /// Create a new [BVHNode] from a slice of hittables. The slice needs to be
    /// mutable since this constructor sorts the objects in the slice.
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        let mut bounding_box = AABB::empty();
        for object in objects.iter() {
            bounding_box = AABB::from_aabbs(&bounding_box, object.bounding_box());
        }
        let dimension = bounding_box.longest_axis();

        match objects {
            [v] => {
                left = v.clone();
                right = v.clone();
            }
            [x, y] => {
                left = x.clone();
                right = y.clone();
            }
            x => {
                x.sort_by(|a, b| a.bounding_box().box_compare(b.bounding_box(), dimension));
                let mid = x.len() / 2;
                let (lower, upper) = x.split_at_mut(mid);
                left = Arc::new(Self::new(lower));
                right = Arc::new(Self::new(upper));
            }
        }
        Self {
            bounding_box,
            left,
            right,
        }
    }

    /// Create a new node from a [World].
    pub fn from_objects(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self::new(&mut objects)
    }

    /// Copy the node. Note that [BVHNode] cannot implement copy because
    /// [Arc] is not [Copy]. We implement this method, in addition to
    /// deriving [Clone], to make it explicit that this type is _cheap_ to copy.
    pub fn copy(&self) -> Self {
        Self {
            bounding_box: self.bounding_box,
            left: Arc::clone(&self.left),
            right: Arc::clone(&self.right),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<crate::hittable::HitRecord> {
        let hit = self.bounding_box.hit(ray, ray_t);
        hit.map(|ray_t| {
            let hit_left = self.left.hit(ray, ray_t);
            let t1 = if let Some(rec) = &hit_left {
                rec.t()
            } else {
                ray_t.max()
            };
            let interval = Interval::new(ray_t.min(), t1);
            let hit_right = self.right.hit(ray, interval);
            hit_right.or(hit_left)
        })?
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
