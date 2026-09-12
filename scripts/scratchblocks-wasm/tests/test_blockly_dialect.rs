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

// ---------------------------------------------------------------------------
// The bevel highlight, against the editor's svgPathLight_ (fixture key
// "highlights"). Blockly lights only the edges that face up and left: the top
// edge with its notch, a glint at the foot of every tab, the floor of each
// mouth, and the left edge with its corners — never the right or the bottom.
// ---------------------------------------------------------------------------

fn highlight(code: &str) -> String {
    let svg = render(code, "jwinf");
    let body = svg.split("</defs>").nth(1).expect("defs");
    let mut paths = body.split("<path d=\"").skip(1).map(|p| p.split('"').next().unwrap().to_string());
    // shadow copy, outline, highlight
    paths.nth(2).expect("highlight")
}

/// gehe 1 Schritte: `m 0.5,7.5 A 7.5,7.5 0 0,1 8,0.5 H 15 l 6,4 3,0 6,-4 H 130 … M 2.6967,22.3033 A 7.5,7.5 0 0,1 0.5,17 V 8`
#[test]
fn the_highlight_follows_the_notch_and_stops_at_the_rounded_corners() {
    let h = highlight("gehe (3) Schritte");
    assert!(h.starts_with("m 0.5,7.5 A 7.5,7.5 0 0,1 8,0.5 H 15 l 6,4 3,0 6,-4 H "), "{h}");
    assert!(h.ends_with(" M 2.6966991,22.3033 A 7.5,7.5 0 0,1 0.5,17 V 8"), "{h}");
    assert!(!h.contains(" v 25") && !h.contains("V 25"), "no light on the right edge: {h}");
}

/// setze x auf 0: `… H 123.73 M 119.23,19.3 l 3.68,-2.1 M 2.6967,22.3033 …` — a glint at the foot of the external tab.
#[test]
fn an_external_tab_gets_a_glint_at_its_foot() {
    let h = highlight("setze [x v] auf (0)");
    assert!(h.contains(",19.3 l 3.68,-2.1 M 2.6966991,22.3033 A 7.5,7.5 0 0,1 0.5,17 V 8"), "{h}");
}

/// math_number: `m 0.5,0.5 H 27.66 M 0.5,24.5 V 18.5 m -7.36,-0.5 q -1.52,-5.5 0,-11 m 7.36,1 V 0.5 H 1`
#[test]
fn a_value_block_is_lit_along_the_back_of_its_tab() {
    let svg = render("((1) + (2))", "jwinf");
    let body = svg.split("</defs>").nth(1).unwrap();
    let paths: Vec<&str> = body.split("<path d=\"").skip(1).map(|p| p.split('"').next().unwrap()).collect();
    // operator: dark, fill, light; first number: dark, fill, light
    let number_light = paths[5];
    assert_eq!(number_light, "m 0.5,0.5 H 28.4 M 0.5,24.5 V 18.5 m -7.36,-0.5 q -1.52,-5.5 0,-11 m 7.36,1 V 0.5 H 1");
    // the operator's inline sockets: right wall, floor, glint
    assert!(paths[2].contains(",5.5 v 27 h -"), "{}", paths[2]);
    assert!(paths[2].contains(",24.3 l 3.68,-2.1"), "{}", paths[2]);
}

/// wiederhole 4 Mal: mache [gehe 1 Schritte] — outline `… v 14 a 8,8 0 0,0 8,8 …`, light `M 66.01,64.01 a 8.5,8.5 0 0,0 6.01,2.49 H 93.52`
#[test]
fn a_mouth_is_lit_along_its_floor_and_holds_a_child_with_five_pixels_to_spare() {
    let code = "wiederhole (4) mal:\n  gehe (1) Schritte\nende";
    let d = outline(code);
    assert!(d.contains(" h -7 a 8,8 0 0,0 -8,8 v 14 a 8,8 0 0,0 8,8 H "), "{d}");
    let h = highlight(code);
    assert!(h.contains(" a 8.5,8.5 0 0,0 6.0104074,2.4895926 H "), "{h}");
}

/// falls … sonst …: `… H 94.02 v 5 tab v 4 H 94.02 l -6,4 … H 94.02 v 10 H 94.02 l -6,4 … H 94.02 v 10 H 29.5 …` — the same edge everywhere.
#[test]
fn every_mouth_of_a_block_shares_one_statement_edge_and_a_full_width_arm() {
    let d = outline("falls <auf Kiste>\n  gehe nach rechts\nsonst\n  drehe nach links\nende");
    let edges: Vec<&str> = d.split(" H ").skip(1).map(|s| s.split(' ').next().unwrap()).collect();
    // notch start, then: top, below the tab, foot 1, mouth 2 notch, foot 2, bottom notch, corner
    assert!(edges.len() >= 8, "{d}");
    let right = edges[1];
    for e in &edges[1..6] {
        assert_eq!(*e, right, "{d}");
    }
}

// ---------------------------------------------------------------------------
// Colours per task family: the jwinf-turtle profile, and a document's own
// overrides on top of any profile.
// ---------------------------------------------------------------------------

