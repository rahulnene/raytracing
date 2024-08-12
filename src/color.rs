use std::io::Write;

use nalgebra::Vector3;

use crate::interval::Interval;

pub type Color = Vector3<f64>;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let intensity = Interval::new(0.0, 0.9999);
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);
    // Write the translated [0, 255] value of each color component
    let r_final = (256.0 * intensity.clamp(r)) as i32;
    let g_final = (256.0 * intensity.clamp(g)) as i32;
    let b_final = (256.0 * intensity.clamp(b)) as i32;
    writeln!(out, "{} {} {}", r_final, g_final, b_final).expect("writing color");
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component < 0.0 {
        0.0
    } else {
        linear_component.sqrt()
    }
}
