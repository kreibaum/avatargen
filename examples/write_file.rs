//! Writes the identicon to a file to make it easy to view.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(&data).unwrap(); // Save
}
