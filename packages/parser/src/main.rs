use block_parser_wasm::parse_blocks;
use std::fs;
fn main() {
    let filename = std::env::args().nth(1).expect("No file given");
    let input = fs::read_to_string(&filename).expect("Failed to read file");
    // start timer
    let start = std::time::Instant::now();
    parse_blocks(&input);
    let duration = start.elapsed();
    println!("Parsing took: {:?}", duration);
}
