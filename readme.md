## README

# File Compression Tool

This is a simple Rust program to compress files using Gzip compression. It reads a source file, compresses it, and writes the compressed data to a destination file.

### Usage

To use this tool, you need to have Rust installed on your machine. You can run the program with the following command:

```sh
cargo run -- <src> <dst>


cargo run -- input.txt output.gz

[dependencies]
flate2 = "1.0"
