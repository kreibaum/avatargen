//! Writes the identicon to a file to make it easy to view.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use avatargen::identicon;

fn main() {
    let path = Path::new(r"output/image.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let seed_str = "204bedcd9a44b3e1db26e7619bca691c";

    identicon(seed_str, w);

    for i in 0..10 {
        let filename = format!("output/image{}.png", i);
        let path = Path::new(&filename);
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);

        identicon(&format!("{}{}", i, seed_str), w);
    }
}
