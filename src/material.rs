use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

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

        // Offset the reflection by a fuzziness factor
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        // Only reflect from the outside
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

pub struct Dialectric {
    refraction_index: f64,
}

impl Dialectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

/// Schlicks approximation for making the reflectivity vary with the angle
fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Material for Dialectric {
    fn scatter(&self, ray_incoming: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_incoming.dir.unit_vector();
        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Reflect if this is the case
        // Because the refractive index where the ray intersects
        // is higher than where the ray is coming from so it cannot refract
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(&hit_record.normal);
            return Some(MaterialInfo {
                attenuation,
                scattered: Ray::new(hit_record.p, reflected),
            });
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        let mut rnd = rand::thread_rng();
        let random: f64 = rnd.gen();
        if random < reflect_prob {
            let reflected = unit_direction.reflect(&hit_record.normal);
            return Some(MaterialInfo {
                attenuation,
                scattered: Ray::new(hit_record.p, reflected),
            });
        }
        // In this case it can refract
        let refracted = unit_direction.refract(&hit_record.normal, etai_over_etat);
        Some(MaterialInfo {
            attenuation,
            scattered: Ray::new(hit_record.p, refracted),
        })
    }
}
