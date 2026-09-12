// MakeCode: the micro:bit and Calliope mini editors' blocks, drawn with
// Blockly's zelos renderer. The fragments below are what the running editors
// draw (tests/fixtures/makecode-reference.json); text widths differ between
// the editors' Consolas and our measurement, so what is checked is everything
// that is not a text width.

use scratchblocks_wasm::{parse_request_json, render_request_json};

fn render(code: &str, profile: &str, language: &str) -> String {
    let payload = serde_json::json!({"code": code, "language": language, "inline": false, "profile": profile});
    render_request_json(&payload.to_string()).expect("render failed")
}

fn parse(code: &str, profile: &str, language: &str) -> serde_json::Value {
    let payload = serde_json::json!({"code": code, "language": language, "inline": false, "profile": profile});
    serde_json::from_str(&parse_request_json(&payload.to_string()).expect("parse failed")).unwrap()
}

/// Every `<path d="…">` in drawing order.
fn paths(svg: &str) -> Vec<String> {
    let body = svg.split("</defs>").nth(1).unwrap_or(svg);
    body.split("<path d=\"").skip(1).map(|p| p.split('"').next().unwrap().to_string()).collect()
}

fn outline(code: &str, profile: &str, language: &str) -> String {
    paths(&render(code, profile, language)).remove(0)
}

const NOTCH_LEFT: &str = "h 8 c 2,0 3,1 4,2 l 4,4 c 1,1 2,2 4,2 h 12 c 2,0 3,-1 4,-2 l 4,-4 c 1,-1 2,-2 4,-2";
const NOTCH_RIGHT: &str = "c -2,0 -3,1 -4,2 l -4,4 c -1,1 -2,2 -4,2 h -12 c -2,0 -3,-1 -4,-2 l -4,-4 c -1,-1 -2,-2 -4,-2 h -8";

// --- vocabulary ------------------------------------------------------------

#[test]
fn the_german_and_english_catalogs_name_the_same_block() {
    assert_eq!(parse("zeige Zahl (0)", "makecode", "de")[0]["id"], "device_show_number");
    assert_eq!(parse("show number (0)", "makecode", "en")[0]["id"], "device_show_number");
    assert_eq!(parse("wenn Knopf [A v] geklickt\nende", "makecode", "de")[0]["id"], "device_button_event");
    assert_eq!(parse("on button [A v] pressed\nend", "makecode", "en")[0]["id"], "device_button_event");
}

#[test]
fn the_calliope_profile_inherits_the_microbit_catalog_and_recolours_it() {
    let node = &parse("zeige Zahl (0)", "makecode-calliope", "de")[0];
    assert_eq!(node["id"], "device_show_number");
    let svg = render("zeige Zahl (0)", "makecode-calliope", "de");
    assert!(svg.contains("fill=\"#54c9c9\""), "Calliope's basic: {svg}");
    let svg = render("zeige Zahl (0)", "makecode", "de");
    assert!(svg.contains("fill=\"#1e90ff\""), "micro:bit's basic: {svg}");
}

#[test]
fn blockly_and_makecode_keep_their_own_definitions_of_shared_ids() {
    // controls_if exists in both dialects: Blockly's has a "mache" mouth label and a gear, MakeCode's neither.
    let mc = render("wenn <wahr> dann\nende", "makecode", "de");
    assert!(!mc.contains(">mache<"), "{mc}");
    assert!(!mc.contains("m4.203,7.296"), "no gear icon: {mc}");
    let bl = render("falls <wahr>\nende", "blockly", "de");
    assert!(bl.contains(">mache<"), "{bl}");
}

// --- geometry, against the editor ----------------------------------------------

