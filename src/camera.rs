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

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let origin = Vec3::zero();
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - Vec3::new(0., 0., focal_length),
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
