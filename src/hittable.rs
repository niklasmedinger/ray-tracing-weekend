//! This module defines a trait for [Hittable] objects. I.e., objects which can
//! be hit by a [Ray]. It also contains the implementations of our geometric
//! primitives which implement [Hittable].

use std::{fmt::Debug, sync::Arc};

use strum::IntoEnumIterator;

use crate::{
    aabb::AABB,
    degrees_to_radians,
    interval::Interval,
    material::Material,
    point::Point,
    ray::Ray,
    vec3::{Dimension, Unit3, Vec3},
    INFINITY, NEG_INFINITY, PI,
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
    u: f32,
    v: f32,
    front_face: bool,
}

impl HitRecord {
    /// Create a new [HitRecord].
    ///
    /// * `ray` - The [Ray] that hit a surface.
    /// * `p` - The [Point] of intersection.
    /// * `normal` - The surface normal vector. We assume this to have unit length!
    /// * `t` - The `t` such that `ray(t) = ray.origin() + t * ray.direction() = p`.
    pub fn new(
        ray: &Ray,
        p: Point,
        normal: Unit3,
        t: f32,
        u: f32,
        v: f32,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let (front_face, normal) = Self::face_normal(ray, normal);
        HitRecord {
            p,
            normal,
            material,
            t,
            u,
            v,
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
            u: self.u,
            v: self.v,
            front_face: self.front_face,
        }
    }

    #[inline]
    /// Return the [Point] `p` where the hit occured.
    pub fn p(&self) -> Point {
        self.p
    }

    #[inline]
    /// Return the vector normal to the surface that was hit.
    pub fn normal(&self) -> Unit3 {
        self.normal
    }

    #[inline]
    /// The `t` which solves `Ray(t) = p` for the [Ray] that hit the surface.
    /// Note that the hit record does not have a reference to this ray. Thus,
    /// the code that creates the record needs to keep the ray and record
    /// associated.
    pub fn t(&self) -> f32 {
        self.t
    }

    #[inline]
    /// Set a new `t`.
    pub fn set_t(&mut self, t: f32) {
        self.t = t;
    }

    #[inline]
    /// Returns true iff the the front face of the object was hit.
    pub fn front_face(&self) -> bool {
        self.front_face
    }

    #[inline]
    /// Return the material the surface is made of.
    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    #[inline]
    /// Return the texture coordinate `u`.
    pub fn u(&self) -> f32 {
        self.u
    }

    #[inline]
    /// Return the texture coordinate `v`.
    pub fn v(&self) -> f32 {
        self.v
    }

    fn face_normal(ray: &Ray, outward_normal: Unit3) -> (bool, Unit3) {
        // SAFETY: We assume that outward_normal has unit length.
        let front_face = ray.direction().dot(*outward_normal) < 0.0;
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

    fn get_sphere_uv(p: Point) -> (f32, f32) {
        let theta = f32::acos(-p.y());
        let phi = f32::atan2(-p.z(), p.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
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
        let (u, v) = Self::get_sphere_uv(Point::from(normal));
        // SAFETY: `normal` is the vector from the center of the sphere to the
        // point where the ray intersected the spheres surface. Thus, dividing
        // by the radius ensures it is of unit length.
        let normal = Unit3::new_unchecked(normal);
        Some(HitRecord::new(
            ray,
            p,
            normal,
            root,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

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

    /// Extend the world with objects from an iterator.
    pub fn extend(&mut self, other: impl IntoIterator<Item = Arc<dyn Hittable>>) {
        self.objects.extend(other)
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

#[derive(Debug, Clone)]
/// A new, translated instance of an object with a [Vec3] offset.
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bounding_box: AABB,
}

impl Translate {
    /// Create a new instance of an object with an offset.
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bounding_box = *object.bounding_box() + offset;
        Self {
            object,
            offset,
            bounding_box,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(*ray.origin() - self.offset, *ray.direction(), ray.time());
        self.object.hit(&offset_ray, ray_t).map(|mut hit_rec| {
            let new_p = hit_rec.p() + self.offset;
            hit_rec.p = new_p;
            hit_rec
        })
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

#[derive(Debug, Clone)]
/// A rotation along the Y-axis.
pub struct RotationY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    /// The bounding box of the rotated object.
    bounding_box: AABB,
}

impl RotationY {
    /// Create a new instance of `object` which is rotated by `angle` degrees
    /// along the Y-axis.
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);
        let bounding_box = object.bounding_box();

        let mut min_point = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max_point = Point::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f32 = i as f32;
                    let j_f32 = j as f32;
                    let k_f32 = k as f32;
                    let x = i_f32 * bounding_box.x().max() + (1.0 - i_f32) * bounding_box.x().min();
                    let y = j_f32 * bounding_box.y().max() + (1.0 - j_f32) * bounding_box.y().min();
                    let z = k_f32 * bounding_box.z().max() + (1.0 - k_f32) * bounding_box.z().min();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);
                    for dimension in Dimension::iter() {
                        min_point[dimension] = min_point[dimension].min(tester[dimension]);
                        max_point[dimension] = max_point[dimension].max(tester[dimension]);
                    }
                }
            }
        }
        let bounding_box = AABB::from_points(min_point, max_point);
        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotationY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let origin = Point::new(
            (self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().z()),
            ray.origin().y(),
            (self.sin_theta * ray.origin().x()) + (self.cos_theta * ray.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * ray.direction().x()) - (self.sin_theta * ray.direction().z()),
            ray.direction().y(),
            (self.sin_theta * ray.direction().x()) + (self.cos_theta * ray.direction().z()),
        );

        let rotated_ray = Ray::new(origin, direction, ray.time());

        self.object.hit(&rotated_ray, ray_t).map(|mut hit_rec| {
            hit_rec.p = Point::new(
                (self.cos_theta * hit_rec.p().x()) + (self.sin_theta * hit_rec.p().z()),
                hit_rec.p().y(),
                (-self.sin_theta * hit_rec.p().x()) + (self.cos_theta * hit_rec.p().z()),
            );
            hit_rec.normal = Unit3::new_unchecked(Vec3::new(
                (self.cos_theta * hit_rec.normal().x()) + (self.sin_theta * hit_rec.normal().z()),
                hit_rec.normal().y(),
                (-self.sin_theta * hit_rec.normal().x()) + (self.cos_theta * hit_rec.normal().z()),
            ));
            hit_rec
        })
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
