mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

/// Hit a sphere
fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let half_b = vec3::dot(&oc, &ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / (2.0 * a))
    }
}

/// Color the ray according to y component
pub fn color_ray(ray: &Ray) -> Vec3 {
    let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, ray);
    if t.is_some() {
        // Calculate the normal vector
        let normal = vec3::unit_vector(ray.at(t.unwrap()) - Vec3::new(0., 0., -1.));
        return 0.5 * Color::new(normal.x + 1., normal.y + 1., normal.z + 1.);
    }
    let dir_u = vec3::unit_vector(ray.dir);
    // Clamp to 0..1
    let t = 0.5 * dir_u.y + 1.0;
    // Linearly interpolate the ray color
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let camera_origin: Point3 = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner =
        camera_origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH);

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
            color::write_color(&color_ray(&ray));
            //let r = h as f64 / (WIDTH - 1) as f64;
            //let g = w as f64 / (HEIGHT - 1) as f64;
            //let b = 0.25;
            //color::write_color(&Color::new(r, g, b));
        }
    }
    eprintln!("Done");
}
