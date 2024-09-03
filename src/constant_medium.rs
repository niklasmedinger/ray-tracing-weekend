//! This module contains the code related to hittables that behave like a
//! constant medium, i.e., a medium where the probability of a ray
//! getting scattered inside of the medium is constant for every small distance
//! the ray travels inside of the medium.

use std::sync::Arc;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Isotropic, Material},
    random_0_1_f32,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::{Unit3, Vec3},
    INFINITY,
};

#[derive(Debug, Clone)]
/// A medium with constant density. It fills the space inside a boundary, which
/// is another hittable.
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    /// Create a new constant medium from a `boundary`, a `density`, and
    /// a `texture`.
    ///
    /// * `boundary` - The hittable which encloses this medium.
    /// * `density` - The density of this medium. A higher density means a higher
    ///   chance for rays to scatter.
    /// * `texture` - The texture of the medium.
    pub fn from_texture(
        boundary: Arc<dyn Hittable>,
        density: f32,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_texture(texture)),
        }
    }

    /// Create a new constant medium from a `boundary`, a `density`, and
    /// an `albedo`.
    ///
    /// * `boundary` - The hittable which encloses this medium.
    /// * `density` - The density of this medium. A higher density means a higher
    ///   chance for rays to scatter.
    /// * `albedo` - The color of the medium.
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f32, albedo: Color) -> Self {
        Self::from_texture(boundary, density, Arc::new(SolidColor::new(albedo)))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_rec1 = self.boundary.hit(ray, Interval::universe())?;
        let mut hit_rec2 = self
            .boundary
            .hit(ray, Interval::new(hit_rec1.t() + 0.0001, INFINITY))?;

        if hit_rec1.t() < ray_t.min() {
            hit_rec1.set_t(ray_t.min());
        }
        if hit_rec2.t() > ray_t.max() {
            hit_rec2.set_t(ray_t.max());
        }

        if hit_rec1.t() >= hit_rec2.t() {
            return None;
        }

        if hit_rec1.t() < 0.0 {
            hit_rec1.set_t(0.0);
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (hit_rec2.t() - hit_rec1.t()) * ray_length;
        let hit_distance = self.neg_inv_density * random_0_1_f32().log2();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit_rec1.t() + hit_distance / ray_length;
        let p = ray.at(t);

        let normal = Unit3::new_unchecked(Vec3::new(1.0, 0.0, 0.0));
        // normal, u, and v are arbitrary.
        let hit_record = HitRecord::new(ray, p, normal, t, 0.0, 0.0, self.phase_function.clone());

        Some(hit_record)
    }

    fn bounding_box(&self) -> &crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
