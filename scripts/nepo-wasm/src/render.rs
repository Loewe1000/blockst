//! SVG output, following `renderDrawTop_` / `renderDrawRight_` /
//! `renderDrawBottom_` / `renderDrawLeft_`.

use crate::matrix;
use crate::measure::{self, BlockLayout, StackLayout};
use crate::model::{single_check, BlockSpec, DocumentSpec, SegmentSpec};
use crate::svg::*;
use crate::theme::{colours_for, connection_colour};

/// Gap between two top-level scripts on the page.
const SCRIPT_GAP: f32 = 24.0;

pub fn render_document(document: &DocumentSpec) -> String {
    let scale = document.scale.unwrap_or(1.0).max(0.1);
    let theme = document.theme.as_deref().unwrap_or("normal");

    let mut body = String::new();
    let mut total_width = 0.0f32;
    let mut total_height = 0.0f32;

    for (index, script) in document.scripts.iter().enumerate() {
        if index > 0 {
            total_height += SCRIPT_GAP;
        }
        let mut blocks = script.blocks.clone();
        for block in blocks.iter_mut() {
            matrix::expand(block);
        }
        let layout = measure::layout_stack(&blocks);

        // A value block's plug hangs into negative x. Shifting the script
        // right by one tab keeps it inside the viewBox; the width already
        // includes the tab, so it must not be added a second time.
        let offset_x = if starts_with_output(&layout) {
            TAB_WIDTH
        } else {
            0.0
        };

        body.push_str(&format!(
            "<g transform=\"translate({} {})\">{}</g>",
            fmt(offset_x),
            fmt(total_height),
            render_stack(&blocks, &layout, theme)
        ));

        total_width = total_width.max(layout.width);
        total_height += layout.height;
    }

    if document.scripts.is_empty() {
        total_width = 1.0;
        total_height = 1.0;
    }

    // One pixel of air so a stroke or the bottom notch is never clipped.
    total_width += 1.0;
    total_height += 1.0;

    let width = total_width * scale;
    let height = total_height * scale;

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">{defs}<g transform=\"scale({scale})\">{body}</g></svg>",
        w = fmt(width),
        h = fmt(height),
        defs = defs(&document.font, theme),
        scale = fmt(scale),
    )
}

fn starts_with_output(stack: &StackLayout) -> bool {
    stack
        .blocks
        .first()
        .map(|(_, b)| b.has_output)
        .unwrap_or(false)
}

/// `.blocklyText` and the editable-field rules from Open Roberta's `css.js`.
fn defs(font: &str, theme: &str) -> String {
    // SVG is XML, and XML gives `<style>` no raw-text exemption: an `&` or a
    // `<` inside the font stack would either make the document ill-formed or
    // — with `</style>` — end the element early and let the rest of the name
    // be read as markup. The stack is caller-supplied (`#nepo(font: ...)`),
    // so it is escaped like any other text node. A parser hands the CSS back
    // unescaped, so a legitimate family name is unaffected.
    let stack = escape_text(&css_font_stack(font));
    let label_fill = if theme == "print" { "#000" } else { "#fff" };
    format!(
        "<style>\
.nepo-label{{font-family:{stack};font-size:{size}px;fill:{label_fill}}}\
.nepo-label.mono{{font-family:\"Courier New\",Courier,monospace}}\
.nepo-field-text{{font-family:{stack};font-size:{size}px;fill:#000}}\
.nepo-field-text.mono{{font-family:\"Courier New\",Courier,monospace}}\
</style>",
        size = fmt(FONT_SIZE_PX),
    )
}

fn css_font_stack(font: &str) -> String {
    let font = font.trim();
    if font.is_empty() {
        return "sans-serif".to_string();
    }
    if font.contains(',') || font.starts_with('"') || font.starts_with('\'') {
        return font.to_string();
    }
    format!("\"{}\", sans-serif", font.replace('"', ""))
}

