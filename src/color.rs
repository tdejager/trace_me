use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Write a color to the standard out
pub fn write_color(c: &Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    // Divide the color by the number of samples
    let r = (c.x * scale).sqrt();
    let g = (c.y * scale).sqrt();
    let b = (c.z * scale).sqrt();

    // Write the translated [0,255] value of each color component
    println!(
        "{:.0} {:.0} {:.0}",
        256.0 * clamp(r, 0., 0.999),
        256.0 * clamp(g, 0., 0.999),
        256.0 * clamp(b, 0., 0.999),
    );
}
