use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct MaterialInfo {
    /// Definees how much the ray should be attenuated
    pub attenuation: Color,
    /// Defines the scattered ray
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_incoming: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo>;
}

/// A diffuse surface
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_incoming: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        Some(MaterialInfo {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.p, scatter_direction),
        })
    }
}

/// A Metal surface
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_incoming: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let reflected = ray_incoming.dir.unit_vector().reflect(&hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.dir.dot(&hit_record.normal) > 0. {
            Some(MaterialInfo {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