fn render_with(code: &str, profile: &str, colors: serde_json::Value, theme: &str) -> String {
    let payload = serde_json::json!({
        "code": code, "language": "de", "inline": false, "profile": profile, "colors": colors, "theme": theme,
    });
    render_request_json(&payload.to_string()).expect("render failed")
}

/// The turtle sandbox colours loops #47cccc where the robot tasks use #2fb5bd; the rest is inherited from jwinf.
#[test]
fn the_turtle_profile_recolours_three_categories_and_inherits_the_rest() {
    let turtle = render("wiederhole (4) mal:\nende", "jwinf-turtle");
    assert!(turtle.contains("#47cccc"), "{turtle}");
    let robot = render("wiederhole (4) mal:\nende", "jwinf");
    assert!(robot.contains("#2fb5bd") && !robot.contains("#47cccc"));
    let vars = render("setze [x v] auf (0)", "jwinf-turtle");
    assert!(vars.contains("#a5416b"), "variables keep jwinf's colour: {vars}");
    let d = outline("gehe (3) Schritte");
    let d_turtle = {
        let svg = render("gehe (3) Schritte", "jwinf-turtle");
        svg.split("</defs>").nth(1).unwrap().split("<path d=\"").nth(2).unwrap().split('"').next().unwrap().to_string()
    };
    assert_eq!(d, d_turtle, "same classic geometry");
}

/// `colors: (logik: "#73cc47")` replaces one category of the profile's palette.
#[test]
fn a_document_can_override_a_category_colour() {
    let svg = render_with("falls <auf Kiste>\nende", "jwinf", serde_json::json!({"logik": "#73cc47"}), "normal");
    assert!(svg.contains("#73cc47") && !svg.contains("#81b31d"), "{svg}");
    // the bevel shades are derived from the new fill, not the old
    assert!(svg.contains("#5ca339"), "dark copy of #73cc47: {svg}");
}

/// The override takes part in the greyscale ranking like any profile colour.
#[test]
fn an_override_is_drawn_in_grayscale_too() {
    let svg = render_with("falls <auf Kiste>\nende", "jwinf", serde_json::json!({"logik": "#73cc47"}), "grayscale");
    assert!(!svg.contains("#73cc47") && !svg.contains("#81b31d"), "{svg}");
}

/// Scratch documents can override too; the untouched categories keep their tables.
#[test]
fn scratch_takes_overrides_as_well() {
    let payload = serde_json::json!({
        "code": "move (10) steps\nsay [hi]", "language": "en", "inline": false, "profile": "scratch", "colors": {"motion": "#123456"},
    });
    let svg = render_request_json(&payload.to_string()).unwrap();
    assert!(svg.contains("#123456"), "{svg}");
    assert!(svg.contains("#9966ff") || svg.contains("#9966FF"), "looks keeps its colour: {svg}");
}


// ---------------------------------------------------------------------------
// The default profile, against Blockly 12.3.1's thrasos renderer
// (tests/fixtures/blockly-modern-reference.json). Same idea as above: what
// is checked is everything that is not a text width.
// ---------------------------------------------------------------------------

fn modern_outline(code: &str) -> String {
    let svg = render(code, "blockly");
    let body = svg.split("</defs>").nth(1).expect("defs");
    body.split("<path d=\"").nth(1).unwrap().split('"').next().unwrap().to_string()
}

/// set x to 0: `m 0,8 a 8 8 0 0,1 8,-8 h 7 l 6,4 3,0 6,-4 h 70.5 v 5 V 5 H 100.5 tab v 3 V 23 V 28 h -70.5 l -6,4 -3,0 -6,-4 h -7 a 8 8 0 0,1 -8,-8 z`
#[test]
fn modern_field_row_is_5_18_5_with_the_tab_at_the_row_top() {
    let d = modern_outline("setze [x v] auf (0)");
    assert!(d.starts_with("m 0,8 a 8 8 0 0,1 8,-8 H 15 l 6,4 3,0 6,-4 H "), "{d}");
    assert!(d.contains(" V 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 3 V 28 H 30 l -6,4 -3,0 -6,-4 H 8 a 8 8 0 0,1 -8,-8 z"), "{d}");
    let svg = render("setze [x v] auf (0)", "blockly");
    // the number block starts at the parent's right edge, its own top row above the parent's row
    let edge = d.split(" H ").nth(2).unwrap().split(' ').next().unwrap();
    assert!(svg.contains(&format!("<g transform=\"translate({edge} 0)\">")), "{svg}");
    assert!(svg.contains("stroke=\"#c08ca6\""), "tertiary stroke: {svg}");
}

