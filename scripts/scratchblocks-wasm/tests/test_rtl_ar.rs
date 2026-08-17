// Arabic / right-to-left checks.
//
// These run natively (`cargo test`), not in WASM, so they exercise the parser
// and renderer directly and say precisely which stage is at fault.

use scratchblocks_wasm::render_request_json;

fn render(code: &str, language: &str) -> String {
    let payload = serde_json::json!({
        "code": code,
        "language": language,
        "inline": false,
    });
    render_request_json(&payload.to_string()).expect("render failed")
}

/// A block the locale knows must not fall back to the grey "unknown" colour.
/// Grey is #9966FF-less: the renderer paints unrecognised blocks with the
/// `other` category, so grey in the output means the spec did not match.
fn assert_recognised(svg: &str, what: &str) {
    // the "unknown/other" category fill used by theme.rs
    assert!(
        !svg.contains("#bfbfbf") && !svg.contains("#BFBFBF"),
        "{what}: block was not recognised (rendered in the grey fallback colour)"
    );
}

#[test]
fn arabic_repeat_is_recognised() {
    let svg = render("كرِّر (4) مرة\nتحرك (10) خطوة\nنهاية", "ar");
    assert_recognised(&svg, "CONTROL_REPEAT");
}

#[test]
fn arabic_flag_hat_is_recognised() {
    let svg = render("عند نقر @greenFlag", "ar");
    assert_recognised(&svg, "EVENT_WHENFLAGCLICKED");
}

#[test]
fn arabic_say_for_secs_is_recognised() {
    let svg = render("قل [مرحبا] لمدة (2) ثانية", "ar");
    assert_recognised(&svg, "LOOKS_SAYFORSECS");
}

/// The whole point of the change: in an RTL locale the first label must sit
/// on the RIGHT of the block, so its x is past the middle.
#[test]
fn arabic_lays_out_right_to_left() {
    let svg = render("تحرك (10) خطوة", "ar");
    let first_label_x = first_label_x(&svg).expect("no label in output");
    let width = svg_width(&svg).expect("no width on the <svg> element");
    assert!(
        first_label_x > width / 2.0,
        "first label should start on the right half in RTL: x={first_label_x}, width={width}"
    );
}

/// ...and the same script in English must still start on the LEFT.
#[test]
fn english_still_lays_out_left_to_right() {
    let svg = render("move (10) steps", "en");
    let first_label_x = first_label_x(&svg).expect("no label in output");
    let width = svg_width(&svg).expect("no width on the <svg> element");
    assert!(
        first_label_x < width / 2.0,
        "first label should start on the left half in LTR: x={first_label_x}, width={width}"
    );
}

// --- tiny helpers so the tests can assert on geometry ---------------------

fn svg_width(svg: &str) -> Option<f32> {
    let at = svg.find("width=\"")? + 7;
    let rest = &svg[at..];
    let end = rest.find('"')?;
    rest[..end].parse().ok()
}

/// x of the first drawn label.
///
/// The element carries BOTH `x="0"` and `transform="translate(X Y)"`, and it
/// is the translate that positions it — reading the `x` attribute reports 0
/// for every label and the assertion becomes meaningless.
fn first_label_x(svg: &str) -> Option<f32> {
    // skip the <style> block, which also mentions .sb-label
    let body = svg.split("</style>").last().unwrap_or(svg);
    let at = body.find("class=\"sb-label\"")?;
    let rest = &body[at..];
    let t = rest.find("translate(")? + "translate(".len();
    let rest = &rest[t..];
    let end = rest.find(' ')?;
    rest[..end].parse().ok()
}

/// Hebrew and Persian ship with the package and are right-to-left too; they
/// only needed the `dir` key, so this guards the locale files themselves.
#[test]
fn hebrew_and_persian_are_rtl() {
    for (lang, code) in [
        ("he", "\u{5d6}\u{5d5}\u{5d6} (10) \u{5e6}\u{5e2}\u{5d3}\u{5d9}\u{5dd}"),
        ("fa", "\u{62d}\u{631}\u{6a9}\u{62a} \u{6a9}\u{646}  (10) \u{6af}\u{627}\u{645}"),
    ] {
        let svg = render(code, lang);
        let x = first_label_x(&svg).expect("no label");
        let w = svg_width(&svg).expect("no width");
        assert!(x > w / 2.0, "{lang} should lay out RTL: x={x}, width={w}");
    }
}

/// Arabic short vowels are optional and their order is not canonical, so the
/// same block must be found whether or not the user types them.
#[test]
fn arabic_matches_with_and_without_harakat() {
    let with = render("\u{643}\u{631}\u{650}\u{651}\u{631} (4) \u{645}\u{631}\u{629}", "ar");
    let without = render("\u{643}\u{631}\u{631} (4) \u{645}\u{631}\u{629}", "ar");
    assert_recognised(&with, "repeat with harakat");
    assert_recognised(&without, "repeat without harakat");
}

