use std::fs::File;

use image::png::PNGEncoder;
use image::ColorType;
use chrono::Local;

use crate::MandelbrotError;

pub fn snapshot(pixels: &[u8], shape: [i32; 2], path: &str) -> Result<(), MandelbrotError> {
    let file = File::create(path.to_owned() + ".png")?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(pixels, shape[0] as u32, shape[1] as u32, ColorType::RGB(8))?;
    Ok(())
}
