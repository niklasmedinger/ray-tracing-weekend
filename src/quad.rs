//! This module contains the code related to quadrilateral hittables (e.g., parallelograms and triangles)

use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, World},
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
        let d = normal.dot(*q);
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

    /// Create a new `box` that contains the opposite vertices `a` and `b`.
    pub fn quad_box(a: Point, b: Point, material: Arc<dyn Material>) -> World {
        let mut sides = World::new();
        let min = Point::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.push(Arc::new(Quad::new(
            Point::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            material.clone(),
        )));

        sides.push(Arc::new(Quad::new(
            Point::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            material.clone(),
        )));

        sides.push(Arc::new(Quad::new(
            Point::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            material.clone(),
        )));

        sides.push(Arc::new(Quad::new(
            Point::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            material.clone(),
        )));

        sides.push(Arc::new(Quad::new(
            Point::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            material.clone(),
        )));

        sides.push(Arc::new(Quad::new(
            Point::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        )));

        sides
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(*ray.direction());

        // No thit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(**ray.origin())) / denom;
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