fn render_stack(blocks: &[BlockSpec], stack: &StackLayout, theme: &str) -> String {
    let mut svg = String::new();
    for (block, (y, layout)) in blocks.iter().zip(stack.blocks.iter()) {
        svg.push_str(&format!(
            "<g transform=\"translate(0 {})\">{}</g>",
            fmt(*y),
            render_block(block, layout, theme)
        ));
    }
    svg
}

fn render_block(block: &BlockSpec, layout: &BlockLayout, theme: &str) -> String {
    let colours = colours_for(&layout.category, theme);
    let mut steps: Vec<String> = Vec::new();

    draw_top(
        &mut steps,
        layout.right_edge,
        layout.square_top_left,
        layout.has_previous,
    );

    let mut overlays = String::new();
    let mut children = String::new();
    let mut fields = String::new();
    // Blockly carves a second contour into a parent reporter for every
    // connected inline value. That contour is transparent (and therefore
    // white on a worksheet), not a painted white stroke. It keeps same-colour
    // Boolean blocks visually separate as they nest recursively.
    let mut inline_holes = String::new();

    for (index, row) in layout.rows.iter().enumerate() {
        let spec_row = &block.rows[index];

        if row.lead_gap > 0.0 {
            vertical(&mut steps, row.lead_gap);
        }

        // Fields first, so the outline below can consume the row.
        for (position, field) in row.fields.iter().enumerate() {
            let field_spec = &spec_row.fields[position];
            match field_spec {
                SegmentSpec::InlineValue { check, value } => {
                    if let (Some(child_layout), Some(child_spec)) =
                        (field.inline_value.as_ref(), value.as_ref())
                    {
                        inline_holes.push_str(&inline_hole_path(field, child_layout, row.y));
                        // A reporter's coloured output tab reaches TAB_WIDTH
                        // to the left of its own origin, exactly filling this
                        // inline socket.
                        children.push_str(&format!(
                            "<g transform=\"translate({} {})\">{}</g>",
                            fmt(field.x + TAB_WIDTH),
                            fmt(row.y + 6.0),
                            render_block(child_spec, child_layout, theme)
                        ));
                    } else {
                        inline_holes.push_str(&empty_inline_hole_path(field, row.y));
                        fields.push_str(&render_empty_inline_socket(
                            field.x,
                            row.y,
                            check,
                            theme,
                            colours.fill,
                        ));
                    }
                }
                _ => fields.push_str(&render_field(
                    field_spec,
                    field.x,
                    field.y,
                    field.width,
                    &colours,
                    colours.fill,
                )),
            }
        }

        match row.kind.as_str() {
            "value" => {
                steps.push(TAB_PATH_DOWN_INNER.to_string());
                vertical(&mut steps, row.height - TAB_HEIGHT);

                match (row.value.as_ref(), spec_row.value.as_ref()) {
                    (Some(child_layout), Some(child_spec)) => {
                        children.push_str(&format!(
                            "<g transform=\"translate({} {})\">{}</g>",
                            fmt(row.value_origin.0),
                            fmt(row.value_origin.1),
                            render_block(child_spec, child_layout, theme)
                        ));
                    }
                    _ => {
                        // An empty socket announces the type it wants — but
                        // only when there is exactly one type to name.
                        if let Some(check) = single_check(&row.check) {
                            overlays.push_str(&format!(
                                "<path d=\"M {},{} {}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>",
                                fmt(layout.right_edge),
                                fmt(row.y + 2.6),
                                INPUT_TYPE_PATH,
                                connection_colour(check, theme),
                                INPUT_TYPE_STROKE_WIDTH,
                            ));
                        }
                    }
                }
            }
            "statement" => {
                horizontal_to(&mut steps, layout.statement_edge + NOTCH_WIDTH + 1.0);
                steps.push(inner_top_left_corner());
                vertical(&mut steps, row.height - 2.0 * CORNER_RADIUS);
                steps.push(inner_bottom_left_corner());
                horizontal_to(&mut steps, layout.right_edge);

                if let (Some(body_layout), false) = (row.body.as_ref(), spec_row.body.is_empty()) {
                    children.push_str(&format!(
                        "<g transform=\"translate({} {})\">{}</g>",
                        fmt(row.body_origin.0),
                        fmt(row.body_origin.1),
                        render_stack(&spec_row.body, body_layout, theme)
                    ));
                }
            }
            _ => {
                vertical(&mut steps, row.height);
            }
        }

        if row.trail_gap > 0.0 {
            vertical(&mut steps, row.trail_gap);
        }
    }

    if layout.rows.is_empty() {
        steps.push(format!("V {}", fmt(layout.bottom)));
    }

    draw_bottom(&mut steps, layout.square_bottom_left, layout.has_next);

    // A typed output plug is a separate path, so the main outline stops at
    // the top of the tab. A plug that names several types takes the block's
    // own colour, exactly as `renderDrawLeft_` does.
    let typed_output = layout.has_output && !layout.output_check.is_empty();
    if typed_output {
        steps.push(format!("V {}", fmt(TAB_HEIGHT)));
        steps.push("z".to_string());
    } else {
        draw_left(&mut steps, layout.has_output);
    }

    steps.push(inline_holes);

    let stroke = match colours.stroke {
        Some(colour) => format!(" stroke=\"{colour}\" stroke-width=\"1\""),
        None => String::new(),
    };

    let fill_rule = if steps.iter().any(|step| step.starts_with("M ")) {
        " fill-rule=\"evenodd\""
    } else {
        ""
    };
    let mut svg = format!(
        "<path d=\"{}\" fill=\"{}\"{} {}/>",
        steps.join(" "),
        colours.fill,
        fill_rule,
        stroke
    );

    if typed_output {
        let plug = match single_check(&layout.output_check) {
            Some(check) => connection_colour(check, theme).to_string(),
            None => colours.fill.to_string(),
        };
        svg.push_str(&format!(
            "<path d=\"M 0,{} {} z\" fill=\"{}\"{}/>",
            fmt(TAB_HEIGHT),
            TAB_PATH_DOWN_OUTER,
            plug,
            stroke
        ));
    }

    svg.push_str(&overlays);
    svg.push_str(&fields);
    svg.push_str(&children);
    svg
}

