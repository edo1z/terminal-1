use core_graphics::display::CGDisplay;
use png::{BitDepth, ColorType, Encoder};
use std::fs;

fn main() {
    let display_ids = CGDisplay::active_displays().ok().unwrap();
    let cg_display = CGDisplay::new(display_ids[0]);
    let cg_img = cg_display.image().unwrap();
    let bgra = Vec::from(cg_img.data().bytes());
    let img = make_png(cg_img.width() as u32, cg_img.height() as u32, bgra);
    fs::write("target/screen.png", &img).unwrap();
}

fn make_png(width: u32, height: u32, bgra: Vec<u8>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut bytes = bgra.clone();
    for i in (0..bytes.len()).step_by(4) {
        let b = bytes[i];
        let r = bytes[i + 2];
        bytes[i] = r;
        bytes[i + 2] = b;
        bytes[i + 3] = 255;
    }
    let mut encoder = Encoder::new(&mut buffer, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bytes).unwrap();
    writer.finish().unwrap();
    buffer
}
