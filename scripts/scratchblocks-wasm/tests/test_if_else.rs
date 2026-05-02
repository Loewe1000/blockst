use scratchblocks_wasm::{parse_request_json, render_request_json};

#[test]
fn test_if_else_parsing() {
    // German if-else block
    let input = r#"{
        "code": "falls <> dann\n  sage [Hallo]\nsonst\n  sage [Tschüss]\nende",
        "language": "de",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    println!("{}", output);
    // Parse output JSON to verify structure
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    // Output is a flat array of blocks
    let blocks = parsed.as_array().unwrap();
    assert!(!blocks.is_empty());
    // First block should be if block with else_body (serialized as "else-body")
    let if_block = &blocks[0];
    let else_body = if_block.get("else-body").unwrap().as_array().unwrap();
    assert!(!else_body.is_empty(), "else_body should not be empty");
    // The else_body should contain a say block
    let else_block = &else_body[0];
    let else_id = else_block.get("id").unwrap().as_str().unwrap();
    assert_eq!(else_id, "LOOKS_SAY");
}

#[test]
fn test_control_else_shape() {
    // Test that "sonst" line is recognized as CONTROL_ELSE with shape "celse"
    let input = r#"{
        "code": "sonst",
        "language": "de",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    // Output is a flat array of blocks
    let blocks = parsed.as_array().unwrap();
    assert!(!blocks.is_empty());
    let block = &blocks[0];
    let shape = block.get("shape").unwrap().as_str().unwrap();
    assert_eq!(shape, "celse");
    let id = block.get("id").unwrap().as_str().unwrap();
    assert_eq!(id, "CONTROL_ELSE");
}

#[test]
fn test_less_than_operator_inside_predicate() {
    let input = r#"{
        "code": "if <(x position) < (-230)> then\nend",
        "language": "en",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let if_block = &blocks[0];
    assert_eq!(if_block.get("id").unwrap().as_str().unwrap(), "CONTROL_IF");

    let condition = &if_block.get("parts").unwrap().as_array().unwrap()[1];
    assert_eq!(condition.get("kind").unwrap().as_str().unwrap(), "input");
    assert_eq!(condition.get("shape").unwrap().as_str().unwrap(), "boolean");
}

#[test]
fn test_touching_color_uses_color_input() {
    let input = r##"{
        "code": "<touching color [#ff4136]?>",
        "language": "en",
        "inline": false
    }"##;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let block = &blocks[0];
    assert_eq!(block.get("id").unwrap().as_str().unwrap(), "SENSING_TOUCHINGCOLOR");

    let color_input = &block.get("parts").unwrap().as_array().unwrap()[2];
    assert_eq!(color_input.get("kind").unwrap().as_str().unwrap(), "input");
    assert_eq!(color_input.get("shape").unwrap().as_str().unwrap(), "color");
    assert_eq!(color_input.get("value").unwrap().as_str().unwrap(), "#ff4136");
}

#[test]
fn test_all_blocks_list_and_backdrop_lines_parse_to_known_blocks() {
    let input = r#"{
        "code": "when backdrop switches to [backdrop1 v]\ninsert [thing] at (1) of [list v]\nreplace item (1) of [list v] with [thing]",
        "language": "en",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let ids: Vec<&str> = blocks
        .iter()
        .map(|block| block.get("id").unwrap().as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        vec![
            "EVENT_WHENBACKDROPSWITCHESTO",
            "DATA_INSERTATLIST",
            "DATA_REPLACEITEMOFLIST"
        ]
    );
}

#[test]
fn test_all_blocks_list_render_contains_all_labels() {
    let input = r#"{
        "code": "insert [thing] at (1) of [list v]\nreplace item (1) of [list v] with [thing]",
        "language": "en",
        "inline": false
    }"#;
    let svg = render_request_json(input).expect("render failed");
    for expected in ["insert", "thing", "at", "replace", "item", "of", "list", "with"] {
        assert!(
            svg.contains(expected),
            "rendered SVG does not contain expected text {expected:?}:\n{svg}"
        );
    }
}

#[test]
fn test_debug_square_dropdown_parse() {
    let input = r#"{
        "code": "when [space v] key pressed",
        "language": "en",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let block = &blocks[0];
    let id = block.get("id").unwrap().as_str().unwrap();
    println!("Block ID: {id}");
    println!("Full parse output: {output}");
}

#[test]
fn test_square_dropdowns() {
    // field_dropdown blocks should render square dropdowns (rx=0) with same fill as block
    // "when [space v] key pressed" has field_dropdown KEY_OPTION
    let input = r#"{
        "code": "when [space v] key pressed",
        "language": "en",
        "inline": false
    }"#;
    let svg = render_request_json(input).expect("render failed");
    // Events fill is #FFBF00, alt is #E6AC00
    // Square dropdown uses fill=#FFBF00, round uses fill=#E6AC00
    assert!(!svg.contains("rx=\"16\""), "Events key dropdown should be square (rx=0), but found rx=16:\n{svg}");
    assert!(svg.contains("fill=\"#FFBF00\""), "Events key dropdown should have fill #FFBF00 (block color):\n\nSVG:\n{svg}");
    assert!(!svg.contains("fill=\"#E6AC00\""), "Events key dropdown should NOT use alt fill #E6AC00:\n\nSVG:\n{svg}");
}

