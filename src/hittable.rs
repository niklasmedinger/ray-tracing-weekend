use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{interval::Interval, material::Material, point::Point, ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct HitRecord {
    p: Point,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point, normal: Vec3, t: f32, material: Rc<dyn Material>) -> HitRecord {
        let (front_face, normal) = Self::face_normal(ray, &normal);
        HitRecord {
            p,
            normal,
            material,
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

    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
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

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
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
        Some(HitRecord::new(&ray, p, normal, root, self.material.clone()))
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

pub struct World(Vec<Box<dyn Hittable>>);

impl World {
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
