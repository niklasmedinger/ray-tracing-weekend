//! This module defines a trait for [Hittable] objects. I.e., objects which can
//! be hit by a [Ray]. It also contains the implementations of our geometric
//! primitives which implement [Hittable].

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{interval::Interval, material::Material, point::Point, ray::Ray, vec3::Unit3};

#[derive(Clone, Debug)]
/// A hit record contains information about where the [Ray] hit the surface,
/// the normal vector from the surface, the [Material] of the surface, how far
/// the ray travelled from its origin to hit the surface, and whether the ray
/// hit the front face of the object.
pub struct HitRecord {
    p: Point,
    normal: Unit3,
    material: Rc<dyn Material>,
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
        material: Rc<dyn Material>,
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
    /// [Rc] is not [Copy]. We implement this method, in addition to
    /// deriving [Clone], to make it explicit that this type is _cheap_ to copy.
    pub fn copy(&self) -> Self {
        Self {
            p: self.p,
            normal: self.normal,
            material: Rc::clone(&self.material),
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
pub trait Hittable: Debug {
    /// Compute whether `ray` hit the `self` in [Interval] `ray_t`.
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Clone, Debug)]
/// A struct that implements a sphere in the world
pub struct Sphere {
    center: Point,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    /// Create a new sphere
    ///
    /// * `center` - The point where the sphere is centered.
    /// * `radius` - The radius from the sphere's center to its surface.
    pub fn new(center: Point, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Copy the sphere. Note that [Sphere] cannot implement copy because
    /// [Rc] is not [Copy]. We implement this method, in addition to
    /// deriving [Clone], to make it explicit that this type is _cheap_ to copy.
    pub fn copy(&self) -> Self {
        Self {
            center: self.center,
            radius: self.radius,
            material: Rc::clone(&self.material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - *ray.origin();
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
pub struct World(Vec<Box<dyn Hittable>>);

impl World {
    /// Create a new world.
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for World {
    type Target = Vec<Box<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max();
        let mut result = None;
        for hittable in self.iter() {
            let interval = Interval::new(ray_t.min(), closest_so_far);
            if let Some(hit_record) = hittable.hit(ray, interval) {
                closest_so_far = hit_record.t();
                result = Some(hit_record);
            }
        }
        result
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