#[test]
fn test_round_dropdowns() {
    // With the [XXX v] = square, (XXX v) = round convention,
    // round dropdowns must be written with round brackets: (pop v)
    let input = r#"{
        "code": "play sound (pop v)",
        "language": "en",
        "inline": false
    }"#;
    let svg = render_request_json(input).expect("render failed");
    // Sound fill is #CF63CF, alt is #C94FC9
    assert!(svg.contains("rx=\"16\""), "Sound dropdown (round brackets) should be round (rx=16):\n\nSVG:\n{svg}");
    assert!(svg.contains("fill=\"#C94FC9\""), "Sound dropdown should use alt fill #C94FC9:\n\nSVG:\n{svg}");
}

#[test]
fn test_square_bracket_sound_dropdown() {
    // [pop v] with square brackets should render SQUARE (field-dropdown style)
    let input = r#"{
        "code": "play sound [pop v]",
        "language": "en",
        "inline": false
    }"#;
    let svg = render_request_json(input).expect("render failed");
    // Square dropdown: no rx=16, uses block fill #CF63CF (not alt #C94FC9)
    assert!(!svg.contains("rx=\"16\""), "Sound dropdown with [v] should be square (no rx=16):\n\nSVG:\n{svg}");
}

#[test]
fn test_nested_boolean_or_parsing() {
    // << > or < >> should parse as boolean OR with two empty predicates,
    // NOT as a single boolean with labels "< > or < >"
    use scratchblocks_wasm::parse_request_json;
    let input = r#"{
        "code": "<< > or < >>",
        "language": "en",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let block = &blocks[0];
    assert_eq!(block.get("id").unwrap(), "OPERATORS_OR",
        "Should parse as OPERATORS_OR, got: {}", block.get("id").unwrap());
    // Should have 3 parts: boolean-input, label "or", boolean-input
    let parts = block.get("parts").unwrap().as_array().unwrap();
    assert_eq!(parts.len(), 3, "Should have 3 parts: boolean + or + boolean");
    assert_eq!(parts[0].get("kind").unwrap(), "input");
    assert_eq!(parts[0].get("shape").unwrap(), "boolean");
    assert_eq!(parts[1].get("kind").unwrap(), "label");
    assert_eq!(parts[1].get("value").unwrap(), "or");
    assert_eq!(parts[2].get("kind").unwrap(), "input");
    assert_eq!(parts[2].get("shape").unwrap(), "boolean");
}

#[test]
fn test_nested_boolean_and_parsing() {
    use scratchblocks_wasm::parse_request_json;
    let input = r#"{
        "code": "<< > and < >>",
        "language": "en",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let block = &blocks[0];
    assert_eq!(block.get("id").unwrap(), "OPERATORS_AND",
        "Should parse as OPERATORS_AND");
    let parts = block.get("parts").unwrap().as_array().unwrap();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].get("shape").unwrap(), "boolean");
    assert_eq!(parts[1].get("value").unwrap(), "and");
    assert_eq!(parts[2].get("shape").unwrap(), "boolean");
}

#[test]
fn test_infix_greater_than_inside_predicate() {
    // <(0) > (50) > should parse as OPERATORS_GT
    use scratchblocks_wasm::parse_request_json;
    let input = r#"{
        "code": "<(0) > (50)>",
        "language": "en",
        "inline": false
    }"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    let blocks = parsed.as_array().unwrap();
    let block = &blocks[0];
    assert_eq!(block.get("id").unwrap(), "OPERATORS_GT",
        "Should parse as OPERATORS_GT, got: {}", block.get("id").unwrap());
    let parts = block.get("parts").unwrap().as_array().unwrap();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].get("shape").unwrap(), "number");
    assert_eq!(parts[1].get("value").unwrap(), ">");
    assert_eq!(parts[2].get("shape").unwrap(), "number");
}

