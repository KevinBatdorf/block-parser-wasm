use block_parser_wasm::parse_blocks_internal as parse;
// use serde_json::json;

#[test]
fn output_structure() {
    assert_eq!(parse("").len(), 0);
    assert_eq!(parse("test").len(), 1);
    assert_eq!(parse("<!-- wp:void /-->").len(), 1);
    assert_eq!(
        parse("<!-- wp:block --><!-- wp:inner /--><!-- /wp:block -->").len(),
        1
    );
    assert_eq!(parse("<!-- wp:first /--><!-- wp:second /-->").len(), 2);
}

#[test]
fn parses_blocks_with_line_breaks() {
    let parsed = parse(
        "<!-- wp:group -->
	  <p>Before</p>
	  <!-- wp:paragraph -->
	    <em>Inside</em>
	  <!-- /wp:paragraph -->
	  <p>After</p>
	<!-- /wp:group -->
",
    ); // space after is important as wp creates another block
    assert_eq!(parsed.len(), 1);
    assert_eq!(
        parsed[0].inner_html,
        "\n\t  <p>Before</p>\n\t  \n\t  <p>After</p>\n\t"
    )
}

#[test]
fn parses_blocks_of_various_types() {
    let cases = vec![
        "<!-- wp:void /-->",
        "<!-- wp:void {} /-->",
        r#"<!-- wp:void {"value":true} /-->"#,
        r#"<!-- wp:void {"a":{}} /-->"#,
        r#"<!-- wp:void { "value" : true } /-->"#,
        r#"<!-- wp:void {\n\t"value" : true\n} /-->"#,
    ];

    for input in cases {
        let blocks = parse(input);
        assert_eq!(blocks.len(), 1, "Expected 1 block from input: {}", input);
        let block = &blocks[0];
        assert_eq!(
            block.block_name.as_deref(),
            Some("core/void"),
            "Input: {}",
            input
        );
    }
    let cases = vec![
        "<!-- wp:block --><!-- /wp:block -->",
        "<!-- wp:block {} --><!-- /wp:block -->",
        r#"<!-- wp:block {"value":true} --><!-- /wp:block -->"#,
        r#"<!-- wp:block {"a":{}} --><!-- /wp:block -->"#,
        r#"<!-- wp:block { "value" : true } --><!-- /wp:block -->"#,
        r#"<!-- wp:block {\n\t"value" : true\n} --><!-- /wp:block -->"#,
    ];

    for input in cases {
        let blocks = parse(input);
        assert_eq!(blocks.len(), 1, "Expected 1 block from input: {}", input);
        let block = &blocks[0];
        assert_eq!(
            block.block_name.as_deref(),
            Some("core/block"),
            "Input: {}",
            input
        );
    }
}

#[test]
fn blockname_is_namespaced_string_except_freeform() {
    assert_eq!(
        parse("freeform has full name")[0].block_name.as_deref(),
        None
    );
    // Incorrect case on B in Block
    assert_eq!(parse("<!-- wp:Block / -->")[0].block_name.as_deref(), None);
    assert_eq!(
        parse("<!-- wp:Block / -->")[0].inner_html,
        "<!-- wp:Block / -->"
    );
    assert_eq!(
        parse("<!-- wp:more /-->")[0].block_name.as_deref(),
        Some("core/more")
    );
    assert_eq!(
        parse("<!-- wp:core/more /-->")[0].block_name.as_deref(),
        Some("core/more")
    );
    assert_eq!(
        parse("<!-- wp:my/more /-->")[0].block_name.as_deref(),
        Some("my/more")
    );
}

