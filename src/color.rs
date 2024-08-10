use std::io::Write;

use crate::{interval::Interval, vec::Vec3};

// Type alias
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let intensity = Interval::new(0.0, 0.9999);
    // Write the translated [0, 255] value of each color component
    let r = (256.0 * intensity.clamp(pixel_color.x())) as i32;
    let g = (256.0 * intensity.clamp(pixel_color.y())) as i32;
    let b = (256.0 * intensity.clamp(pixel_color.z())) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
