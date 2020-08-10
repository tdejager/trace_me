mod camera;
mod color;
mod geometry;
mod hittable;
mod ray;
mod vec3;

use crate::hittable::Hittable;
use crate::vec3::unit_vector;
use camera::Camera;
use color::Color;
use geometry::Sphere;
use rand::prelude::*;
use ray::Ray;
use vec3::{Point3, Vec3};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 50;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// Color the ray according to y component
pub fn color_ray(ray: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(hit) = world.hit(ray, 0.0001, std::f64::INFINITY) {
        // Lambertian shading
        let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
        // Color based on normal
        return 0.5 * color_ray(&Ray::new(hit.p, target - hit.p), world, depth - 1);
    }
    // Color based on y, scale from [-1, 1] to [0, 1]
    let unit_dir = unit_vector(ray.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    // Linearly interpolate the ray color
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let sphere_center = Sphere {
        center: Point3::new(0., 0., -1.0),
        radius: 0.5,
    };
    let big_sphere = Sphere {
        center: Point3::new(0., -100.5, -1.0),
        radius: 100.,
    };

    let camera = Camera::new();
    let mut rng = rand::thread_rng();
    let samples_per_pixel: u32 = 100;

    let mut hittables: Vec<&dyn Hittable> = Vec::new();
    hittables.push(&sphere_center);
    hittables.push(&big_sphere);

    println!("P3\n {} {}\n255", WIDTH, HEIGHT);
    for h in (0..HEIGHT - 1).rev() {
        eprintln!("\tScanlines remaining: {}", h);
        for w in 0..WIDTH {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (w as f64 + rng.gen::<f64>()) / (WIDTH - 1) as f64;
                let v = (h as f64 + rng.gen::<f64>()) / (HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += color_ray(&ray, &hittables, MAX_DEPTH)
            }
            color::write_color(&pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}