/// The C-block mouth, the notch and the loop arrow are asymmetric shapes.
/// In RTL the outline is mirrored, so the SVG must carry the flip transform.
#[test]
fn rtl_mirrors_the_block_outline() {
    let svg = render("\u{643}\u{631}\u{631} (4) \u{645}\u{631}\u{629}\n\u{62a}\u{62d}\u{631}\u{643} (10) \u{62e}\u{637}\u{648}\u{629}\n\u{646}\u{647}\u{627}\u{64a}\u{629}", "ar");
    assert!(svg.contains("scale(-1 1)"), "the outline should be mirrored in RTL");
    let en = render("repeat (4)\nmove (10) steps\nend", "en");
    assert!(!en.contains("scale(-1 1)"), "LTR must not mirror anything");
}

/// Blocks in a stack share an edge, and it must be the reading edge.
///
/// This is what a reader notices first: with every block stacked at x = 0
/// their LEFT sides line up, so in Arabic the stack looks ragged down the
/// side the eye follows. The short block and the long one must instead end
/// flush on the right.
#[test]
fn rtl_stack_is_flush_on_the_reading_edge() {
    // a deliberately uneven stack: a short block above a much longer one
    let svg = render(
        "\u{62a}\u{62d}\u{631}\u{643} (10) \u{62e}\u{637}\u{648}\u{629}\n\u{642}\u{644} [\u{645}\u{631}\u{62d}\u{628}\u{627}] \u{644}\u{645}\u{62f}\u{629} (2) \u{62b}\u{627}\u{646}\u{64a}\u{629}",
        "ar",
    );
    let boxes = stack_boxes(&svg);
    assert!(boxes.len() >= 2, "expected at least two stacked blocks");
    let rights: Vec<f32> = boxes.iter().map(|(x, w)| x + w).collect();
    let spread = rights
        .iter()
        .fold(0.0f32, |acc, r| acc.max((r - rights[0]).abs()));
    assert!(
        spread < 3.0,
        "stacked blocks should be flush on the right in RTL, spread was {spread}: {rights:?}"
    );
}

/// The same stack in English must stay flush on the LEFT.
#[test]
fn ltr_stack_is_flush_on_the_left() {
    let svg = render("move (10) steps\nsay [Hello] for (2) seconds", "en");
    let lefts: Vec<f32> = stack_boxes(&svg).iter().map(|(x, _)| *x).collect();
    assert!(lefts.len() >= 2, "expected at least two stacked blocks");
    let spread = lefts
        .iter()
        .fold(0.0f32, |acc, l| acc.max((l - lefts[0]).abs()));
    assert!(spread < 3.0, "LTR stack should stay flush left: {lefts:?}");
}

/// The (x, width) of each block in the top-level stack.
///
/// `render_script` emits one `<g transform="translate(x y)">` per stacked
/// block, with y being the running vertical offset. In RTL that group holds
/// a nested `translate(w 0) scale(-1 1)` that mirrors the outline — and that
/// nested translate carries the block's own width, which is exactly what is
/// needed here. In LTR the width comes from the outline path instead.
fn stack_boxes(svg: &str) -> Vec<(f32, f32)> {
    let body = svg.split("</defs>").last().unwrap_or(svg);
    let mut out = Vec::new();
    let chunks: Vec<&str> = body.split("<g transform=\"translate(").collect();
    for (i, part) in chunks.iter().enumerate().skip(1) {
        let Some(sp) = part.find(' ') else { continue };
        let Ok(x) = part[..sp].parse::<f32>() else { continue };
        let rest = &part[sp + 1..];
        let Some(close) = rest.find(')') else { continue };
        let Ok(y) = rest[..close].trim().parse::<f32>() else { continue };
        if y == 0.0 {
            continue; // the mirror wrapper, not a stacked block
        }
        // width: either the mirror translate that immediately follows, or
        // the largest coordinate of this group's own outline
        let mut w = 0.0f32;
        if let Some(next) = chunks.get(i + 1) {
            if next.contains("scale(-1 1)") {
                if let Some(sp2) = next.find(' ') {
                    if let Ok(v) = next[..sp2].parse::<f32>() {
                        w = v;
                    }
                }
            }
        }
        if w == 0.0 {
            if let Some(ds) = part.find("d=\"") {
                let seg = &part[ds + 3..];
                if let Some(de) = seg.find('"') {
                    for tok in seg[..de].split_whitespace() {
                        if let Ok(v) = tok.parse::<f32>() {
                            w = w.max(v);
                        }
                    }
                }
            }
        }
        if w > 1.0 {
            out.push((x, w));
        }
    }
    out
}

