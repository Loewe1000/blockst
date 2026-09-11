// The Blockly dialect and its jwinf profile: text goes in with the
// scratchblocks notation, comes out with Blockly blocks.

use scratchblocks_wasm::{parse_request_json, render_request_json};

fn parse(code: &str, profile: &str) -> serde_json::Value {
    let payload = serde_json::json!({
        "code": code, "language": "de", "inline": false, "profile": profile,
    });
    serde_json::from_str(&parse_request_json(&payload.to_string()).expect("parse failed")).unwrap()
}

fn render(code: &str, profile: &str) -> String {
    let payload = serde_json::json!({
        "code": code, "language": "de", "inline": false, "profile": profile,
    });
    render_request_json(&payload.to_string()).expect("render failed")
}

/// A standard Blockly block is recognised from Blockly's own German wording.
#[test]
fn a_standard_block_is_recognised_in_the_blockly_profile() {
    let node = &parse("wiederhole (10)-mal:\nende", "blockly-modern")[0];
    assert_eq!(node["id"], "controls_repeat_ext");
    assert_eq!(node["category"], "schleifen");
}

/// jwinf keeps the pre-2019 wording — "wiederhole %1 mal:" without the
/// hyphen — and its own locale must win over the base for that block.
#[test]
fn jwinf_uses_the_old_wording_for_standard_blocks() {
    let node = &parse("wiederhole (10) mal:\nende", "jwinf")[0];
    assert_eq!(node["id"], "controls_repeat_ext", "{node}");
    assert_eq!(node["category"], "schleifen");
}

/// A world block comes from the France-IOI catalog.
#[test]
fn a_world_block_is_recognised_in_the_jwinf_profile() {
    let node = &parse("gehe nach rechts", "jwinf")[0];
    assert_eq!(node["id"], "east");
    assert_eq!(node["category"], "aktionen");
}

/// And a standard block is still there through the base locale.
#[test]
fn jwinf_inherits_the_standard_blocks() {
    let node = &parse("setze [x v] auf (7)", "jwinf")[0];
    assert_eq!(node["id"], "variables_set", "{node}");
    assert_eq!(node["category"], "variablen");
}

/// Free text is the normal case on jwinf, because block labels change from
/// task to task. It renders with the category the author names.
#[test]
fn free_text_takes_the_category_from_the_suffix() {
    let node = &parse("hebe Murmel auf ::aktionen", "jwinf")[0];
    assert_eq!(node["category"], "aktionen", "{node}");
}

/// A jwinf document must not pick up Scratch's English vocabulary: the
/// dialect chain ends at its base locale, not at Scratch's English one.
#[test]
fn scratch_english_does_not_leak_into_jwinf() {
    let node = &parse("move (10) steps", "jwinf")[0];
    assert_ne!(node["id"], "MOTION_MOVESTEPS", "{node}");
}

/// The C-block closes with the German keyword and its body is nested.
#[test]
fn ende_closes_a_c_block() {
    let nodes = parse("wiederhole (4) mal:\n  gehe nach rechts\nende\ngehe nach links", "jwinf");
    assert_eq!(nodes.as_array().unwrap().len(), 2, "{nodes}");
    assert_eq!(nodes[0]["body"][0]["id"], "east");
    assert_eq!(nodes[1]["id"], "west");
}

/// Rendering goes through the Blockly outlines, not Scratch's.
#[test]
fn jwinf_renders_with_blockly_outlines() {
    let svg = render("gehe nach rechts", "jwinf");
    assert!(svg.contains("l 6,4 3,0 6,-4"), "Blockly notch expected:\n{svg}");
    assert!(!svg.contains("c 2 0 3 1 4 2"), "Scratch notch must not appear:\n{svg}");
}

/// Scratch is untouched: no profile means the Scratch locale and shapes.
#[test]
fn scratch_still_parses_without_a_profile() {
    let payload = serde_json::json!({ "code": "gehe (10) er Schritt", "language": "de", "inline": false });
    let nodes: serde_json::Value =
        serde_json::from_str(&parse_request_json(&payload.to_string()).unwrap()).unwrap();
    assert_eq!(nodes[0]["id"], "MOTION_MOVESTEPS");
}

/// The jwinf categories have colours of their own: nothing on a jwinf sheet
/// should come out in the grey that marks an unknown category.
#[test]
fn jwinf_categories_are_not_grey() {
    for code in ["gehe nach rechts", "wiederhole (4) mal:\nende", "setze [x v] auf (1)", "hebe Murmel auf ::aktionen"] {
        let svg = render(code, "jwinf");
        assert!(!svg.contains("#bfbfbf"), "grey fallback in {code}:\n{svg}");
    }
    assert!(render("gehe nach rechts", "jwinf").contains("#723ca5"), "Aktionen must be jwinf purple");
}

/// The mouth label Blockly draws beside a C-block's arm.
#[test]
fn a_c_block_carries_its_mouth_label() {
    let svg = render("wiederhole (4) mal:\n  gehe nach rechts\nende", "jwinf");
    assert!(svg.contains(">mache<"), "arm label expected:\n{svg}");
}

/// Greyscale keeps jwinf's categories apart, whatever their colours.
#[test]
fn jwinf_greyscale_steps_differ_between_categories() {
    let payload = |code: &str| serde_json::json!({
        "code": code, "language": "de", "inline": false, "profile": "jwinf", "theme": "grayscale",
    });
    // The block outline is the first <path d=…> after the defs; its fill is
    // the category colour. Earlier fills belong to the icon definitions.
    let fill = |svg: &str| {
        let body = svg.split("</defs>").nth(1).expect("defs");
        let outline = body.split("<path d=").nth(1).expect("outline");
        outline.split("fill=\"#").nth(1).unwrap()[..6].to_string()
    };
    let a = fill(&render_request_json(&payload("gehe nach rechts").to_string()).unwrap());
    let b = fill(&render_request_json(&payload("wiederhole (4) mal:\nende").to_string()).unwrap());
    assert_ne!(a, b, "aktionen and schleifen must not share a grey");
}

/// `sonst` opens the else branch of a Blockly if, as `else` does for Scratch.
#[test]
fn sonst_opens_the_else_branch() {
    let nodes = parse("falls <auf Kiste>\n  gehe nach rechts\nsonst\n  gehe nach links\nende", "jwinf");
    assert_eq!(nodes.as_array().unwrap().len(), 1, "{nodes}");
    assert_eq!(nodes[0]["body"][0]["id"], "east");
    assert_eq!(nodes[0]["else-body"][0]["id"], "west", "{nodes}");
}
