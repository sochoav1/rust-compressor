extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, copy};
use std::time::Instant;

pub fn compress(input_path: &str, output_path: &str) {
    let input_file = File::open(input_path).expect(&format!("Failed to open input file: {}", input_path));
    let mut input = BufReader::new(input_file);
    let output_file = File::create(output_path).expect(&format!("Failed to create output file: {}", output_path));
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).expect("Failed to copy data to encoder");
    let output = encoder.finish().expect("Failed to finish encoding");

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().expect("Failed to get input file metadata").len()
    );
    println!("Target len: {:?}", output.metadata().expect("Failed to get output file metadata").len());
    println!("Elapsed: {:?}", start.elapsed());
}
