//! What comes out of the renderer: valid SVG, and Open Roberta's own colours.

use nepo_wasm::parser;
use nepo_wasm::theme;

const PROGRAM: &str = "Start
  Zeige Text \"Hallo\"
  Schalte RGB LED an (#ff0000)
  Wiederhole unendlich oft
    Zeige Bild (.#.#.|.#.#.|.....|#...#|.###.)
  Ende";

fn render(source: &str) -> String {
    render_with(source, "normal")
}

fn render_with(source: &str, theme: &str) -> String {
    let request = serde_json::json!({
        "code": source,
        "language": "de",
        "platform": "calliope",
        "theme": theme,
    });
    parser::render_request(&request.to_string()).expect("render")
}

#[test]
fn the_svg_carries_no_invalid_measurements() {
    let svg = render(PROGRAM);
    for bad in ["NaN", "Infinity", "inf", "null", "undefined"] {
        assert!(!svg.contains(bad), "output contains {bad}");
    }
    // A stray `-` or a lone `.` would break a path. Splitting on whitespace is
    // not enough: Blockly's own fragments run numbers and commands together
    // ("l-5.154 -0.469c0 0"), so the path is scanned the way an SVG parser
    // scans it.
    for path in paths(&svg) {
        for number in numbers(&path) {
            assert!(
                number.parse::<f64>().is_ok(),
                "unparseable number {number:?} in path {path}"
            );
        }
    }
}

#[test]
fn no_placeholder_survives_into_the_output() {
    let svg = render(PROGRAM);
    for name in [
        "%TYPE", "%OUT", "%COLOR", "%VALUE", "%DO", "%PICTURE", "%TEXT",
    ] {
        assert!(!svg.contains(name), "unresolved placeholder {name}");
    }
}

#[test]
fn the_svg_is_balanced() {
    let svg = render(PROGRAM);
    assert!(svg.starts_with("<svg "), "not an svg root");
    assert!(svg.ends_with("</svg>"), "unterminated svg");
    let opens = svg.matches("<g ").count();
    let closes = svg.matches("</g>").count();
    assert_eq!(opens, closes, "unbalanced groups");
    assert_eq!(
        svg.matches("<text").count(),
        svg.matches("</text>").count(),
        "unbalanced text elements"
    );
    // Self-closing elements must actually close.
    assert_eq!(
        svg.matches("<path").count(),
        svg.matches("/>").count() - svg.matches("<rect").count()
    );
}

#[test]
fn categories_use_open_robertas_own_colours() {
    let svg = render(PROGRAM);
    // CAT_ACTIVITY_RGB, CAT_ACTION_RGB, CAT_CONTROL_RGB, CAT_IMAGE_RGB
    for (category, expected) in [
        ("activity", "#E2001A"),
        ("action", "#F29400"),
        ("control", "#EB6A0A"),
        ("image", "#DF01D7"),
        ("colour", "#EBC300"),
        ("text", "#BACC1E"),
    ] {
        assert_eq!(theme::category_colour(category), expected);
        assert!(
            svg.contains(&format!("fill=\"{expected}\"")),
            "{category} block missing from the output"
        );
    }
}

#[test]
fn a_typed_plug_takes_the_colour_of_its_data_type() {
    // A Boolean sensor on an olive body must still plug in cyan.
    let svg = render("Taste A gedrückt?");
    assert!(svg.contains("fill=\"#8FA402\""), "sensor body");
    assert!(svg.contains("fill=\"#33B8CA\""), "Boolean plug");
    assert_eq!(theme::data_type_colour("Boolean"), "#33B8CA");
}

#[test]
fn empty_single_type_sockets_are_traced_in_their_type_colour() {
    let svg = render("Warte ms");
    assert!(
        svg.contains("stroke=\"#005A94\""),
        "empty Number socket should be outlined in the Number colour"
    );
    let filled = render("Warte ms 500");
    assert!(
        !filled.contains("stroke=\"#005A94\""),
        "a filled socket needs no type hint"
    );

    let multi_type = render("Zeige Text");
    assert!(
        !multi_type.contains("stroke=\"#BACC1E\""),
        "a Number/Boolean/String socket must not claim to be String-only"
    );
}

#[test]
fn blocks_are_flat_fills_in_the_normal_theme() {
    // Open Roberta's stylesheet has no `.blocklyPath` stroke rule, and the
    // fork comments out the highlight and shadow paths. A stroke here would
    // be a Scratch habit leaking in.
    let svg = render(PROGRAM);
    assert!(!svg.contains("stroke=\"#000000\""), "unexpected outline");
}

#[test]
fn the_print_theme_outlines_instead_of_filling() {
    let svg = render_with(PROGRAM, "print");
    assert!(svg.contains("fill=\"#ffffff\""));
    assert!(svg.contains("stroke=\"#000000\""));
    assert!(
        !svg.contains("fill=\"#F29400\""),
        "colour survived into print"
    );
}

#[test]
fn every_prototype_block_renders() {
    for source in [
        "Start",
        "Zeige Text \"Hallo\"",
        "Schalte RGB LED an (#ff0000)",
        "Taste A gedrückt?",
        "Wiederhole unendlich oft\n  Zeige Text \"Hallo\"\nEnde",
        "Zeige Bild (.#.#.|.#.#.|.....|#...#|.###.)",
        "Lösche Bildschirm",
        "Schalte RGB LED aus",
        "Wiederhole 3 mal\n  Warte ms 500\nEnde",
        "Warte bis Taste A gedrückt?",
        "wenn wahr\n  Lösche Bildschirm\nsonst\n  Schalte RGB LED aus\nEnde",
    ] {
        let svg = render(source);
        assert!(svg.len() > 200, "suspiciously small output for {source:?}");
        assert!(svg.contains("<path"), "no outline for {source:?}");
    }
}

#[test]
fn inline_sockets_and_predefined_images_render() {
    let svg = render("Warte ms 1 +\nWarte bis Taste A gedrückt? und wahr\nZeige Bild Herz");
    // Empty inline sockets have their own rounded outline, and a selected
    // image is a compact 5x5 LED preview rather than a text-only dropdown.
    assert!(svg.contains("stroke=\"#005A94\""), "number socket missing");
    assert!(svg.matches("width=\"3\" height=\"3\"").count() >= 10);
}

#[test]
fn connected_inline_reporters_knock_a_border_out_of_the_parent() {
    let svg = render("Warte bis gib Wert % Lichtsensor < 50 und gib Wert % Lichtsensor ≤ 100");
    assert!(
        svg.contains("fill-rule=\"evenodd\""),
        "inline children must cut a transparent contour from their parent"
    );
    // The two child comparisons each add a path starting at their own slot.
    let all_paths = paths(&svg);
    assert!(
        all_paths.iter().any(|path| path.matches("M ").count() >= 2),
        "no recursively knocked-out reporter path: {all_paths:?}"
    );
}

#[test]
fn colour_picker_keeps_a_useful_fixed_width() {
    let svg = render("Schalte RGB LED an (#ff0000)");
    assert!(
        svg.contains("width=\"22\" height=\"16\""),
        "the colour reporter should paint a 22px swatch"
    );
}

#[test]
fn variables_use_open_robertas_variable_colour_and_menu_shape() {
    let svg = render("Schreibe Punkte 0\nZeige Text Punkte");
    assert!(svg.contains("fill=\"#9085BA\""), "variable category colour");
    assert!(svg.contains("Punkte"), "variable name was lost");
    // `FieldDropdown.render_` appends the arrow to the option text and tints
    // it with the block's own colour, which is why it arrives in its own
    // tspan rather than inside the label run.
    assert!(
        svg.contains("Punkte<tspan fill=\"#9085BA\"> \u{25BE}</tspan>"),
        "variable should render as a menu"
    );
}

#[test]
fn the_matrix_paints_twenty_five_cells() {
    let svg = render("Zeige Bild (.#.#.|.#.#.|.....|#...#|.###.)");
    // 25 pixel boxes, all 16 wide.
    let cells = svg.matches("width=\"16\" height=\"16\"").count();
    assert_eq!(cells, 25);
    // Nine lit LEDs in this face: two eyes twice, then the mouth.
    let lit = svg.matches(">#</text>").count();
    assert_eq!(lit, 9);
}

#[test]
fn text_measurements_from_the_host_are_used() {
    let plain = render("Start");
    let request = serde_json::json!({
        "code": "Start",
        "language": "de",
        "platform": "calliope",
        "widths": { "prop\u{1}Start": 400.0 },
    });
    let wide = parser::render_request(&request.to_string()).expect("render");
    assert_ne!(plain, wide, "supplied widths were ignored");
    // The official start block also has its two-space layout field before
    // the hidden DEBUG field, and the plus mutator ahead of both:
    // 27 icon + 400 + 10 separator + 8.155 space width + 20.
    assert!(
        wide.contains("H 465.155"),
        "block should grow to fit the label"
    );
}

/// Pull the numeric tokens out of path data the way an SVG parser would:
/// commands, commas and whitespace all end a number, and nothing has to
/// separate a number from the command that follows it.
fn numbers(path: &str) -> Vec<String> {
    let chars: Vec<char> = path.chars().collect();
    let mut out = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let ch = chars[index];
        if !(ch.is_ascii_digit() || ch == '-' || ch == '+' || ch == '.') {
            index += 1;
            continue;
        }
        let start = index;
        if chars[index] == '-' || chars[index] == '+' {
            index += 1;
        }
        let mut seen_dot = false;
        while index < chars.len() {
            match chars[index] {
                '.' if !seen_dot => {
                    seen_dot = true;
                    index += 1;
                }
                c if c.is_ascii_digit() => index += 1,
                _ => break,
            }
        }
        out.push(chars[start..index].iter().collect());
    }

    out
}

fn paths(svg: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = svg;
    while let Some(start) = rest.find("<path d=\"") {
        let from = start + "<path d=\"".len();
        let Some(end) = rest[from..].find('"') else {
            break;
        };
        out.push(rest[from..from + end].to_string());
        rest = &rest[from + end..];
    }
    out
}

#[test]
fn a_declaration_is_drawn_in_the_start_blocks_own_colour_with_a_minus() {
    let svg = render("Start\n  Variable Punkte : Zahl = 0");
    // CAT_ACTIVITY_RGB: a declaration belongs to the Start block, not to the
    // violet variable category.
    assert!(
        svg.matches(theme::category_colour("activity")).count() >= 2,
        "the Start block and its declaration share the activity colour"
    );
    assert!(
        svg.contains("M18 11h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12"),
        "the declaration carries Blockly's minus mutator"
    );
    assert!(
        svg.contains("M18 10h-4v-4"),
        "and the Start block the plus that created it"
    );
}

#[test]
fn a_declared_type_colours_the_plug_of_every_use() {
    let svg = render("Start\n  Variable Name : Zeichenkette = \"Ida\"\n  Zeige Text Name");
    // Blockly.DATA_TYPE.String. Without the declaration the getter's plug
    // would take the block's own violet.
    assert!(
        svg.contains(&format!(
            "{} z\" fill=\"{}\"/>",
            nepo_wasm::svg::TAB_PATH_DOWN_OUTER,
            theme::data_type_colour("String")
        )),
        "the getter should report the declared type"
    );
}

#[test]
fn a_font_name_cannot_break_out_of_the_stylesheet() {
    // The font stack comes from the document, and lands in an XML <style>.
    let request = serde_json::json!({
        "code": "Start",
        "language": "de",
        "platform": "calliope",
        "font": "Fo\"o</style><script>alert(1)</script>, sans & serif",
    });
    let svg = parser::render_request(&request.to_string()).expect("render");

    assert_eq!(
        svg.matches("</style>").count(),
        1,
        "the stylesheet is closed once, by the renderer"
    );
    assert!(!svg.contains("<script>"), "no markup came in with the font");
    assert!(
        svg.contains("&lt;/style&gt;") && svg.contains("sans &amp; serif"),
        "the name survives as text: {svg}"
    );
}