/// math_number: `m 8,0 h 28.15 v 5 V 5 V 23 V 23 V 28 h -28.15 H 8 V 20 tab z` — square, 28 high, the box 18 tall at y 5.
#[test]
fn modern_value_block_is_square_and_28_high() {
    let svg = render("((1) + (2))", "blockly");
    let body = svg.split("</defs>").nth(1).unwrap();
    let paths: Vec<&str> = body.split("<path d=\"").skip(1).map(|p| p.split('"').next().unwrap()).collect();
    assert!(paths[1].ends_with(" V 28 H 0 V 20 c 0,-10 -8,8 -8,-7.5 s 8,2.5 8,-7.5 z"), "{}", paths[1]);
    assert!(svg.contains("<rect x=\"5\" y=\"5\" width=\"18.9\" height=\"18\" rx=\"4\""), "{svg}");
    // the arithmetic block: 38 high, sockets 28 tall cut in at y 5, the first one 16 in
    assert!(paths[0].contains(" V 38 H 0 V 20 "), "{}", paths[0]);
    assert!(paths[0].contains(" M 16,5 v 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 8 h "), "{}", paths[0]);
    assert!(paths[0].contains(" v -28 z"), "{}", paths[0]);
    assert!(svg.contains("<g transform=\"translate(16 5)\">"), "first child at the cut-out: {svg}");
}

/// if (1 = 2) do […]: `… v 5 V 5 H 66.3 tab v 13 V 37 H 66.3 notch h -7 a 8 8 0 0,0 -8,8 v 16 a 8 8 0 0,0 8,8 H 66.3 V 69 V 79 …`
#[test]
fn modern_if_row_grows_with_its_condition_and_the_mouth_with_its_child() {
    let d = modern_outline("falls <(1) = (2)>\n  setze [x v] auf (7)\nende");
    assert!(d.contains(" V 5 c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5 v 13 V 37 H "), "row 28 = child 38 - 10, then a 4 spacer: {d}");
    assert!(d.contains(" l -6,4 -3,0 -6,-4 h -7 a 8 8 0 0,0 -8,8 v 16 a 8 8 0 0,0 8,8 H "), "mouth 32 = child 28 + notch: {d}");
    assert!(d.contains(" V 79 H 30 l -6,4 -3,0 -6,-4 H 8 a 8 8 0 0,1 -8,-8 z"), "10 under the mouth: {d}");
}

/// if [not ?] do ? else ?: rows 5 | 18 | 4 | 24 | 10 | 24 | 10, one statement edge.
#[test]
fn modern_else_has_a_10_spacer_between_the_mouths() {
    let d = modern_outline("falls <nicht <>>\nsonst\nende");
    assert!(d.contains(" v 3 V 27 H "), "{d}");
    assert!(d.contains(" v 8 a 8 8 0 0,0 8,8 H "), "empty mouth 24: {d}");
    assert!(d.contains(" V 61 H "), "{d}");
    assert!(d.contains(" V 95 H 30 l -6,4"), "61 + 24 + 10: {d}");
    let edges: Vec<&str> = d.split(" H ").skip(1).map(|s| s.split(' ').next().unwrap()).collect();
    assert_eq!(edges[1], edges[2], "{d}");
    assert_eq!(edges[2], edges[4], "{d}");
}

/// repeat ? times do [print, print]: an empty socket makes a 26 row; two children stack at 28 apart, mouth 60.
#[test]
fn modern_stacks_sit_28_apart_and_an_empty_socket_row_is_26() {
    let code = "wiederhole ()-mal:\n  setze [x v] auf (1)\n  setze [x v] auf (2)\nende";
    let d = modern_outline(code);
    assert!(d.contains(" V 35 H "), "5 + 26 + 4: {d}");
    assert!(d.contains(" v 44 a 8 8 0 0,0 8,8 H "), "60 - 16: {d}");
    let svg = render(code, "blockly");
    assert!(svg.contains("<g transform=\"translate(0 28)\">"), "{svg}");
    assert!(svg.contains(" v 6 h 14.5 v -26 z"), "empty socket: {d}");
}


// ---------------------------------------------------------------------------
// Icons: the mutator gear on falls and on procedure definitions, drawn as
// the editor draws it and pushing the label to x = 37 in both generations.
// ---------------------------------------------------------------------------

#[test]
fn falls_carries_the_mutator_gear_in_both_generations() {
    for profile in ["jwinf", "blockly"] {
        let svg = render("falls <auf Kiste>\nende", profile);
        assert!(svg.contains("opacity=\"0.6\""), "{profile}: icon group: {svg}");
        assert!(svg.contains("d=\"m4.203,7.296 "), "{profile}: gear symbol: {svg}");
        assert!(svg.contains("<text class=\"sb-label\" x=\"37\""), "{profile}: label after the icon: {svg}");
    }
    let classic = render("falls <auf Kiste>\nende", "jwinf");
    assert!(classic.contains("<g transform=\"translate(10 5)\" opacity"), "classic icon at (10,5): {classic}");
    let modern = render("falls <auf Kiste>\nende", "blockly");
    assert!(modern.contains("<g transform=\"translate(10 5.5)\" opacity"), "modern icon centred in the 18 row: {modern}");
}

#[test]
fn a_plain_block_has_no_icon() {
    let svg = render("gehe (3) Schritte", "jwinf");
    assert!(!svg.contains("opacity=\"0.6\""), "{svg}");
}
