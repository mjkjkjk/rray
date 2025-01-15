use crate::vec3::Vec3;
use std::io::Write;
pub type Color = Vec3;

pub fn write_color(file: &mut std::fs::File, pixel_color: Color) -> std::io::Result<()> {
    let r = (255.999 * pixel_color.x()) as u8;
    let g = (255.999 * pixel_color.y()) as u8;
    let b = (255.999 * pixel_color.z()) as u8;
    writeln!(file, "{} {} {}", r, g, b)
}