/// The internal contour from Blockly's `renderDrawRight_` for a filled inline
/// input. Its left edge follows the reporter output tab, while its right and
/// lower edges leave a deliberately tight half-pixel breathing room around
/// the child. This keeps the white knockout visible without making nested
/// Boolean expressions look separated by a wide gap.
/// With `fill-rule=evenodd` this becomes a true knockout in the parent path.
fn inline_hole_path(field: &measure::FieldLayout, child: &BlockLayout, row_y: f32) -> String {
    let child_x = field.x + TAB_WIDTH;
    let left = child_x - 0.5;
    let right = child_x + child.right_edge + 0.5;
    let tail = (child.bottom - TAB_HEIGHT + 0.5).max(0.0);
    format!(
        "M {},{} h {} v 1 {} v {} h {} z",
        fmt(right),
        fmt(row_y + 5.5),
        fmt(left - right),
        TAB_PATH_DOWN_INNER,
        fmt(tail),
        fmt(right - left),
    )
}

/// The companion contour for an empty inline input. It follows the exact
/// `renderDrawRight_` shape (including the output-tab-shaped left side), so a
/// worksheet's background shows through rather than a fabricated white box.
fn empty_inline_hole_path(field: &measure::FieldLayout, row_y: f32) -> String {
    let left = field.x + 6.0;
    let right = field.x + field.width;
    format!(
        "M {},{} h {} v 1 {} v 5 h {} z",
        fmt(right),
        fmt(row_y + 5.0),
        fmt(left - right),
        TAB_PATH_DOWN_INNER,
        fmt(right - left),
    )
}

