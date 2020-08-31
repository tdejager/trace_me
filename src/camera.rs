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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

/// Struct to create a camera with specific settings
pub struct CameraBuilder {
    // Look from this point
    look_from: Point3,
    // Camera looks at this point
    look_at: Point3,
    // The global up vector
    vup: Vec3,
    // The vertical fov in degrees
    vfov_degrees: f64,
    // The aspect ratio e.g 16 / 9
    aspect_ratio: f64,
    // Lens aperture
    aperture: f64,
    // The distance where the camera should start to focus
    focus_distance: f64,
}

impl CameraBuilder {
    /// Create a new builder object
    pub fn new(look_from: Point3, look_at: Point3) -> Self {
        Self {
            look_from,
            look_at,
            vup: Vec3::new(0., 1., 0.),
            aperture: 0.5,
            focus_distance: (look_from - look_at).length(),
            vfov_degrees: 20.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }

    /// Set the up vector
    pub fn set_up_vector(&mut self, vup: &Vec3) -> &Self {
        self.vup = *vup;
        self
    }

    /// Set the aperture of the camera lens
    pub fn set_aperture(&mut self, aperture: f64) -> &Self {
        self.aperture = aperture;
        self
    }

    /// Set the focus distance for the camera
    pub fn set_focus_distance(&mut self, focus_distance: f64) -> &Self {
        self.focus_distance = focus_distance;
        self
    }

    /// Set the aspect ratio for the camera
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> &Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Build the final camera
    pub fn build(self) -> Camera {
        Camera::new(
            &self.look_from,
            &self.look_at,
            &self.vup,
            self.vfov_degrees,
            self.aspect_ratio,
            self.aperture,
            self.focus_distance,
        )
    }
}

/// Convert from degrees to radians
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
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        // Calculate lengths based on the vertical fov
        let theta = degrees_to_radians(vfov_degrees);
        let h = (theta / 2.).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        // Calculate basis vectors
        // This looks at the point
        let w = (look_from - look_at).unit_vector();
        // This is perpendicular to the world up vector and the vector looking at the point
        let u = vup.cross(&w).unit_vector();
        // This the up vector
        let v = w.cross(&u);

        // Calculate camera parameters
        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;

        Camera {
            origin: *look_from,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - focus_distance * w,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    /// Cast a ray with the camera
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // Get an offset for the lens radius
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        // Transform it into the correct frame
        let offset = (self.u * rd.x) + (self.v * rd.y);
        
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