/// A C-block whose BODY is wider than its header.
///
/// The header labels are mirrored inside the width the block is actually
/// drawn at. Mirroring them inside `c_block_inner_width` — the header's own
/// measure — leaves the text stranded against the left edge as soon as a
/// long block inside the mouth widens the shape.
#[test]
fn rtl_c_block_header_hugs_the_reading_edge() {
    // "forever / say [a long sentence] for (2) seconds / end"
    let svg = render(
        "\u{643}\u{631}\u{651}\u{650}\u{631} \u{628}\u{627}\u{633}\u{62a}\u{645}\u{631}\u{627}\u{631}\n\u{642}\u{644} [\u{645}\u{631}\u{62d}\u{628}\u{627} \u{64a}\u{627} \u{623}\u{635}\u{62f}\u{642}\u{627}\u{621}] \u{644}\u{645}\u{62f}\u{629} (2) \u{62b}\u{627}\u{646}\u{64a}\u{629}\n\u{646}\u{647}\u{627}\u{64a}\u{629}",
        "ar",
    );
    let width = svg_width(&svg).expect("no width");
    let x = first_label_x(&svg).expect("no label");
    // the header label must sit well past the middle, not near x = 0
    assert!(
        x > width * 0.5,
        "C-block header should hug the right edge: x={x}, width={width}"
    );
}

/// The header must be painted once. It used to be emitted twice in a row,
/// so every C-block label was drawn on top of itself.
#[test]
fn c_block_header_is_not_drawn_twice() {
    let svg = render("repeat (4)\nmove (10) steps\nend", "en");
    let repeats = svg.matches(">repeat<").count();
    assert_eq!(repeats, 1, "the C-block header should be painted once");
}

/// The C-block OUTLINE, not just its text, must reach the reading edge.
///
/// A C-block is drawn at `c_block_inner_width` — the header's own measure —
/// while its footprint is as wide as the widest block in the mouth. In LTR
/// both start at x = 0 and the difference only shows as content spilling
/// past the arms. In RTL the stack flushes the footprint right, so an
/// outline mirrored about its own narrow width stops short of that edge and
/// the block visibly steps in from the ones above and below it.
#[test]
fn rtl_c_block_outline_reaches_the_reading_edge() {
    // "repeat (4) / turn right (90) degrees / end" — the nested block is
    // markedly wider than the header, which is what exposes the bug.
    let svg = render(
        "\u{643}\u{631}\u{651}\u{650}\u{631} (4) \u{645}\u{631}\u{629}\n\u{627}\u{633}\u{62a}\u{62f}\u{631} @turnRight (90) \u{62f}\u{631}\u{62c}\u{629}\n\u{646}\u{647}\u{627}\u{64a}\u{629}",
        "ar",
    );
    let width = svg_width(&svg).expect("no width");
    let right = mirrored_group_origin(&svg).expect("no mirrored outline in RTL output");
    // The mirror origin is the outline's right edge. Allow the 4px the
    // renderer keeps as trailing margin, nothing like the ~57px gap the bug
    // left behind.
    assert!(
        right >= width - 8.0,
        "C-block outline should reach the reading edge: right={right}, width={width}"
    );
}

/// x of the first `scale(-1 1)` group — in RTL that translate IS the right
/// edge the shape is reflected about.
fn mirrored_group_origin(svg: &str) -> Option<f32> {
    let body = svg.split("</defs>").last().unwrap_or(svg);
    for part in body.split("<g transform=\"translate(") {
        if !part.contains("scale(-1 1)") {
            continue;
        }
        let end = part.find(' ')?;
        if let Ok(v) = part[..end].parse::<f32>() {
            return Some(v);
        }
    }
    None
}

/// The loop arrow POINTS — it must point back into the loop.
///
/// The glyph curls anticlockwise, drawn for a script read left to right.
/// Placing it on the left edge in RTL without flipping it aims it away from
/// the block it loops back to. It is layout, not meaning (unlike @turnRight,
/// where the direction is what the sprite is told to do), so it mirrors.
#[test]
fn rtl_loop_arrow_points_back_into_the_loop() {
    let svg = render(
        "\u{643}\u{631}\u{651}\u{650}\u{631} (4) \u{645}\u{631}\u{629}\n\u{62a}\u{62d}\u{631}\u{643} (10) \u{62e}\u{637}\u{648}\u{629}\n\u{646}\u{647}\u{627}\u{64a}\u{629}",
        "ar",
    );
    let use_tag = loop_arrow_use(&svg).expect("no loop arrow in the C-block");
    assert!(
        use_tag.contains("scale(-"),
        "the loop arrow should be mirrored in RTL: {use_tag}"
    );
}

