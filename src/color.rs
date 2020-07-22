use crate::vec3::Vec3;

pub type Color = Vec3;
const MULT: f64 = 255.999;

pub fn write_color(c: &Color) {
    println!("{:.0} {:.0} {:.0}", c.x * MULT, c.y * MULT, c.z * MULT);
}