#[test]
fn test_german_block_parsing() {
    // German control blocks with official Scratch 3 names
    use scratchblocks_wasm::parse_request_json;
    
    // Repeat block
    let input = r#"{"code":"wiederhole (4) mal\n  sage [Hallo]\nende","language":"de","inline":false}"#;
    let output = parse_request_json(input).expect("de repeat parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "CONTROL_REPEAT");
    
    // Forever block
    let input = r#"{"code":"wiederhole fortlaufend\nende","language":"de","inline":false}"#;
    let output = parse_request_json(input).expect("de forever parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "CONTROL_FOREVER");
    
    // Motion block
    let input = r#"{"code":"gehe (10) er Schritt","language":"de","inline":false}"#;
    let output = parse_request_json(input).expect("de motion parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "MOTION_MOVESTEPS");
}

#[test]
fn test_french_block_parsing() {
    // French control blocks with official Scratch 3 names
    use scratchblocks_wasm::parse_request_json;
    
    // If-then-else block
    let input = r#"{"code":"si <> alors\n  dire [Bonjour]\nsinon\n  dire [Au revoir]\nfin","language":"fr","inline":false}"#;
    let output = parse_request_json(input).expect("fr if-else parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "CONTROL_IF");
    assert!(parsed[0].get("else-body").and_then(|b| b.as_array()).map_or(false, |b| !b.is_empty()),
        "French if-else should have else-body");
    
    // Repeat block
    let input = r#"{"code":"répéter (4) fois\n  dire [Salut]\nfin","language":"fr","inline":false}"#;
    let output = parse_request_json(input).expect("fr repeat parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "CONTROL_REPEAT");
    
    // Motion block
    let input = r#"{"code":"avancer de (10) pas","language":"fr","inline":false}"#;
    let output = parse_request_json(input).expect("fr motion parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "MOTION_MOVESTEPS");
}

#[test]
fn test_procedure_definition() {
    // define myBlock should be recognized as define-hat
    use scratchblocks_wasm::render_request_json;
    let input = r#"{"code":"define my block\nsay [Hello]","language":"en","inline":false}"#;
    let svg = render_request_json(input).expect("procedure render failed");
    assert!(svg.contains("my"), "SVG should contain 'my':\n{svg}");
}

#[test]
fn test_call_block() {
    // "call" prefix should be recognized as custom stack block
    use scratchblocks_wasm::render_request_json;
    let input = r#"{"code":"call my block","language":"en","inline":false}"#;
    let svg = render_request_json(input).expect("call render failed");
    assert!(svg.contains("call"), "SVG should contain 'call':\n{svg}");
}

#[test]
fn test_custom_arg_reporter() {
    // (var [myVar]) from SB3 import should render
    use scratchblocks_wasm::render_request_json;
    let input = r#"{"code":"set [myVar v] to (var [myVar])","language":"en","inline":false}"#;
    let svg = render_request_json(input).expect("custom-arg render failed");
    assert!(svg.contains("myVar"), "SVG should contain 'myVar':\n{svg}");
}

#[test]
fn test_category_suffix_french_list_block() {
    // ajouter (12) à [add v] ::list should be recognized as DATA_ADDTOLIST
    use scratchblocks_wasm::parse_request_json;
    let input = r#"{"code":"ajouter (12) à [add v] ::list","language":"fr","inline":false}"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "DATA_ADDTOLIST",
        "Should be DATA_ADDTOLIST, got: {}", parsed[0].get("id").unwrap());
}

#[test]
fn test_category_suffix_french_variable_block() {
    // mettre [myVar v] à (10) ::variable should be recognized as DATA_SETVARIABLETO
    use scratchblocks_wasm::parse_request_json;
    let input = r#"{"code":"mettre [myVar v] à (10) ::variable","language":"fr","inline":false}"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "DATA_SETVARIABLETO",
        "Should be DATA_SETVARIABLETO, got: {}", parsed[0].get("id").unwrap());
}

#[test]
fn test_category_suffix_french_variable_change_dropdown_shape() {
    // [i v] must be parsed as square dropdown input (shape "dropdown"), not round number-dropdown.
    use scratchblocks_wasm::parse_request_json;
    let input = r#"{"code":"ajouter (5) à [i v] ::variable","language":"fr","inline":false}"#;
    let output = parse_request_json(input).expect("parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

    let block = &parsed[0];
    assert_eq!(block.get("id").unwrap(), "DATA_CHANGEVARIABLEBY",
        "Should be DATA_CHANGEVARIABLEBY, got: {}", block.get("id").unwrap());

    let parts = block.get("parts").and_then(|p| p.as_array()).expect("parts should be an array");
    // Structure for "ajouter %1 à %2": label, input, label, input
    let var_input = &parts[3];
    assert_eq!(var_input.get("kind").unwrap(), "input");
    assert_eq!(var_input.get("shape").unwrap(), "dropdown",
        "[i v] should parse as square dropdown input, got: {}", var_input.get("shape").unwrap());
}

#[test]
fn test_french_when_flag_block_variants() {
    use scratchblocks_wasm::parse_request_json;

    let canonical = r#"{"code":"quand @greenFlag est cliqué","language":"fr","inline":false}"#;
    let output = parse_request_json(canonical).expect("fr when-flag canonical parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "EVENT_WHENFLAGCLICKED",
        "Canonical fr when-flag should match EVENT_WHENFLAGCLICKED, got: {}", parsed[0].get("id").unwrap());

    let alias = r#"{"code":"quand le drapeau vert pressé","language":"fr","inline":false}"#;
    let output = parse_request_json(alias).expect("fr when-flag alias parse failed");
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed[0].get("id").unwrap(), "EVENT_WHENFLAGCLICKED",
        "Alias fr when-flag should match EVENT_WHENFLAGCLICKED, got: {}", parsed[0].get("id").unwrap());
}
