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

/// An operator between two values is Blockly's compare block, and it hangs
/// off the right edge of the `falls` row as an external input.
#[test]
fn a_compare_block_hangs_off_the_if_row() {
    let node = &parse("falls <(1) = (2)>\n  gehe (3) Schritte\nende", "jwinf")[0];
    assert_eq!(node["id"], "controls_if", "{node}");
    let svg = render("falls <(1) = (2)>\n  gehe (3) Schritte\nende", "jwinf");
    assert!(svg.contains(">= ▾<"), "operator dropdown expected:\n{svg}");
    assert!(svg.contains("#3950a5"), "shadow numbers in the mathe colour expected:\n{svg}");
}

// ---------------------------------------------------------------------------
// Against the editor. The fragments below are copied from the outlines the
// running jwinf.de editor draws (tests/fixtures/jwinf-reference.json); text
// widths differ between the editor's font and ours, so what is checked is
// everything that is not a text width: row heights, notches, sockets, the
// foot and mouth of a C-block, the corners.
// ---------------------------------------------------------------------------

fn outline(code: &str) -> String {
    let svg = render(code, "jwinf");
    let body = svg.split("</defs>").nth(1).expect("defs");
    let mut paths = body.split("<path d=\"").skip(1).map(|p| p.split('"').next().unwrap().to_string());
    // With the bevel on, the first path is the shadow copy; the outline is the second.
    paths.nth(1).expect("outline")
}

/// gehe 1 Schritte: `m 0,8 A 8,8 0 0,1 8,0 H 15 l 6,4 3,0 6,-4 H 130.5 v 25 H 29.5 l -6,4 -3,0 -6,-4 H 8 a 8,8 0 0,1 -8,-8 z`
#[test]
fn a_field_row_is_25_high_with_square_right_corners() {
    let d = outline("gehe (3) Schritte");
    assert!(d.starts_with("m 0,8 A 8,8 0 0,1 8,0 H 15 l 6,4 3,0 6,-4 H "), "{d}");
    assert!(d.contains(" v 25 H 29.5 l -6,4 -3,0 -6,-4 H 8 a 8,8 0 0,1 -8,-8 z"), "{d}");
}

/// The start block: `m 0,8 A 8,8 0 0,1 8,0 H 174 v 24 H 29.5 …` — no hat, no top notch, a label-only row of 24.
#[test]
fn the_start_block_has_no_hat_and_a_24_row() {
    let d = outline("Roboter-Programm");
    assert!(d.starts_with("m 0,8 A 8,8 0 0,1 8,0 H "), "{d}");
    assert!(!d.contains("l 6,4 3,0 6,-4 H"), "no top notch: {d}");
    assert!(d.contains(" v 24 H 29.5 l -6,4 -3,0 -6,-4"), "{d}");
    assert!(render("Roboter-Programm", "jwinf").contains("#4789cc"), "start blocks are blue");
}

/// wiederhole 10 Mal: — `… v 36 H 94 l -6,4 -3,0 -6,-4 h -7 a 8,8 0 0,0 -8,8 v 9 a 8,8 0 0,0 8,8 H 94 v 10 H 29.5 …`
/// plus the inline socket `M 135,5 h -38 v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 7 h 38 z`.
#[test]
fn a_c_block_has_a_36_header_a_25_mouth_and_a_10_foot() {
    let d = outline("wiederhole (10) mal:\nende");
    assert!(d.contains(" v 36 H "), "inline header row: {d}");
    assert!(d.contains(" l -6,4 -3,0 -6,-4 h -7 a 8,8 0 0,0 -8,8 v 9 a 8,8 0 0,0 8,8 H "), "empty mouth: {d}");
    assert!(d.contains(" v 10 H 29.5 l -6,4 -3,0 -6,-4 H 8 a 8,8 0 0,1 -8,-8 z"), "foot: {d}");
    assert!(d.contains(" v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 7 h "), "socket around the shadow number: {d}");
}

/// falls ? mache ? — the condition is an external socket: `H 94 v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 4 H 94 l -6,4 …`
#[test]
fn an_if_cuts_its_condition_into_the_right_edge() {
    let d = outline("falls < >\n  gehe (3) Schritte\nende");
    assert!(d.contains(" v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 4 H "), "external socket, 24 row: {d}");
}

/// falls ? mache ? sonst ? — a 10 arm between two mouths.
#[test]
fn an_else_is_a_10_arm_and_a_second_mouth() {
    let d = outline("falls < >\n  gehe (3) Schritte\nsonst\n  gehe (3) Schritte\nende");
    assert_eq!(d.matches(" h -7 a 8,8 0 0,0 -8,8 v ").count(), 2, "two mouths: {d}");
    assert!(d.contains("a 8,8 0 0,0 8,8 H ") && d.matches(" v 10 H ").count() == 2, "arm and foot: {d}");
}

/// nicht ? — `m 0,0 H 59 v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 4 H 0 V 20 c 0,-10 -8,8 -8,-7.5 s 8,2.5 8,-7.5 z`:
/// a value block is a square box with the output tab on its left, and its
/// own single value input is cut into the right edge.
#[test]
fn a_value_block_is_a_square_box_with_a_tab() {
    let d = outline("<nicht < >>");
    assert!(d.starts_with("m 0,0 H "), "{d}");
    assert!(d.ends_with(" v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 4 H 0 V 20 c 0,-10 -8,8 -8,-7.5 s 8,2.5 8,-7.5 z"), "{d}");
}

/// 1 + 1 — inline value block: row 36 with two sockets and the operator as a dropdown.
#[test]
fn an_arithmetic_block_is_an_inline_row_of_36() {
    let d = outline("((1) + (2))");
    assert!(d.contains(" v 36 H 0 V 20 c 0,-10"), "{d}");
    assert_eq!(d.matches("v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 7 h ").count(), 2, "two sockets: {d}");
    assert!(render("((1) + (2))", "jwinf").contains(">+ ▾<"), "operator dropdown");
}

/// Stacked: the second block's top-left corner is square, the first block's bottom-left too.
#[test]
fn stacked_blocks_share_square_corners_where_they_touch() {
    let svg = render("gehe (3) Schritte\ngehe (3) Schritte", "jwinf");
    let body = svg.split("</defs>").nth(1).unwrap();
    let outlines: Vec<&str> = body.split("<path d=\"").skip(1).map(|p| p.split('"').next().unwrap()).collect();
    // shadow, outline, shadow, outline
    assert!(outlines[1].ends_with(" H 0 z"), "first block, square bottom-left: {}", outlines[1]);
    assert!(outlines[3].starts_with("m 0,0 H 15 "), "second block, square top-left: {}", outlines[3]);
    assert!(outlines[3].ends_with(" H 8 a 8,8 0 0,1 -8,-8 z"), "second block, rounded bottom-left: {}", outlines[3]);
}
