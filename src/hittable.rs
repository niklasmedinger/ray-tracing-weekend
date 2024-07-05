use std::ops::{Deref, DerefMut};

use crate::{ray::Ray, Interval, Point, Vec3};

pub struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point, normal: Vec3, t: f32) -> HitRecord {
        let (front_face, normal) = Self::face_normal(ray, &normal);
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn p(&self) -> Point {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    fn face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        // SAFETY: We assume that outward_normal has unit length.
        let front_face = ray.direction().dot(*outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -*outward_normal
        };
        (front_face, normal)
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = *self.center - **ray.origin();
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
        let normal = (*p - *self.center) / self.radius;
        Some(HitRecord::new(&ray, p, normal, root))
    }
}

pub struct World<'a>(Vec<&'a dyn Hittable>);

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<'a> Deref for World<'a> {
    type Target = Vec<&'a dyn Hittable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for World<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Hittable for World<'a> {
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