/// zeige Zahl (0): `m 0,4 a 4 4 0 0,1 4,-4 h 8 [notch] … a 4 4 0 0,1 4,4 v 8 V 40 V 44 a 4 4 0 0,1 -4,4 h -… [notch] h -8 a 4 4 0 0,1 -4,-4 z`
#[test]
fn a_statement_block_is_48_high_with_the_notch_at_12() {
    let d = outline("zeige Zahl (0)", "makecode", "de");
    assert!(d.starts_with(&format!("m 0,4 a 4 4 0 0,1 4,-4 {NOTCH_LEFT} H ")), "{d}");
    assert!(d.contains(" a 4 4 0 0,1 4,4 V 40 V 44 a 4 4 0 0,1 -4,4 H 48 "), "rows 8 + 32 + 4 + 4: {d}");
    assert!(d.ends_with(&format!("{NOTCH_RIGHT} a 4 4 0 0,1 -4,-4 z")), "{d}");
    let svg = render("zeige Zahl (0)", "makecode", "de");
    // the number is a white pill, 40 x 32, at the row's top
    assert!(svg.contains("m 16,0 h 8 a 16 16 0 0,1 16,16 v 0 a 16 16 0 0,1 -16,16 V 32 h -8 a 16 16 0 0,1 -16,-16"), "{svg}");
    assert!(svg.contains("fill=\"#ffffff\" stroke=\"#bfbfbf\""), "shadow stroke: {svg}");
    assert!(svg.contains(" 8)\">"), "shadow at y = 8: {svg}");
}

/// beim Start: every number is text-independent, so the whole outline is compared.
#[test]
fn the_on_start_block_matches_the_editor_outline() {
    let d = outline("beim Start\nende", "makecode", "de");
    let expected = format!(
        "m 0,4 a 4 4 0 0,1 4,-4 H 156 a 4 4 0 0,1 4,4 V 40 V 44 a 4 4 0 0,1 -4,4 H 64 {NOTCH_RIGHT} a 4 4 0 0,0 -4,4 v 16 a 4 4 0 0,0 4,4 {NOTCH_LEFT} H 156 a 4 4 0 0,1 4,4 V 80 V 100 a 4 4 0 0,1 -4,4 H 4 a 4 4 0 0,1 -4,-4 z"
    );
    assert_eq!(d, expected);
    let svg = render("beim Start\nende", "makecode", "de");
    assert!(!svg.contains("c 25,-22"), "no hat: {svg}");
}

/// wenn Knopf [A v] geklickt: a 34px dropdown makes the row 34; statement child at (16, 50).
#[test]
fn an_event_with_a_dropdown_has_a_34_row() {
    let svg = render("wenn Knopf [A v] geklickt\n  Bildschirminhalt löschen\nende", "makecode", "de");
    let d = paths(&svg).remove(0);
    assert!(d.contains(" a 4 4 0 0,1 4,4 V 42 V 46 a 4 4 0 0,1 -4,4 H 64 "), "{d}");
    assert!(svg.contains("height=\"34\" rx=\"4\""), "{svg}");
    assert!(svg.contains("<g transform=\"translate(16 50)\">"), "{svg}");
}

/// (4) -mal wiederholen: the socket is bumped past the notch to x = 48; mouth child at (…, 48); bottom row 24 + notch.
#[test]
fn a_socket_at_the_row_start_is_bumped_to_48() {
    let svg = render("(4) -mal wiederholen\n  zeige Zahl (0)\nende", "makecode", "de");
    assert!(svg.contains("<g transform=\"translate(48 8)\">"), "{svg}");
    let d = paths(&svg).remove(0);
    assert!(d.contains(" v 40 a 4 4 0 0,0 4,4 "), "mouth 48 = child 56 - notch: {d}");
    assert!(d.contains(" a 4 4 0 0,1 4,4 V 104 V 124 a 4 4 0 0,1 -4,4 "), "24 under the mouth: {d}");
}

