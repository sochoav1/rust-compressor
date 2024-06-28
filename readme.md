# File Compression and Decompression Tool

This is a simple Rust program to compress and decompress files using Gzip compression. It reads a source file, compresses or decompresses it, and writes the compressed or decompressed data to a destination file.

## Usage

To use this tool, you need to have Rust installed on your machine. You can run the program with the following commands:

### Compression

```sh
cargo run -- compress <src> <dst>

cargo run -- decompress <src> <dst>

[dependencies]
flate2 = "1.0"
