extern crate flate2;

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufWriter, copy};
use std::time::Instant;

pub fn decompress(input_path: &str, output_path: &str) {
    let input_file = File::open(input_path).expect(&format!("Failed to open input file: {}", input_path));
    let mut decoder = GzDecoder::new(input_file);
    let output_file = File::create(output_path).expect(&format!("Failed to create output file: {}", output_path));
    let mut output = BufWriter::new(output_file);

    let start = Instant::now();
    copy(&mut decoder, &mut output).expect("Failed to copy data from decoder");

    println!("Elapsed: {:?}", start.elapsed());
}
