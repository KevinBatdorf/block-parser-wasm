mod types;
pub use types::Block;
pub mod parser;
pub use parser::*;

use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

pub fn parse_blocks_internal(input: &str) -> Vec<Block> {
    block_parser::block_list(input).unwrap_or_else(|_| vec![])
}

#[wasm_bindgen]
pub fn parse_blocks(input: &str) -> JsValue {
    let blocks = parse_blocks_internal(input);
    let serializer = Serializer::json_compatible();
    blocks.serialize(&serializer).unwrap()
}
