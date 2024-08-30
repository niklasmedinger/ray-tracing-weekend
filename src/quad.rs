//! This module contains the code related to quadrilateral hittables (e.g., parallelograms and triangles)

use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    point::Point,
    vec3::{Unit3, Vec3},
};

#[derive(Debug, Clone)]
/// This struct implements a general quadrilateral.
pub struct Quad {
    /// The starting corner.
    q: Point,
    /// One vector from the corner.
    u: Vec3,
    /// The other vector from the corner.
    v: Vec3,
    /// The `w` component of the quad needed to orient points on its plane.
    w: Vec3,
    /// The surface material of the quad.
    material: Arc<dyn Material>,
    /// The bounding box of the quad.
    bounding_box: AABB,
    /// The normal vector of the plane this quad lies in.
    normal: Unit3,
    /// The solution to the equation `d = normal.dot(self.q)`.
    d: f32,
}

impl Quad {
    /// Create a new quad.
    pub fn new(q: Point, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let bounding_box = Self::compute_bounding_box(q, u, v);
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.as_vec3().dot(*q);
        let w = n / n.dot(n);
        Self {
            q,
            u,
            v,
            w,
            material,
            bounding_box,
            normal,
            d,
        }
    }

    fn compute_bounding_box(q: Point, u: Vec3, v: Vec3) -> AABB {
        let box_diagonal1 = AABB::from_points(q, q + u + v);
        let box_diagonal2 = AABB::from_points(q + u, q + v);
        AABB::from_aabbs(&box_diagonal1, &box_diagonal2)
    }

    /// Checks whether `a` and `b` are inside the parallelogram defined by
    /// this quad.
    fn is_interior(a: f32, b: f32) -> bool {
        let unit_interval = Interval::unit();
        unit_interval.contains(a) && unit_interval.contains(b)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.as_vec3().dot(*ray.direction());

        // No thit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.as_vec3().dot(**ray.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Self::is_interior(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            ray,
            intersection,
            self.normal,
            t,
            alpha,
            beta,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{
        color::Color, hittable::Hittable, interval::Interval, material::Lambertian, point::Point,
        ray::Ray, vec3::Vec3,
    };

    use super::Quad;

    #[test]
    fn hit_quad() {
        let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
        let q = Quad::new(
            Point::new(0.0, 0.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Arc::new(back_green),
        );
        let r = Ray::new(Point::new(1.0, 1.0, 9.0), Vec3::new(0.0, 0.0, -1.0), 0.0);
        let hit = q.hit(&r, Interval::universe());
        assert!(hit.is_some());
    }
}