#[test]
fn json_attributes_are_key_value_object() {
    assert_eq!(
        parse("freeform has empty attrs")[0].attrs,
        serde_json::json!({})
    );
    assert_eq!(parse("<!-- wp:void /-->")[0].attrs, serde_json::json!({}));
    assert_eq!(
        parse("<!-- wp:void {} /-->")[0].attrs,
        serde_json::json!({})
    );
    assert_eq!(
        parse(r#"<!-- wp:void {"key":"value"} /-->"#)[0].attrs,
        serde_json::json!({"key": "value"})
    );
    assert_eq!(
        parse("<!-- wp:block --><!-- /wp:block -->")[0].attrs,
        serde_json::json!({})
    );
    assert_eq!(
        parse("<!-- wp:block {} --><!-- /wp:block -->")[0].attrs,
        serde_json::json!({})
    );
    assert_eq!(
        parse(r#"<!-- wp:block {"key": "value"} --><!-- /wp:block -->"#)[0].attrs,
        serde_json::json!({"key": "value"})
    );
}

#[test]
fn inner_blocks_is_a_list() {
    assert_eq!(
        parse("freeform has empty innerBlocks")[0].inner_blocks,
        vec![]
    );
    assert_eq!(parse("<!-- wp:void /-->")[0].inner_blocks, vec![]);
    assert_eq!(
        parse("<!-- wp:block {} --><!-- /wp:block -->")[0].inner_blocks,
        vec![]
    );
    assert_eq!(
        parse("<!-- wp:block --><!-- wp:inner /--><!-- /wp:block -->")[0]
            .inner_blocks
            .len(),
        1
    );
    assert_eq!(
        parse("<!-- wp:block -->a<!-- wp:first /-->b<!-- wp:second /-->c<!-- /wp:block -->")[0]
            .inner_blocks
            .len(),
        2
    );
}

#[test]
fn inner_html_is_a_string() {
    assert_eq!(parse("test")[0].inner_html, "test");
    assert_eq!(parse("<!-- wp:test /-->")[0].inner_html, "");
    assert_eq!(parse("<!-- wp:test --><!-- /wp:test -->")[0].inner_html, "");
    assert_eq!(
        parse("<!-- wp:test -->test<!-- /wp:test -->")[0].inner_html,
        "test"
    );
}

#[test]
fn parse_accepts_inputs_with_multiple_reusable_blocks() {
    let blocks = parse(r#"<!-- wp:block {"ref":313} /--><!-- wp:block {"ref":482} /-->"#);
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].block_name.as_deref(), Some("core/block"));
    assert_eq!(blocks[0].attrs, serde_json::json!({"ref": 313}));
    assert_eq!(blocks[1].block_name.as_deref(), Some("core/block"));
    assert_eq!(blocks[1].attrs, serde_json::json!({"ref": 482}));
}

#[test]
fn treats_void_blocks_and_empty_blocks_identically() {
    assert_eq!(
        parse("<!-- wp:block /-->"),
        parse("<!-- wp:block --><!-- /wp:block -->")
    );
    assert_eq!(
        parse(r#"<!-- wp:my/bus { "is": "fast" } /-->"#),
        parse(r#"<!-- wp:my/bus { "is": "fast" } --><!-- /wp:my/bus -->"#)
    );
}

#[test]
fn should_grab_html_soup_before_block_openers() {
    let cases = vec![
        "<p>Break me</p><!-- wp:block /-->",
        "<p>Break me</p><!-- wp:block --><!-- /wp:block -->",
    ];
    for input in cases {
        let blocks = parse(input);
        assert_eq!(blocks.len(), 2, "Expected 2 blocks from input: {}", input);
        assert_eq!(blocks[0].inner_html, "<p>Break me</p>");
        assert_eq!(
            blocks[0].inner_content,
            vec![Some("<p>Break me</p>".into())]
        );
        assert_eq!(blocks[1].inner_html, "");
        assert_eq!(blocks[1].inner_content, vec![]);
    }
}
#[test]
fn should_grab_html_soup_before_inner_block_openers() {
    let cases = vec![
        "<!-- wp:outer --><p>Break me</p><!-- wp:block /--><!-- /wp:outer -->",
        "<!-- wp:outer --><p>Break me</p><!-- wp:block --><!-- /wp:block --><!-- /wp:outer -->",
    ];
    for input in cases {
        let blocks = parse(input);
        assert_eq!(blocks.len(), 1, "Expected 1 block from input: {}", input);
        assert_eq!(blocks[0].inner_html, "<p>Break me</p>");
        assert_eq!(
            blocks[0].inner_blocks[0].block_name.as_deref(),
            Some("core/block")
        );
        assert_eq!(blocks[0].inner_blocks[0].inner_html, "");
    }
}

#[test]
fn should_grab_html_soup_after_blocks() {
    let cases = vec![
        "<!-- wp:block /--><p>Break me</p>",
        "<!-- wp:block --><!-- /wp:block --><p>Break me</p>",
    ];
    for input in cases {
        let blocks = parse(input);
        assert_eq!(blocks.len(), 2, "Expected 2 blocks from input: {}", input);
        assert_eq!(blocks[0].inner_html, "");
        assert_eq!(blocks[0].inner_content, vec![]);
        assert_eq!(blocks[1].inner_html, "<p>Break me</p>");
        assert_eq!(
            blocks[1].inner_content,
            vec![Some("<p>Break me</p>".into())]
        );
    }
}

#[test]
fn inner_block_placemarkers() {
    // Inner content exists
    assert_eq!(parse("test")[0].inner_content, vec![Some("test".into())]);
    assert_eq!(parse("<!-- wp:void /-->")[0].inner_content, vec![]);

    // innerContent contains innerHtml
    assert_eq!(
        parse("<!-- wp:block -->Inner<!-- /wp:block -->")[0].inner_content,
        vec![Some("Inner".into())]
    );
    assert_eq!(
        parse("<!-- wp:block --><!-- wp:void /--><!-- /wp:block -->")[0].inner_content,
        vec![None]
    );

    // block locations become null
    assert_eq!(
        parse("<!-- wp:block --><!-- wp:void /--><!-- /wp:block -->")[0].inner_content,
        vec![None]
    );

    // html soup appears after blocks
    assert_eq!(
        parse("<!-- wp:block --><!-- wp:void /-->After<!-- /wp:block -->")[0].inner_content,
        vec![None, Some("After".into())]
    );

    // html soup appears before blocks
    assert_eq!(
        parse("<!-- wp:block -->Before<!-- wp:void /--><!-- /wp:block -->")[0].inner_content,
        vec![Some("Before".into()), None]
    );

    // blocks follow each over
    assert_eq!(
        parse("<!-- wp:block --><!-- wp:void /--><!-- wp:void /--><!-- /wp:block -->")[0]
            .inner_content,
        vec![None, None]
    );
}

#[test]
fn attack_vectors() {
    // Really long JSON attributes
    let length = 100_000;
    let long_string = "a".repeat(length);
    let input = format!(r#"<!-- wp:fake {{"a":"{}"}} /-->"#, long_string);

    let blocks = parse(&input);
    assert_eq!(blocks.len(), 1);

    let block = &blocks[0];
    let a_value = block.attrs.get("a").and_then(|v| v.as_str());

    assert!(a_value.is_some(), "Expected 'a' key in attrs");
    assert_eq!(a_value.unwrap().len(), length);
}

#[test]
fn invalid_block_comment_syntax() {
    // extra space after void closer
    assert_eq!(parse("<!-- wp:block / -->")[0].block_name.as_deref(), None);
    assert_eq!(
        parse("<!-- wp:block / -->")[0].inner_content,
        vec![Some("<!-- wp:block / -->".into())]
    );
    assert_eq!(
        parse("<!-- wp:block / -->")[0].inner_html,
        "<!-- wp:block / -->"
    );
}

#[test]
fn sets_attrs_to_null_on_json_error() {
    let input = r#"<!-- wp:void {"broken": "json} /-->"#;
    let blocks = parse(input);
    assert_eq!(blocks[0].attrs, serde_json::Value::Null);
}

#[test]
fn parses_malformed_blocks() {
    // Missing closing tag
    let input = "<!-- wp:block --><!-- wp:inner /-->";
    let blocks = parse(input);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].block_name.as_deref(), Some("core/block"));
    assert_eq!(blocks[0].inner_blocks.len(), 1);
    assert_eq!(
        blocks[0].inner_blocks[0].block_name.as_deref(),
        Some("core/inner")
    );

    // Extra content after block end
    let input = "<!-- wp:block --><!-- /wp:block -->Extra content";
    let blocks = parse(input);
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].block_name.as_deref(), Some("core/block"));
    assert_eq!(blocks[1].inner_html, "Extra content");

    // Attempts to close tags when missing or malformed
    // ! This diverges from the wp spec, but I feel this is more gracefully handled
    let input = "<!-- wp:block --><!-- wp:inner -->";
    let blocks = parse(input);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].block_name.as_deref(), Some("core/block"));
    assert_eq!(blocks[0].inner_blocks.len(), 1);
    assert_eq!(
        blocks[0].inner_blocks[0].block_name.as_deref(),
        Some("core/inner")
    );

    // multi-nested malformed blocks
    let input = "<!-- wp:block -->a<!-- wp:first -->ok<!-- wp:first -->b<!-- wp:second /-->c<!-- /wp:block -->";
    let blocks = parse(input);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].block_name.as_deref(), Some("core/block"));
    assert_eq!(blocks[0].inner_blocks.len(), 1);
    assert_eq!(
        blocks[0].inner_blocks[0].block_name.as_deref(),
        Some("core/first")
    );
    let nested2 = &blocks[0].inner_blocks[0];
    assert_eq!(nested2.inner_blocks.len(), 1);
    assert_eq!(nested2.inner_html, "ok");
    assert_eq!(
        nested2.inner_blocks[0].block_name.as_deref(),
        Some("core/first")
    );
    let nested3 = &nested2.inner_blocks[0];
    assert_eq!(nested3.inner_blocks.len(), 1);
    assert_eq!(nested3.block_name.as_deref(), Some("core/first"));
    assert_eq!(nested3.inner_html, "bc");
    assert_eq!(
        nested3.inner_content,
        vec![Some("b".into()), None, Some("c".into())]
    );
    let nested4 = &nested3.inner_blocks[0];
    assert_eq!(nested4.block_name.as_deref(), Some("core/second"));
    assert_eq!(nested4.inner_blocks.len(), 0);
    assert_eq!(nested4.inner_html, "");
    assert_eq!(nested4.inner_content, vec![]);
}