/// wenn <(1) = (2)> dann: a 42-high boolean in the first row pulls it up by 4 (tight nesting).
#[test]
fn a_tall_boolean_pulls_the_first_row_up() {
    let svg = render("wenn <(1) = (2)> dann\n  zeige Zahl (0)\nende", "makecode", "de");
    let d = paths(&svg).remove(0);
    assert!(d.contains(" a 4 4 0 0,1 4,4 V 46 a 4 4 0 0,1 -4,4 H 64 "), "8 - 4 + 42: {d}");
    assert!(svg.contains(" 4)\">"), "the compare block at y = 4: {svg}");
    // the compare block: a 42 hexagon with two pills at y = 5
    assert!(svg.contains("m 21,0 h "), "{svg}");
    assert!(svg.contains(" l 21,21 l -21,21 V 42 h -"), "{svg}");
    assert!(svg.contains("<g transform=\"translate(21 5)\">"), "round in hexagon: no hug: {svg}");
}

/// ((1) + (2)): round in round hugs by 17, the pills sit at x = 4.
#[test]
fn a_reporter_hugs_its_round_children() {
    let svg = render("((1) + (2))", "makecode", "de");
    let d = paths(&svg).remove(0);
    assert!(d.starts_with("m 21,0 h "), "{d}");
    assert!(d.contains(" a 21 21 0 0,1 21,21 v 0 a 21 21 0 0,1 -21,21 V 42 h -"), "{d}");
    assert!(svg.contains("<g transform=\"translate(4 5)\">"), "{svg}");
    assert!(svg.contains("fill=\"#ffffff\" stroke=\"#9a4dbb\""), "shadow stroke under a dark border: {svg}");
}

/// (Lichtstärke): a 40 pill whose label starts at 12.
#[test]
fn a_label_reporter_is_40_high_with_the_label_at_12() {
    let svg = render("(Lichtstärke)", "makecode", "de");
    let d = paths(&svg).remove(0);
    assert!(d.starts_with("m 20,0 h ") && d.contains(" a 20 20 0 0,1 20,20 v 0 a 20 20 0 0,1 -20,20 V 40 h -"), "{d}");
    assert!(svg.contains("x=\"12\""), "{svg}");
}

/// <nicht < >>: a hexagon with a hexagonal hole in the tertiary colour.
#[test]
fn an_empty_boolean_slot_is_a_hexagonal_hole() {
    let svg = render("<nicht <>>", "makecode", "de");
    let ps = paths(&svg);
    assert!(ps[0].starts_with("m 20,0 h ") && ps[0].contains(" l 20,20 l -20,20 V 40 h -"), "{}", ps[0]);
    assert!(ps[1].contains(" h 16 l 16,16 l -16,16 h -16 l -16,-16 l 16,-16 z"), "{}", ps[1]);
    assert!(svg.contains("fill=\"#007b7d\"/>"), "hole in tertiary: {svg}");
}

#[test]
fn a_string_literal_is_a_white_pill_with_quotes() {
    let svg = render("zeige Text [Hello!]", "makecode", "de");
    assert!(svg.contains(">\"</text>"), "{svg}");
    assert!(svg.contains("fill=\"#ffffff\" stroke=\"#bfbfbf\""), "{svg}");
}

// --- notation --------------------------------------------------------------------

/// "pausiere (ms) %1": the unit in parentheses is typed like a value and drawn as the label.
#[test]
fn a_parenthesised_unit_is_typed_like_a_value_and_drawn_as_a_label() {
    let node = &parse("pausiere (ms) (100)", "makecode", "de")[0];
    assert_eq!(node["id"], "device_pause", "{node}");
    let svg = render("pausiere (ms) (100)", "makecode", "de");
    assert!(svg.contains(">pausiere (ms)<"), "{svg}");
    assert_eq!(parse("Temperatur (°C)", "makecode", "de")[0]["id"], "device_temperature");
}

#[test]
fn operators_take_their_spelling_variants() {
    assert_eq!(parse("((1) * (2))", "makecode", "de")[0]["id"], "math_arithmetic_mul");
    assert_eq!(parse("((1) × (2))", "makecode", "de")[0]["id"], "math_arithmetic_mul");
    assert_eq!(parse("<(1) ≥ (2)>", "makecode", "en")[0]["id"], "logic_compare_ge");
    assert_eq!(parse("<(1) >= (2)>", "makecode", "en")[0]["id"], "logic_compare_ge");
    assert_eq!(parse("<(1) <= (2)>", "makecode", "de")[0]["id"], "logic_compare_le");
    assert_eq!(parse("<(1) != (2)>", "makecode", "de")[0]["id"], "logic_compare_ne");
    assert_eq!(parse("((1) ^ (2))", "makecode", "en")[0]["id"], "math_arithmetic_pow");
}

