//! Geometry cross-check against `examples/nepo/references/blockly-reference.mjs`.
//!
//! Both sides are transcriptions of Open Roberta's `block_render_svg.js`, made
//! separately, in different languages, from different descriptions of the same
//! blocks — the JavaScript reads the official `blocks/*.js`, the Rust reads the
//! catalog in `data/blocks.toml`. Agreement therefore says something: it means
//! the outline, the connection positions and the catalog all match the source.
//!
//! Text widths are supplied by `fixtures.json` rather than measured, so a
//! failure here is always a geometry failure and never a font difference.
//!
//! Regenerate the reference with:
//!   node examples/nepo/references/blockly-reference.mjs \
//!     > examples/nepo/references/reference.json

use nepo_wasm::measure::{self, width_key};
use nepo_wasm::{matrix, parser};
use std::collections::HashMap;

fn references_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/nepo/references")
}

fn load(name: &str) -> serde_json::Value {
    let path = references_dir().join(name);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

/// The image block's rulers print in Courier and everything else in the label
/// font, and label text reaches the renderer with its spaces turned into
/// non-breaking ones. Each stipulated width is therefore installed under all
/// four keys, so the fixture can be written with ordinary spaces.
fn install_widths(widths: &serde_json::Value) {
    let mut map = HashMap::new();
    for (text, width) in widths.as_object().expect("widths object") {
        let width = width.as_f64().expect("width number") as f32;
        let nbsp: String = text
            .chars()
            .map(|ch| if ch.is_whitespace() { '\u{00A0}' } else { ch })
            .collect();
        for variant in [text.as_str(), nbsp.as_str()] {
            map.insert(width_key(variant, false), width);
            map.insert(width_key(variant, true), width);
        }
    }
    measure::set_widths(map);
}

#[test]
fn geometry_matches_the_blockly_reference() {
    let fixtures = load("fixtures.json");
    let reference = load("reference.json");
    install_widths(&fixtures["widths"]);

    let mut checked = 0;
    for case in fixtures["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let source = case["source"].as_str().expect("source");
        let expected = match reference.get(id) {
            Some(value) => value,
            None => continue,
        };

        let scripts =
            parser::parse(source, "de", "calliope").unwrap_or_else(|err| panic!("{id}: {err}"));
        let mut blocks = scripts.into_iter().next().expect("one script");
        for block in blocks.iter_mut() {
            matrix::expand(block);
        }
        let layout = measure::layout_block(&blocks[0], false, false);

        let expect = |field: &str| expected[field].as_f64().expect("number") as f32;
        assert_eq!(layout.right_edge, expect("rightEdge"), "{id}: right edge");
        assert_eq!(
            layout.statement_edge,
            expect("statementEdge"),
            "{id}: statement edge"
        );
        assert_eq!(layout.bottom, expect("bottom"), "{id}: bottom");
        assert_eq!(layout.width, expect("width"), "{id}: width");
        checked += 1;
    }

    measure::clear_widths();
    assert_eq!(checked, 12, "every fixture should have a reference");
}

/// The numbers agreeing is not quite the same as the outline agreeing: a notch
/// drawn at the right x with the wrong profile would pass the check above.
#[test]
fn outline_matches_the_blockly_reference() {
    let fixtures = load("fixtures.json");
    let reference = load("reference.json");
    install_widths(&fixtures["widths"]);

    for case in fixtures["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let source = case["source"].as_str().expect("source");
        let expected = match reference.get(id) {
            Some(value) => value["d"].as_str().expect("d"),
            None => continue,
        };

        let request = serde_json::json!({
            "code": source,
            "language": "de",
            "platform": "calliope",
            "widths": widths_payload(&fixtures["widths"]),
        });
        let svg = parser::render_request(&request.to_string())
            .unwrap_or_else(|err| panic!("{id}: {err}"));

        // The outermost block's own outline is the first path emitted.
        let actual = first_path(&svg).unwrap_or_else(|| panic!("{id}: no path in output"));
        assert_eq!(
            normalise(&actual),
            normalise(expected),
            "{id}: outline differs
  ours: {actual}
  ref:  {expected}"
        );
    }

    measure::clear_widths();
}

fn widths_payload(widths: &serde_json::Value) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (text, width) in widths.as_object().expect("widths object") {
        let nbsp: String = text
            .chars()
            .map(|ch| if ch.is_whitespace() { '\u{00A0}' } else { ch })
            .collect();
        for variant in [text.as_str(), nbsp.as_str()] {
            map.insert(width_key(variant, false), width.clone());
            map.insert(width_key(variant, true), width.clone());
        }
    }
    serde_json::Value::Object(map)
}

fn first_path(svg: &str) -> Option<String> {
    let start = svg.find("<path d=\"")? + "<path d=\"".len();
    let end = svg[start..].find('"')? + start;
    Some(svg[start..end].to_string())
}

/// Compare path data by token, not by text: the two implementations format
/// numbers differently ("120" against "120.0") and separate commands with
/// different amounts of whitespace.
fn normalise(path: &str) -> Vec<String> {
    path.replace(',', " , ")
        .split_whitespace()
        .map(|token| match token.parse::<f64>() {
            Ok(number) => format!("{:.3}", number),
            Err(_) => token.to_string(),
        })
        .collect()
}
