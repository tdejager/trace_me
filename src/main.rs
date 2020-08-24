mod camera;
mod color;
mod geometry;
mod hittable;
mod material;
mod ray;
mod vec3;

use crate::camera::CameraBuilder;
use crate::hittable::Hittable;
use crate::vec3::unit_vector;
use camera::Camera;
use color::Color;
use geometry::Sphere;
use material::{Dialectric, Lambertian, Metal};
use rand::prelude::*;
use ray::Ray;
use vec3::{Point3, Vec3};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 50;

/// Color the ray according to y component
pub fn color_ray(ray: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(hit) = world.hit(ray, 0.0001, std::f64::INFINITY) {
        // Color based on the material
        if let Some(material) = hit.material.scatter(ray, &hit) {
            return material.attenuation * color_ray(&material.scattered, world, depth - 1);
        }
    }
    // Color based on y, scale from [-1, 1] to [0, 1]
    let unit_dir = unit_vector(ray.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    // Linearly interpolate the ray color
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dialectric::new(1.5);
    let material_right = Metal::new(Color::new(0.7, 0.6, 0.2), 0.0);

    let big_sphere = Sphere {
        center: Point3::new(0., -100.5, -1.0),
        radius: 100.,
        material: Box::new(material_ground),
    };
    let sphere_center = Sphere {
        center: Point3::new(0., 0., -1.0),
        radius: 0.5,
        material: Box::new(material_center),
    };
    let sphere_left = Sphere {
        center: Point3::new(-1.0, 0., -1.0),
        radius: 0.5,
        material: Box::new(material_left),
    };
    let sphere_right = Sphere {
        center: Point3::new(1.0, 0., -1.0),
        radius: 0.5,
        material: Box::new(material_right),
    };

    let mut hittables: Vec<&dyn Hittable> = Vec::new();
    hittables.push(&sphere_center);
    hittables.push(&big_sphere);
    hittables.push(&sphere_left);
    hittables.push(&sphere_right);

    // Create camera
    let look_from = Vec3::new(-2., 2., 1.);
    let look_at = Vec3::new(0., 0., -1.);

    let camera = CameraBuilder::new(look_from, look_at).build();
    let mut rng = rand::thread_rng();
    let samples_per_pixel: u32 = 100;

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
