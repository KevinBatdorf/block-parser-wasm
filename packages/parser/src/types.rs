use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Block {
    #[serde(rename = "blockName")]
    pub block_name: Option<String>,
    #[serde(rename = "attrs")]
    pub attrs: serde_json::Value,
    #[serde(rename = "innerBlocks")]
    pub inner_blocks: Vec<Block>,
    #[serde(rename = "innerHTML")]
    pub inner_html: String,
    #[serde(rename = "innerContent")]
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
