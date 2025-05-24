use block_parser_wasm::parse_blocks_internal;
use serde_json::json;

#[test]
fn parses_empty_string_to_empty_vec() {
    let blocks = parse_blocks_internal("");
    assert_eq!(blocks.len(), 0);
}

#[test]
fn parses_non_block_text_to_freeform_block() {
    let blocks = parse_blocks_internal("test");
    assert_eq!(blocks.len(), 1);
    let block = &blocks[0];
    assert_eq!(block.block_name, None);
    assert_eq!(block.attrs, json!({}));
    // assert_eq!(block.inner_blocks.len(), 0);
    assert_eq!(block.inner_html, "test");
    assert_eq!(block.inner_content, vec![Some("test".to_string())]);
}
