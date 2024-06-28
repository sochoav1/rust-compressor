mod compress;
mod decompress;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        eprintln!("Usage: compress <src> <dst>");
        eprintln!("       decompress <src> <dst>");
        return;
    }

    let action = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    match action.as_str() {
        "compress" => {
            compress::compress(input_path, output_path);
        },
        "decompress" => {
            decompress::decompress(input_path, output_path);
        },
        _ => {
            eprintln!("Invalid action: {}. Use 'compress' or 'decompress'.", action);
        }
    }
}
