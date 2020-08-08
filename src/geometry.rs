use crate::vec3;
use crate::{hittable::HitRecord, hittable::Hittable, vec3::Point3};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {

    /// Create a HitRecord for a ray and a t that has been hit
    fn create_record(&self, ray: &Ray, hit_at_t: f64) -> HitRecord {
        let hit_point = ray.at(hit_at_t);
        let normal = (hit_point - self.center) / self.radius;
        HitRecord::new(&ray, hit_point.clone(), &normal, hit_at_t)
    }
}


impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        // Use quadratic formula for collisions
        let oc = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let hit_at_t = (-half_b - root) / a;

            // Return when (-root) is in range
            if hit_at_t < t_max && hit_at_t > t_min {
                return Some(self.create_record(r, hit_at_t))
            }

            // Return when (+root) is in range
            let hit_at_t = (-half_b + root) / a;
            if hit_at_t < t_max && hit_at_t > t_min {
                return Some(self.create_record(r, hit_at_t))
            }
        }

        None
    }
}
