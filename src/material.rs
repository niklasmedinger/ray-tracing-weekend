//! This module defines a trait for [Material]s. [Materials] describe how a
//! [Ray] is refracted by a surface. The module also contains the implementations
//! for types that implement the material trait.

use std::{fmt::Debug, sync::Arc};

use crate::{
    color::Color,
    hittable::HitRecord,
    point::Point,
    random_0_1_f32, random_unit_vector,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: Vec3, n: Vec3, eta_i_over_eta_t: f32) -> Vec3 {
    let cos_theta = -uv.dot(n).min(1.0);
    let r_out_perp = eta_i_over_eta_t * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

/// A trait that defines behavior that structs which can act as the surface
/// material of objects in the world must implement.
///
/// We require that any implementor must also implement [Debug]. Yes, this is
/// not how you would normally write library code, but this library is only
/// consumed internally and we want everything to implement [Debug].
///
/// Implementing [Send] and [Sync] is required to concurrently render pixels.
pub trait Material: Debug + Send + Sync {
    /// Compute the ray that is scattered away from the hit of the ray and
    /// its attenuation as a [Color].
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Color)>;

    /// Returns the color of the light this material emits.
    fn emitted(&self, _u: f32, _v: f32, _p: Point) -> Color {
        Color::black()
    }
}

#[derive(Clone, Debug)]
/// A material that produces scatter rays according to the lambertian distribution.
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    /// Create a new lambertian material from its color.
    pub fn new(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a new lambertian material from a texture.
    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = *hit_record.normal() + *random_unit_vector();

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            *hit_record.normal()
        } else {
            scatter_direction
        };

        let scattered = Ray::new(hit_record.p(), scatter_direction, ray.time());
        let attenuation = self
            .texture
            .value(hit_record.u(), hit_record.v(), hit_record.p());
        Some((scattered, attenuation))
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
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(*ray.direction(), *hit_record.normal());
        let reflected = *reflected.unit() + (self.fuzz * *random_unit_vector());
        let scattered = Ray::new(hit_record.p(), reflected, ray.time());
        Some((scattered, self.albedo))
    }
}

#[derive(Clone, Copy, Debug)]
/// A material that implements reflection by a dieletric material.
pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    refraction_index: f32,
}

impl Dielectric {
    /// Create a new material with a given refraction index.
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
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Color)> {
        let ri = if hit_record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction().unit();
        let cos_theta = -unit_direction.dot(*hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_0_1_f32() {
            reflect(*unit_direction, *hit_record.normal())
        } else {
            refract(*unit_direction, *hit_record.normal(), ri)
        };

        let scattered = Ray::new(hit_record.p(), direction, ray.time());
        let attenuation = Color::white();
        Some((scattered, attenuation))
    }
}

#[derive(Debug, Clone)]
/// A struct that implements a source of diffuse light.
pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    /// Create a light source.
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit_record: HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Point) -> Color {
        self.texture.value(u, v, p)
    }
}

#[derive(Debug, Clone)]
/// An isotropic material which scatteres in a random direction.
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    /// Create a new material from a [Color].
    pub fn from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a new material from a [Texture].
    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new(hit_record.p(), *random_unit_vector(), ray.time());
        let attenuation = self
            .texture
            .value(hit_record.u(), hit_record.v(), hit_record.p());
        Some((scattered, attenuation))
    }
}
