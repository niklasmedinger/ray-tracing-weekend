use std::fmt::Debug;

use crate::{
    color::Color, hittable::HitRecord, random_0_1_f32, random_unit_vector, ray::Ray, reflect,
    refract,
};

/// A trait that defines behavior that structs which can act as the surface
/// material of objects in the world must implement.
///
/// We require that any implementor must also implement [Debug]. Yes, this is
/// not how you would normally write library code, but this library is only
/// consumed internally and we want everything to implement [Debug].
pub trait Material: Debug {
    // TODO: We currently always produce a scatter ray. Maybe return an Option
    // to allow for absorbing the ray?
    /// Compute the ray that is scattered away from the hit of the ray and
    /// its attenuation as a [Color].
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Ray, Color);
}

#[derive(Clone, Copy, Debug)]
/// A material that produces scatter rays according to the lambertian distribution.
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    /// Create a new lambertian material from its color.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: HitRecord) -> (Ray, Color) {
        let scatter_direction = hit_record.normal() + random_unit_vector();

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            hit_record.normal()
        } else {
            scatter_direction
        };

        let scattered = Ray::new(hit_record.p(), scatter_direction);
        (scattered, self.albedo)
    }
}

#[derive(Clone, Copy, Debug)]
/// A material that implements reflection by a metal material.
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    /// Create a new material with its color and a `fuzz` which randomizes
    /// the reflection. A bigger `fuzz` means more deviation from the true
    /// reflection.
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Ray, Color) {
        let reflected = reflect(*ray.direction(), hit_record.normal());
        let reflected = reflected.unit() + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(hit_record.p(), reflected);
        (scattered, self.albedo)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    /// Schlick approximation for reflectance.
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Ray, Color) {
        let ri = if hit_record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction().unit();
        let cos_theta = -unit_direction.dot(hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_0_1_f32() {
            reflect(unit_direction, hit_record.normal())
        } else {
            refract(unit_direction, hit_record.normal(), ri)
        };

        let scattered = Ray::new(hit_record.p(), direction);
        let attenuation = Color::white();
        (scattered, attenuation)
    }
}