fn render_field(
    segment: &SegmentSpec,
    x: f32,
    y: f32,
    width: f32,
    colours: &crate::theme::BlockColours,
    block_fill: &str,
) -> String {
    let mono = matches!(
        segment,
        SegmentSpec::Text {
            monospace: true,
            ..
        }
    );
    let baseline = y + FIELD_TEXT_BASELINE;

    match segment {
        SegmentSpec::Text { .. } => format!(
            "<text class=\"nepo-label{}\" x=\"{}\" y=\"{}\">{}</text>",
            if mono { " mono" } else { "" },
            fmt(x),
            fmt(baseline),
            escape_text(&measure::display_text(segment))
        ),
        SegmentSpec::ColourField { colour } => format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{r}\" ry=\"{r}\" fill=\"{colour}\" fill-opacity=\"1\"/>",
            fmt(x + FIELD_BOX_X),
            fmt(y),
            fmt(width + FIELD_BOX_PAD),
            fmt(FIELD_BOX_HEIGHT),
            r = fmt(FIELD_BOX_RADIUS),
        ),
        SegmentSpec::PixelCell { value } => {
            let mut svg = format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{r}\" ry=\"{r}\" fill=\"{}\" fill-opacity=\"{}\"/>",
                fmt(x + FIELD_BOX_X),
                fmt(y),
                fmt(PIXEL_BOX_SIZE),
                fmt(FIELD_BOX_HEIGHT),
                colours.field_fill,
                colours.field_fill_opacity,
                r = fmt(FIELD_BOX_RADIUS),
            );
            if !value.is_empty() {
                svg.push_str(&format!(
                    "<text class=\"nepo-field-text mono\" x=\"{}\" y=\"{}\">{}</text>",
                    fmt(x),
                    fmt(baseline),
                    escape_text(value)
                ));
            }
            svg
        }
        SegmentSpec::Dropdown { .. } => {
            let value = measure::display_text(segment);
            let value = value.trim_end_matches(['\u{25BE}', ' ']);
            let box_svg = format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{r}\" ry=\"{r}\" fill=\"{}\" fill-opacity=\"{}\"/>",
                fmt(x + FIELD_BOX_X),
                fmt(y),
                fmt(width + FIELD_BOX_PAD),
                fmt(FIELD_BOX_HEIGHT),
                colours.field_fill,
                colours.field_fill_opacity,
                r = fmt(FIELD_BOX_RADIUS),
            );
            // `FieldDropdown.render_` tints the arrow with the block's colour.
            format!(
                "{box_svg}<text class=\"nepo-field-text\" x=\"{}\" y=\"{}\">{}<tspan fill=\"{block_fill}\"> \u{25BE}</tspan></text>",
                fmt(x),
                fmt(baseline),
                escape_text(value)
            )
        }
        SegmentSpec::Input { .. } => format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{r}\" ry=\"{r}\" fill=\"{}\" fill-opacity=\"{}\"/><text class=\"nepo-field-text\" x=\"{}\" y=\"{}\">{}</text>",
            fmt(x + FIELD_BOX_X),
            fmt(y),
            fmt(width + FIELD_BOX_PAD),
            fmt(FIELD_BOX_HEIGHT),
            colours.field_fill,
            colours.field_fill_opacity,
            fmt(x),
            fmt(baseline),
            escape_text(&measure::display_text(segment)),
            r = fmt(FIELD_BOX_RADIUS),
        ),
        SegmentSpec::Image { value } => render_image_field(value, x, y, colours),
        SegmentSpec::Icon { name } if name == "plus" || name == "minus" => {
            render_mutator_icon(name, x, y)
        }
        SegmentSpec::Icon { name } if name == "quote_open" || name == "quote_close" => {
            let quote = if name == "quote_open" { "“" } else { "”" };
            format!(
                "<text class=\"nepo-field-text\" x=\"{}\" y=\"{}\" font-size=\"16\">{quote}</text>",
                fmt(x),
                fmt(y + FIELD_TEXT_BASELINE + 1.0),
            )
        }
        SegmentSpec::Icon { .. }
        | SegmentSpec::Block { .. }
        | SegmentSpec::InlineValue { .. }
        | SegmentSpec::PixelMatrix { .. } => {
            String::new()
        }
    }
}

