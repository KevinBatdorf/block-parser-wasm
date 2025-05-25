use crate::Block;

#[derive(Debug)]
enum ContentPiece {
    Block(Block),
    Text(String),
}

peg::parser! {
    pub grammar block_parser() for str {
        rule ws() = [' ' | '\t' | '\r' | '\n']+

        rule block_name() -> String
            = namespaced:$(['a'..='z']['a'..='z' | '0'..='9' | '_' | '-']* "/" ['a'..='z']['a'..='z' | '0'..='9' | '_' | '-']*)
                { namespaced.to_string() }
            / core:$(['a'..='z']['a'..='z' | '0'..='9' | '_' | '-']*)
                { format!("core/{}", core) }

        rule fallback_block_comment() -> &'input str
            = s:$("<!--" ws()? "wp:" (!"-->" [_])* ("-->" / "/" ws()? "-->")) { s }

        rule block_attrs() -> serde_json::Value
            = "{" s:nested_json() "}" {
                let json = format!("{{{}}}", s);
                serde_json::from_str(&json).unwrap_or(serde_json::Value::Null)
            }

        rule nested_json() -> &'input str
            = $((quoted_string() / balanced_braces() / non_brace())*)
        rule non_brace() = !("{" / "}") [_]
        rule quoted_string() = "\"" (!"\"" [_])* "\""
        rule balanced_braces() = "{" nested_json() "}"
        rule freeform_text() -> String
            = s:$((!is_block_start() [_])+) { s.to_string() }

        // The start/end of a balanced block. returns (block_name, attrs)
        rule block_start() -> (String, serde_json::Value)
            = "<!--" ws()? "wp:" name:block_name() ws()? attrs:block_attrs()? ws()? "-->" {
                (name, attrs.unwrap_or_else(|| serde_json::json!({})))
            }
        rule block_end() -> String
            = "<!--" ws()? "/wp:" name:block_name() ws()? "-->" { name }

        rule is_block_start() = "<!--" ws()? "wp:"
        rule is_block_end() = "<!--" ws()? "/wp:"

        rule content_piece() -> ContentPiece = b:block_balanced_nested()
            {?
                // handles when block is malformed (missing closing tags)
                match b.block_name {
                    Some(_) => Ok(ContentPiece::Block(b)),
                    None => Err("malformed block"),
                }
            }
            / b:block_void() { ContentPiece::Block(b) }
            / s:$((!is_block_end() !is_block_start() [_])+) {
                ContentPiece::Text(s.to_string())
            }

        rule block_with_trailing() -> (Block, String)
            = b:(block_balanced_top() / block_void()) html:freeform_text()? {
                (b, html.unwrap_or("".into()))
            }
            / fallback:fallback_block_comment() {
                (Block::freeform(fallback.to_string()), "".into())
            }

        // spec handles nested blocks slightly differently
        rule block_balanced_top() -> Block = block_balanced(false)
        rule block_balanced_nested() -> Block = block_balanced(true)
        rule block() -> Block = block_balanced_nested() / block_void()

        rule block_balanced(nested: bool) -> Block
            = start:block_start()
            children:content_piece()*
            end:block_end()? {
                let (name, attrs) = start;
                let mut inner_blocks = vec![];
                let mut inner_html = String::new();
                let mut inner_content = vec![];

                for piece in children {
                    match piece {
                        ContentPiece::Block(b) => {
                            if b.block_name.is_none() {
                                return Block::freeform(format!("<!-- wp:{} -->", name));
                            }
                            inner_blocks.push(b);
                            inner_content.push(None);
                        },
                        ContentPiece::Text(s) => {
                            inner_html.push_str(&s);
                            inner_content.push(Some(s));
                        }
                    }
                }

                Block {
                    block_name: Some(name),
                    attrs,
                    inner_blocks,
                    inner_html,
                    inner_content,
                }
            }

        rule block_void() -> Block
            = "<!--" ws()? "wp:" name:block_name() ws()? attrs:block_attrs()? ws()? "/-->" {
                Block {
                    block_name: Some(name),
                    attrs: attrs.unwrap_or_else(|| serde_json::json!({})),
                    inner_blocks: vec![],
                    inner_html: "".to_string(),
                    inner_content: vec![],
                }
            }

        pub rule block_list_inner() -> Vec<Block>
            = pre:freeform_text()? items:(block_with_trailing() ** "") post:freeform_text()? {
                let mut blocks = vec![];

                // Push non-empty text as a freeform block if it exists
                if let Some(text) = pre {
                    if !text.is_empty() {
                    blocks.push(Block::freeform(text));
                    }
                }

                for (block, html) in items {
                    // Always push the parsed block itself
                    blocks.push(block);

                    // If there was trailing text, push it too
                    if !html.is_empty() {
                        blocks.push(Block::freeform(html));
                    }
                }

                // If there's any HTML/text after the last block, push it
                if let Some(text) = post {
                    if !text.is_empty() {
                        blocks.push(Block::freeform(text));
                    }
                }
                blocks
            }

        pub rule block_list() -> Vec<Block>
            = full:block_list_inner() { full }
            / fallback:fallback_block_comment() { vec![Block::freeform(fallback.to_string())] }
            / s:$([_]+) { vec![Block::freeform(s.to_string())] }
    }
}

pub use block_parser::*;
