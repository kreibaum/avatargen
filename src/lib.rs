use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::Write;

pub fn identicon<W: Write>(seed_str: &str, w: W) {
    let mut rng = seed_rng(seed_str);

    let data = on_off_grid(&mut rng);

    // Generate a random hue and use it to create a palette
    // The palette has 0, 0 , 0 first and then the color
    let hue = random_hue(&mut rng);
    let color = hsv_to_rgb(hue, 0.9, 0.7);
    let palette = [0, 0, 0, color[0], color[1], color[2]];

    let mut writer = create_encoder(w, palette);
    writer.write_image_data(&scale_up(data)).unwrap();
}

// A random float in the range 0-360
fn random_hue(rng: &mut StdRng) -> f32 {
    rng.gen_range(0.0..360.0)
}

/// Allowed range: h: 0-360, s: 0-1.0, v: 0-1.0
/// Output range: r: 0-255, g: 0-255, b: 0-255
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}

fn create_encoder<W: Write>(w: W, palette: [u8; 6]) -> png::Writer<W> {
    let mut encoder = png::Encoder::new(w, 100, 100);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_palette(palette.as_ref());
    encoder.set_trns([0, 255].as_ref());
    encoder.write_header().unwrap()
}

/// Set up a vertically symmetrical 5x5 grid of pixels
/// They are all set to 0 or 1 and can be colored in subsequent steps.
fn on_off_grid(rng: &mut StdRng) -> [u8; 25] {
    let mut data = [0; 25];
    for x in 0..3 {
        for y in 0..5 {
            data[x + y * 5] = if rng.gen() { 1 } else { 0 };
            data[(4 - x) + y * 5] = data[x + y * 5];
        }
    }
    data
}

fn seed_rng(seed_str: &str) -> StdRng {
    let mut seed: [u8; 32] = [0; 32];
    let seed_bytes = seed_str.as_bytes();

    // Determine the number of bytes to copy (up to 32)
    let len = std::cmp::min(seed_bytes.len(), seed.len());

    // Copy the bytes from the seed string into the seed array
    seed[..len].copy_from_slice(&seed_bytes[..len]);

    StdRng::from_seed(seed)
}

fn scale_up(data: [u8; 25]) -> [u8; 10000] {
    let mut image_data = [0; 10000];
    for x in 0..5 {
        for y in 0..5 {
            for i in 0..20 {
                for j in 0..20 {
                    image_data[(x * 20 + i) * 100 + y * 20 + j] = data[x * 5 + y];
                }
            }
        }
    }
    image_data
}