/// `Blockly.MutatorPlus.drawIcon_` / `Blockly.MutatorMinus.drawIcon_`: a
/// transparent 16x16 hit target with the symbol drawn at two-thirds scale.
/// Open Roberta shows the plus wherever a block can grow — another condition,
/// another list item — and the minus on every part that can be taken away
/// again, which for a variable is its declaration.
fn render_mutator_icon(name: &str, x: f32, y: f32) -> String {
    let symbol = if name == "minus" {
        "M18 11h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12c1.104 0 2-.896 2-2s-.896-2-2-2z"
    } else {
        "M18 10h-4v-4c0-1.104-.896-2-2-2s-2 .896-2 2l.071 4h-4.071c-1.104 0-2 .896-2 2s.896 2 2 2l4.071-.071-.071 4.071c0 1.104.896 2 2 2s2-.896 2-2v-4.071l4 .071c1.104 0 2-.896 2-2s-.896-2-2-2z"
    };
    format!(
        "<g transform=\"translate({} {})\"><rect width=\"16\" height=\"16\" fill-opacity=\"0\"/><path d=\"{symbol}\" transform=\"scale(.67)\" fill=\"#fff\" fill-opacity=\".6\"/></g>",
        fmt(x),
        fmt(y),
    )
}

fn render_empty_inline_socket(
    x: f32,
    row_y: f32,
    checks: &[String],
    theme: &str,
    block_fill: &str,
) -> String {
    let stroke = single_check(checks)
        .map(|check| connection_colour(check, theme))
        .unwrap_or(block_fill);
    format!(
        "<path d=\"M {},{} {}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>",
        fmt(x + 6.0),
        fmt(row_y + 8.6),
        INPUT_TYPE_PATH,
        stroke,
        INPUT_TYPE_STROKE_WIDTH,
    )
}

/// Open Roberta's picture dropdown, without Open Roberta's picture.
///
/// The lab loads a 24x24 PNG per image from its media bundle. Shipping those
/// is not an option here, so the preview is drawn from the pixel pattern
/// instead — at the size the PNG would have had, so the surrounding geometry
/// is the real one either way. The arrow beside it is Blockly's: the field is
/// a dropdown, and its `textElement_` holds nothing but that arrow.
fn render_image_field(name: &str, x: f32, y: f32, colours: &crate::theme::BlockColours) -> String {
    let pixels = image_pixels(name);
    let cell = 3.0;
    // Centre the five cells in the picture: (24 - 5 * 3) / 2.
    let inset = (IMAGE_FIELD_PICTURE - crate::matrix::SIZE as f32 * cell) / 2.0;
    let mut svg = format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{size}\" height=\"{size}\" rx=\"3\" fill=\"{}\" fill-opacity=\"{}\"/>",
        fmt(x),
        fmt(y),
        colours.field_fill,
        colours.field_fill_opacity,
        size = fmt(IMAGE_FIELD_PICTURE),
    );
    for (row, pattern) in pixels.iter().enumerate() {
        for (column, on) in pattern.chars().enumerate() {
            if on == '#' {
                svg.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{c}\" height=\"{c}\" fill=\"#000\"/>",
                    fmt(x + inset + column as f32 * cell),
                    fmt(y + inset + row as f32 * cell),
                    c = fmt(cell),
                ));
            }
        }
    }
    // `arrow_.style.fill = '#ffffff'` — the arrow is a label, not field text,
    // so it follows the block's label colour into the print theme.
    svg.push_str(&format!(
        "<text class=\"nepo-label\" x=\"{}\" y=\"{}\">\u{25BE}</text>",
        fmt(x + IMAGE_FIELD_ARROW_X),
        fmt(y + IMAGE_FIELD_ARROW_Y),
    ));
    svg
}

fn image_pixels(name: &str) -> [&'static str; 5] {
    match name {
        "Herz" | "Heart" => [".#.#.", "#####", "#####", ".###.", "..#.."],
        "Lächeln" | "Smile" => [".....", ".#.#.", ".....", "#...#", ".###."],
        "Strichmännchen" | "Stick figure" => ["..#..", ".###.", "..#..", ".#.#.", "#...#"],
        "Giraffe" => ["..#..", ".###.", "..#..", ".#.#.", ".#.#."],
        "Regenschirm" | "Umbrella" => [".###.", "#####", "..#..", "..#..", ".###."],
        _ => [".....", ".....", ".....", ".....", "....."],
    }
}
