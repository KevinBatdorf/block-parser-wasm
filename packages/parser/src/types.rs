use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Block {
    pub block_name: Option<String>,
    pub attrs: serde_json::Value,
    pub inner_blocks: Vec<Block>,
    pub inner_html: String,
    pub inner_content: Vec<Option<String>>,
}

impl Block {
    pub fn freeform(html: String) -> Self {
        Block {
            block_name: None,
            attrs: serde_json::json!({}),
            inner_blocks: vec![],
            inner_html: html.clone(),
            inner_content: vec![Some(html)],
        }
    }
}
