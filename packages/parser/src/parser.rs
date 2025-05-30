use crate::Block;
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug)]
enum ContentPiece<'a> {
    Block(Block<'a>),
    Text(Cow<'a, str>),
}

peg::parser! {
    pub grammar block_parser() for str {
        rule ws() = [' ' | '\t' | '\r' | '\n']+

        rule block_name() -> Cow<'input, str>
            = namespaced:$(['a'..='z']['a'..='z' | '0'..='9' | '_' | '-']* "/" ['a'..='z']['a'..='z' | '0'..='9' | '_' | '-']*)
                { Cow::Borrowed(namespaced) }
            / core:$(['a'..='z']['a'..='z' | '0'..='9' | '_' | '-']*)
                {
                    let mut buf = String::with_capacity(5 + core.len()); // "core/" + core
                    buf.push_str("core/");
                    buf.push_str(core);
                    Cow::Owned(buf)
                }

        rule fallback_block_comment() -> &'input str
            = s:$("<!--" ws()? "wp:" (!"-->" [_])* ("-->" / "/" ws()? "-->")) { s }

        rule block_attrs() -> serde_json::Value
          = s:$("{" s:nested_json() "}") { serde_json::from_str(s).unwrap_or(Value::Null) }
          / { Value::Object(Default::default()) }

        rule nested_json() -> &'input str
            = $((quoted_string() / balanced_braces() / non_brace())*)
        rule non_brace() = !("{" / "}") [_]
        rule quoted_string() = "\"" (!"\"" [_])* "\""
        rule balanced_braces() = "{" nested_json() "}"
        rule freeform_text() -> Cow<'input, str>
          = s:$((!is_block_start() [_])+) {
              Cow::Borrowed(s)
          }

        // The start/end of a balanced block. returns (block_name, attrs)
        rule block_start() -> (Cow<'input, str>, Value)
            = "<!--" ws()? "wp:" name:block_name() ws()? attrs:block_attrs() ws()? "-->" {
                (name, attrs)
            }
        rule block_end() -> Cow<'input, str>
            = "<!--" ws()? "/wp:" name:block_name() ws()? "-->" { name }

        rule is_block_start() = "<!--" ws()? "wp:"
        rule is_block_end() = "<!--" ws()? "/wp:"

        rule content_piece() -> ContentPiece<'input> = b:block_balanced_nested()
            {?
                // handles when block is malformed (missing closing tags)
                match b.block_name {
                    Some(_) => Ok(ContentPiece::Block(b)),
                    None => Err("malformed block"),
                }
            }
            / b:block_void() { ContentPiece::Block(b) }
            / s:$((!is_block_end() !is_block_start() [_])+) {
                ContentPiece::Text(Cow::Borrowed(s))
            }

        rule block_with_trailing() -> (Block<'input>, Cow<'input, str>)
            = b:(block_balanced_top() / block_void()) html:freeform_text()? {
                (b, html.unwrap_or(Cow::Borrowed("")))
            }
            / fallback:fallback_block_comment() {
                (Block::freeform(Cow::Borrowed(fallback)), Cow::Borrowed(""))
            }

        // spec handles nested blocks slightly differently
        rule block_balanced_top() -> Block<'input> = block_balanced(false)
        rule block_balanced_nested() -> Block<'input> = block_balanced(true)
        rule block() -> Block<'input> = block_balanced_nested() / block_void()

        rule block_balanced(nested: bool) -> Block<'input>
            = start:block_start()
            children:content_piece()*
            end:block_end()? {
                let (name, attrs) = start;
                let mut inner_blocks = Vec::with_capacity(children.len());
                let mut inner_html = String::with_capacity(children.len() * 64);
                let mut inner_content = Vec::with_capacity(children.len());


                for piece in children {
                    match piece {
                        ContentPiece::Block(b) => {
                            if b.block_name.is_none() {
                                let mut buf = String::with_capacity(10 + name.len()); // "<!-- wp:" + name + "-->"
                                buf.push_str("<!-- wp:");
                                buf.push_str(&name);
                                buf.push_str(" -->");
                                return Block::freeform(Cow::Owned(buf));
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
                    inner_html: Cow::Owned(inner_html),
                    inner_content: inner_content,
                    is_freeform: None
                }
            }

        rule block_void() -> Block<'input>
            = "<!--" ws()? "wp:" name:block_name() ws()? attrs:block_attrs() ws()? "/-->" {
                Block {
                    block_name: Some(name),
                    attrs,
                    inner_blocks: vec![],
                    inner_html: Cow::Borrowed(""),
                    inner_content: vec![],
                    is_freeform: None,
                }
            }

        pub rule block_list_inner() -> Vec<Block<'input>>
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

        pub rule block_list() -> Vec<Block<'input>>
            = full:block_list_inner() { full }
            / fallback:fallback_block_comment() { vec![Block::freeform(Cow::Borrowed(fallback))] }
            / s:$([_]+) { vec![Block::freeform(Cow::Borrowed(s))] }
    }
}

pub use block_parser::*;
