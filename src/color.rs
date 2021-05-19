use std::io::Write;

use crate::rtweekend::clamp;

use super::vec3::*;

pub fn write_color(stream: &mut impl Write, pixel_color: Color, samples_per_pixel: i64) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    write!(
        stream,
        "{} {} {}\n",
        (256 as f64 * clamp(r, 0.0, 0.999)) as i64,
        (256 as f64 * clamp(g, 0.0, 0.999)) as i64,
        (256 as f64 * clamp(b, 0.0, 0.999)) as i64
    )
    .unwrap();
}
