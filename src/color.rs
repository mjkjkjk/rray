use crate::{math::interval::Interval, vec3::Vec3};
use std::io::Write;
pub type Color = Vec3;

pub fn write_color(file: &mut std::fs::File, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r_byte = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let g_byte = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let b_byte = (256.0 * b.clamp(0.0, 0.999)) as u8;

    writeln!(file, "{} {} {}", r_byte, g_byte, b_byte)
}
