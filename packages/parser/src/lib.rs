mod types;
pub use types::Block;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

pub fn parse_blocks_internal(input: &str) -> Vec<Block> {
    if input.trim().is_empty() {
        vec![]
    } else {
        vec![Block::freeform(input.to_string())]
    }
}

#[wasm_bindgen]
pub fn parse_blocks(input: &str) -> JsValue {
    let result = parse_blocks_internal(input);
    to_value(&result).unwrap()
}
