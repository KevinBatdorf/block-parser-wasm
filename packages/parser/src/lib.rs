mod types;
pub use types::Block;
pub mod parser;
pub use parser::*;

use std::borrow::Cow;
use wasm_bindgen::prelude::*;

pub fn parse_blocks_internal(input: &str) -> Vec<Block> {
    match block_list(input.trim()) {
        Ok(blocks) => blocks,
        Err(_) => vec![Block::freeform(Cow::Borrowed(input.trim()))],
    }
}

#[wasm_bindgen]
pub fn parse_blocks(input: &str) -> String {
    let blocks = parse_blocks_internal(input);
    serde_json::to_string(&blocks).unwrap()
}
