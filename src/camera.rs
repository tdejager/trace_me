use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// A simple camera class
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        vup: &Vec3,
        vfov_degrees: f64,
        aspect_ratio: f64,
    ) -> Self {
        // Calculate lengths based on fov
        let theta = degrees_to_radians(vfov_degrees);
        let h = (theta / 2.).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        // Calculate basis vectors
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate camera parameters
        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin: *look_from,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - w,
        }
    }

    /// Cast a ray with the camera
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