/// ...and in English it must keep pointing the way upstream draws it.
#[test]
fn ltr_loop_arrow_is_not_mirrored() {
    let svg = render("repeat (4)\nmove (10) steps\nend", "en");
    let use_tag = loop_arrow_use(&svg).expect("no loop arrow in the C-block");
    assert!(
        !use_tag.contains("scale(-"),
        "the loop arrow must not be mirrored in LTR: {use_tag}"
    );
}

/// The `<use>` element that draws the loop arrow, taken from the body of the
/// document — `<defs>` also mentions `sb-loopArrow`, as the definition.
fn loop_arrow_use(svg: &str) -> Option<String> {
    let body = svg.split("</defs>").last()?;
    let at = body.find("href=\"#sb-loopArrow\"")?;
    let rest = &body[at..];
    let end = rest.find("/>")?;
    Some(rest[..end].to_string())
}

/// The `define` hat had no right-to-left handling at all: the dome, the
/// keyword and the prototype outline all stayed on the left while the rest
/// of the script flushed right.
#[test]
fn rtl_define_hat_is_mirrored() {
    let svg = render("\u{62a}\u{639}\u{631}\u{64a}\u{641} (\u{646}\u{642}\u{637}\u{629})", "ar");
    let width = svg_width(&svg).expect("no width");
    let x = first_label_x(&svg).expect("no label");
    assert!(
        x > width / 2.0,
        "the define keyword should sit on the reading edge: x={x}, width={width}"
    );
}

/// ...and the pen badge, which marks where a pen block starts, likewise.
#[test]
fn rtl_pen_badge_sits_on_the_reading_edge() {
    let svg = render("\u{623}\u{646}\u{632}\u{644} \u{627}\u{644}\u{642}\u{644}\u{645}", "ar");
    let width = svg_width(&svg).expect("no width");
    let use_tag = pen_icon_use(&svg).expect("no pen icon");
    let x = translate_x(&use_tag).expect("no translate on the pen icon");
    assert!(
        x > width / 2.0,
        "the pen badge should sit on the reading edge: x={x}, width={width}"
    );
    assert!(use_tag.contains("scale(-"), "the pen badge should be flipped: {use_tag}");
}

#[test]
fn ltr_pen_badge_stays_on_the_left() {
    let svg = render("pen down", "en");
    let use_tag = pen_icon_use(&svg).expect("no pen icon");
    let x = translate_x(&use_tag).expect("no translate");
    assert!(x < 12.0, "the pen badge should hug the left in LTR: x={x}");
    assert!(!use_tag.contains("scale(-"), "must not be flipped in LTR: {use_tag}");
}

/// The `<g>` that positions the pen badge, taken from the document body.
fn pen_icon_use(svg: &str) -> Option<String> {
    let body = svg.split("</defs>").last()?;
    let at = body.find("#sb-penIcon")?;
    // walk back to the enclosing <g transform="...">
    let start = body[..at].rfind("<g transform=\"translate(")?;
    let rest = &body[start..];
    let end = rest.find("</g>")?;
    Some(rest[..end].to_string())
}

fn translate_x(tag: &str) -> Option<f32> {
    let at = tag.find("translate(")? + "translate(".len();
    let rest = &tag[at..];
    let end = rest.find(' ')?;
    rest[..end].parse().ok()
}

/// The greyscale theme has to keep the categories apart. Converting each
/// Scratch colour to its own luminance does not: `lists` and `motion` both
/// land on grey 151. The palette is assigned instead, and this guards it.
#[test]
fn grayscale_gives_each_category_a_distinct_shade() {
    let mut seen: Vec<String> = Vec::new();
    for code in [
        "move (10) steps",
        "say [hi]",
        "play sound (Meow v)",
        "when green flag clicked",
        "wait (1) seconds",
        "touching (edge v)?",
        "((1) + (2))",
        "set [v v] to (0)",
        "add [x] to [list v]",
        "pen down",
    ] {
        let payload = serde_json::json!({
            "code": code, "language": "en", "inline": false, "theme": "grayscale",
        });
        let svg = render_request_json(&payload.to_string()).expect("render failed");
        let fill = first_block_fill(&svg).expect("no filled path");
        assert!(
            !seen.contains(&fill),
            "two categories share the grey {fill}; the scale has collapsed"
        );
        seen.push(fill);
    }
    assert_eq!(seen.len(), 10);
}

/// Fill of the first drawn block path.
fn first_block_fill(svg: &str) -> Option<String> {
    let body = svg.split("</defs>").last()?;
    let at = body.find("<path")?;
    let rest = &body[at..];
    let f = rest.find("fill=\"")? + 6;
    let rest = &rest[f..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
