use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug, PartialEq)]
pub struct Block<'a> {
    #[serde(rename = "blockName")]
    pub block_name: Option<Cow<'a, str>>,
    #[serde(rename = "attrs")]
    pub attrs: serde_json::Value,
    #[serde(rename = "innerBlocks")]
    pub inner_blocks: Vec<Block<'a>>,
    #[serde(rename = "innerHTML")]
    pub inner_html: Cow<'a, str>,
    #[serde(rename = "innerContent")]
    pub inner_content: Vec<Option<Cow<'a, str>>>,
}

impl<'a> Block<'a> {
    pub fn freeform(html: Cow<'a, str>) -> Self {
        Block {
            block_name: Some("core/freeform".into()),
            attrs: serde_json::json!({}),
            inner_blocks: vec![],
            inner_html: html.clone(),
            inner_content: vec![Some(html)],
        }
    }
}