#[test]
fn a_variable_reporter_is_red() {
    let svg = render("zeige Zahl (zähler)", "makecode", "de");
    assert!(svg.contains("fill=\"#dc143c\""), "{svg}");
}

#[test]
fn the_else_row_carries_its_button_at_the_right_edge_and_the_plus_row_follows() {
    let svg = render("wenn <wahr> dann\n  zeige Zahl (0)\nansonsten\n  zeige Zahl (1)\nende", "makecode", "de");
    assert!(svg.contains(">ansonsten<"), "{svg}");
    let d = paths(&svg).remove(0);
    // rows: 8 | 32 | 8 | mouth 48 | 8 | 32 | 8 | mouth 48 | 4 | 24 | 0 | bottom 12
    assert!(d.contains(" V 40 V 44 a 4 4 0 0,1 -4,4 H 64 "), "{d}");
    assert!(d.ends_with("h -8 a 4 4 0 0,1 -4,-4 z"), "{d}");
    assert_eq!(svg.matches("<circle").count(), 2, "one minus, one plus: {svg}");
}

/// A block keeps its own colour wherever the toolbox lists it: setze and ändere are both variables-red.
#[test]
fn set_and_change_variable_share_the_variables_colour() {
    for code in ["setze [zähler v] auf (0)", "ändere [zähler v] um (1)"] {
        let svg = render(code, "makecode", "de");
        assert!(svg.contains("fill=\"#dc143c\""), "{code}: {svg}");
    }
}

// --- editor fields ---------------------------------------------------------------

/// zeige LEDs: a 167 x 169 grid of 25px cells on a 32px pitch in a row of its own; the editor's outline is `… V 40 V 48 V 217 V 221 …`, 233 high.
#[test]
fn the_led_matrix_is_a_5_by_5_grid_in_its_own_row() {
    let svg = render("zeige LEDs [#...#|.#.#.|..#..|.#.#.|#...#]", "makecode", "de");
    let d = paths(&svg).remove(0);
    assert!(d.contains(" V 48 V 217 V 221 a 4 4 0 0,1 -4,4 H 48 "), "{d}");
    assert_eq!(svg.matches("width=\"25\" height=\"25\" rx=\"5\"").count(), 25, "{svg}");
    assert_eq!(svg.matches("rx=\"5\" fill=\"#ffffff\"").count(), 9, "nine lit LEDs: {svg}");
    assert!(svg.contains("x=\"15\" y=\"53\""), "first cell at (8 + 7, 48 + 5): {svg}");
}

/// spiele (Melodie […] mit Tempo (120) (bpm)) [bis zum Ende v]: the melody shadow is a 50 pill holding a 140 x 42 grey editor with eight 10 x 20 note cells.
#[test]
fn the_melody_editor_is_an_eight_note_grid_in_a_grey_pill() {
    let code = "spiele (Melodie [C D E F - - - -] mit Tempo (120) (bpm)) [bis zum Ende v]";
    let node = &parse(code, "makecode", "de")[0];
    assert_eq!(node["id"], "music_playable_play", "{node}");
    let svg = render(code, "makecode", "de");
    assert!(svg.contains("fill=\"#d9d9d9\""), "{svg}");
    assert_eq!(svg.matches("width=\"10\" height=\"20\" rx=\"3\"").count(), 8, "{svg}");
    assert_eq!(svg.matches("fill=\"#dcdcdc\"").count(), 4, "four rests: {svg}");
    let d = paths(&svg).remove(0);
    assert!(d.contains(" V 58 V 62 a 4 4 0 0,1 -4,4 H 48 "), "8 + 50 + 4 + 4: {d}");
    assert!(svg.contains("m 25,0 h "), "the melody shadow is a 50 pill: {svg}");
}
