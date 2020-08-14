use crate::vec3::{dot, Point3};
use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    /// Point where the ray hit
    pub p: Point3,
    /// Normal on the surface
    pub normal: Vec3,
    /// The t values located on the ray
    pub t: f64,
    /// If the ray is fromt facing
    pub front_face: bool,
    /// Material on the surface
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

fn calculate_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
    // See if the face is front facing
    let front_face = dot(&ray.dir, outward_normal) < 0.;
    // If it is not front-facing invert the normal so it points against the ray
    let normal = if front_face {
        outward_normal.clone()
    } else {
        -outward_normal.clone()
    };

    (front_face, normal)
}

impl<'a> HitRecord<'a> {
    pub fn new(
        ray: &Ray,
        hit_point: Point3,
        normal: &Vec3,
        t: f64,
        material: &'a dyn Material,
    ) -> Self {
        let (front_face, normal) = calculate_face_normal(&ray, &normal);
        HitRecord {
            p: hit_point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

impl Hittable for Vec<&dyn Hittable> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        self.iter().for_each(|hittable| {
            // Loop over all records and return the closest
            if let Some(hitted) = hittable.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hitted.t;
                hit_record = Some(hitted);
            }
        });
        hit_record
    }
}
