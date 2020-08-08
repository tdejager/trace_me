mod color;
mod geometry;
mod hittable;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
use crate::hittable::Hittable;
use crate::vec3::unit_vector;
use geometry::Sphere;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;


fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}


/// Color the ray according to y component
pub fn color_ray(ray: &Ray, world: &impl Hittable) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, std::f64::INFINITY) {
        // Color based on normal
        return 0.5 * (hit.normal + Color::new(1., 1., 1.))
    }
    // Color based on y, scale from [-1, 1] to [0, 1]
    let unit_dir = unit_vector(ray.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    // Linearly interpolate the ray color
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let camera_origin: Point3 = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner =
        camera_origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH);

    let sphere_center = Sphere{center: Point3::new(0., 0., -1.0), radius: 0.5};
    let big_sphere = Sphere{center: Point3::new(0., -100.5, -1.0), radius: 100.};

    let mut hittables: Vec<&dyn Hittable> = Vec::new();
    hittables.push(&sphere_center);
    hittables.push(&big_sphere);

    println!("P3\n {} {}\n255", WIDTH, HEIGHT);
    for h in (0..HEIGHT - 1).rev() {
        eprintln!("\tScanlines remaining: {}", h);
        for w in 0..WIDTH {
            let u = w as f64 / (WIDTH - 1) as f64;
            let v = h as f64 / (HEIGHT - 1) as f64;
            let ray = Ray::new(
                camera_origin,
                (lower_left_corner + u * horizontal + v * vertical) - camera_origin,
            );
            color::write_color(&color_ray(&ray, &hittables));
        }
    }
    eprintln!("Done");
}
