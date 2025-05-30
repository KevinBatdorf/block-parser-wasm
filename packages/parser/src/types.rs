use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug, PartialEq)]
pub struct Block<'a> {
    #[serde(rename = "blockName")]
    pub block_name: Option<Cow<'a, str>>,
    #[serde(rename = "isFreeform", skip_serializing_if = "Option::is_none")]
    pub is_freeform: Option<bool>,
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
            block_name: None,
            is_freeform: Some(true),
            attrs: serde_json::json!({}),
            inner_blocks: vec![],
            inner_html: html.clone(),
            inner_content: vec![Some(html)],
        }
    }
}
