use crate::{color::Color, hittable::HitRecord, random_unit_vector, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Ray, Color);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
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

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Ray, Color) {
        let reflected = crate::reflect(*ray.direction(), hit_record.normal());
        let scattered = Ray::new(hit_record.p(), reflected);
        (scattered, self.albedo)
    }
}
