//! This module defines a trait for [Hittable] objects. I.e., objects which can
//! be hit by a [Ray]. It also contains the implementations of our geometric
//! primitives which implement [Hittable].

use std::{fmt::Debug, sync::Arc};

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    point::Point,
    ray::Ray,
    vec3::{Unit3, Vec3},
};

#[derive(Clone, Debug)]
/// A hit record contains information about where the [Ray] hit the surface,
/// the normal vector from the surface, the [Material] of the surface, how far
/// the ray travelled from its origin to hit the surface, and whether the ray
/// hit the front face of the object.
pub struct HitRecord {
    p: Point,
    normal: Unit3,
    material: Arc<dyn Material>,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    /// Create a new [HitRecord].
    ///
    /// * `ray` - The [Ray] that hit a surface.
    /// * `p` - The [Point] of intersection.
    /// * `normal` - The surface normal vector. We assume this to have unit length!
    /// * `t` - The `t` such that `ray(t) = ray.origin() = + t * ray.direction() = p`.
    pub fn new(
        ray: &Ray,
        p: Point,
        normal: Unit3,
        t: f32,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let (front_face, normal) = Self::face_normal(ray, normal);
        HitRecord {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }

    /// Copy the record. Note that [HitRecord] cannot implement copy because
    /// [Arc] is not [Copy]. We implement this method, in addition to
    /// deriving [Clone], to make it explicit that this type is _cheap_ to copy.
    pub fn copy(&self) -> Self {
        Self {
            p: self.p,
            normal: self.normal,
            material: Arc::clone(&self.material),
            t: self.t,
            front_face: self.front_face,
        }
    }

    /// Return the [Point] `p` where the hit occured.
    pub fn p(&self) -> Point {
        self.p
    }

    /// Return the vector normal to the surface that was hit.
    pub fn normal(&self) -> Unit3 {
        self.normal
    }

    /// The `t` which solves `Ray(t) = p` for the [Ray] that hit the surface.
    /// Note that the hit record does not have a reference to this ray. Thus,
    /// the code that creates the record needs to keep the ray and record
    /// associated.
    pub fn t(&self) -> f32 {
        self.t
    }

    /// Returns true iff the the front face of the object was hit.
    pub fn front_face(&self) -> bool {
        self.front_face
    }

    /// Return the material the surface is made of.
    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    fn face_normal(ray: &Ray, outward_normal: Unit3) -> (bool, Unit3) {
        // SAFETY: We assume that outward_normal has unit length.
        let front_face = ray.direction().dot(outward_normal.as_vec3()) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

/// A trait that defines the behavior objects that can be 'hit' by a [Ray]
/// must implement.
///
/// We require that any implementor must also implement [Debug]. Yes, this is
/// not how you would normally write library code, but this library is only
/// consumed internally and we want everything to implement [Debug].
///
/// Implementing [Send] and [Sync] is required to concurrently render pixels.
pub trait Hittable: Debug + Send + Sync {
    /// Compute whether `ray` hits the `self` in [Interval] `ray_t`.
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    /// Return a reference to the bounding box of the hittable.
    fn bounding_box(&self) -> &AABB;
}

#[derive(Clone, Debug)]
/// A struct that implements a sphere in the world
pub struct Sphere {
    center: Point,
    center_vec: Option<Vec3>,
    radius: f32,
    material: Arc<dyn Material>,
    bounding_box: AABB,
}

impl Sphere {
    /// Create a new sphere
    ///
    /// * `center` - The point where the sphere is centered.
    /// * `radius` - The radius from the sphere's center to its surface.
    pub fn new(center: Point, radius: f32, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bounding_box = AABB::from_points(center - rvec, center + rvec);
        Sphere {
            center,
            center_vec: None,
            radius,
            material,
            bounding_box,
        }
    }

    /// Create a new sphere
    ///
    /// * `center` - The point where the sphere is centered.
    /// * `radius` - The radius from the sphere's center to its surface.
    pub fn new_moving(
        center: Point,
        radius: f32,
        material: Arc<dyn Material>,
        moves_to: Point,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bounding_box1 = AABB::from_points(center - rvec, center + rvec);
        let bounding_box2 = AABB::from_points(moves_to - rvec, moves_to + rvec);
        let bounding_box = AABB::from_aabbs(&bounding_box1, &bounding_box2);
        Sphere {
            center,
            center_vec: Some(moves_to - center),
            radius,
            material,
            bounding_box,
        }
    }

    /// Copy the sphere. Note that [Sphere] cannot implement copy because
    /// [Arc] is not [Copy]. We implement this method, in addition to
    /// deriving [Clone], to make it explicit that this type is _cheap_ to copy.
    pub fn copy(&self) -> Self {
        Self {
            center: self.center,
            radius: self.radius,
            material: Arc::clone(&self.material),
            center_vec: self.center_vec,
            bounding_box: self.bounding_box,
        }
    }

    fn sphere_center(&self, at_time: f32) -> Point {
        // Linearly interpolate from center to moves_to according to time, where t=0 yields
        // center1, and t=1 yields center2.
        self.center_vec
            .map_or(self.center, |v| self.center + at_time * v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center = self.sphere_center(ray.time());
        let oc = center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        // SAFETY: `normal` is the vector from the center of the sphere to the
        // point where the ray intersected the spheres surface. Thus, dividing
        // by the radius ensures it is of unit length.
        let normal = Unit3::new_unchecked(normal);
        Some(HitRecord::new(ray, p, normal, root, self.material.clone()))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

// #[derive(Clone)]
// pub struct Triangle {
//     a: Point,
//     b: Point,
//     c: Point,
//     material: Rc<dyn Material>,
// }

// impl Triangle {
//     pub fn new(a: Point, b: Point, c: Point) -> Self {
//         Self {
//             a,
//             b,
//             c,
//             material: todo!(),
//         }
//     }
// }

// impl Hittable for Triangle {
//     fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
//         let e1 = self.b - self.a;
//         let e2 = self.c - self.a;
//         let n = e1.cross(e2);
//         let det = -ray.direction().dot(n);
//         let inv_det = 1.0 / det;
//         let a_o = *ray.origin() - self.a;
//         let d_a_o = a_o.cross(*ray.direction());
//         let u = e2.dot(d_a_o) * inv_det;
//         let v = -e1.dot(d_a_o) * inv_det;
//         let t = a_o.dot(n) * inv_det;
//         let p = *ray.origin() + (t * *ray.direction());
//         if ray_t.surrounds(p.as_vec3().length())
//             && det >= (1.0 / 10.0_f32.powi(10))
//             && t >= 0.0
//             && u >= 0.0
//             && v >= 0.0
//             && (u + v) <= 1.0
//         {
//             Some(HitRecord::new(
//                 ray,
//                 p,
//                 n / n.length(),
//                 t,
//                 self.material.clone(),
//             ))
//         } else {
//             None
//         }
//     }
// }

#[derive(Default, Debug)]
/// A thing wrapper around a [Vec] of [Hittable]s.
pub struct World {
    /// The objects in the world.
    objects: Vec<Arc<dyn Hittable>>,
    /// The bounding box for this world.
    bounding_box: AABB,
}

impl World {
    /// Create a new world.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bounding_box: AABB::default(),
        }
    }

    /// Add a new object to the world.
    pub fn push(&mut self, object: Arc<dyn Hittable>) {
        self.bounding_box = AABB::from_aabbs(&self.bounding_box, object.bounding_box());
        self.objects.push(object)
    }

    /// Consume this world and return its objects.
    pub fn into_objects(self) -> Vec<Arc<dyn Hittable>> {
        self.objects
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max();
        let mut result = None;
        for hittable in self.objects.iter() {
            let interval = Interval::new(ray_t.min(), closest_so_far);
            if let Some(hit_record) = hittable.hit(ray, interval) {
                closest_so_far = hit_record.t();
                result = Some(hit_record);
            }
        }
        result
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{interval::Interval, point::Point, ray::Ray, vec3::Vec3, INFINITY};

//     use super::{Hittable, Triangle};

// #[test]
// fn hit_triangle() {
//     let a = Point::new(-1.0, 0.0, -1.0);
//     let b = Point::new(1.0, 0.0, -1.0);
//     let c = Point::new(0.0, 1.0, -1.0);
//     let t = Triangle::new(a, b, c);
//     let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, -1.0));
//     let i = Interval::new(0.0, INFINITY);
//     let record = t.hit(&ray, i);
//     assert!(
//         record.is_some(),
//         "Ray r: {:?} did not hit triangle: {:?}",
//         ray,
//         t
//     );
//     let record = record.unwrap();
//     assert_eq!(record.p(), c, "expected {:?}, got {:?}", c, record.p());

//     assert!(
//         record.front_face(),
//         "Front face was not set for ray {:?} and triangle {:?}",
//         ray,
//         t,
//     );

//     let expected_surface_normal = Vec3::new(0.0, 0.0, 1.0);
//     assert_eq!(
//         record.normal(),
//         expected_surface_normal,
//         "Expected surface normal {}, got {:?}",
//         expected_surface_normal,
//         record.normal()
//     );

//     let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
//     let record = t.hit(&ray, i);
//     assert!(
//         record.is_none(),
//         "Ray r: {:?} did hit triangle: {:?}",
//         ray,
//         t
//     );
// }
// }
