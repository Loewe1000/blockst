//! Layout and drawing for the Blockly family of shapes.
//!
//! Scratch lays a block out as one run of segments. Blockly builds it out of
//! rows, and the rules for those rows are what make a Blockly block look
//! like one: a value plugged into the middle of a row sits in a cut-out
//! socket and lifts the row to 35; a value at the end of a row hangs off the
//! block's right edge instead, one row per value; a C-block's foot is ten
//! pixels high and only as wide as its arm label plus the notch; the right
//! corners are square and the left ones are rounded only where nothing is
//! attached. None of that maps onto the Scratch renderer's single row, so
//! this module does the whole job for the Blockly family and `render.rs`
//! hands over at the block level.
//!
//! Every constant and rule here was read out of the running editor on
//! jwinf.de (`tests/fixtures/jwinf-reference.json`), not estimated. This
//! is the pre-2019 renderer; today's Blockly lays rows out differently and
//! lives in `blockly_modern.rs`, which shares the row plan built here.

use crate::geometry::{geometry, Geometry};
use crate::measure::text_width;
use crate::model::{BlockSpec, SegmentSpec};
use crate::palette::{colors_for, CategoryColors};
use crate::svg::escape_text;

/// Horizontal gap between items in a row, and the left/right padding.
const SEP_X: f32 = 10.0;
/// Height of the foot under a mouth, and of the arm between two mouths.
const ARM_H: f32 = 10.0;
/// Field boxes are 16 high and start 5 below the row in a 25 row.
const FIELD_H: f32 = 16.0;
const FIELD_Y: f32 = 5.0;
/// The puzzle tab: 8 wide, drawn 15 tall from 5 below the row top.
const TAB_W: f32 = 8.0;
const TAB_DOWN: &str = "c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5";
const TAB_UP: &str = "c 0,-10 -8,8 -8,-7.5 s 8,2.5 8,-7.5";
/// An empty inline socket is a hole this wide.
const EMPTY_SOCKET_W: f32 = 14.5;
/// Narrowest arm beside a mouth (no label).
const STATEMENT_EDGE_MIN: f32 = 20.0;
pub(crate) const DROPDOWN_ARROW: &str = " ▾";
/// Blockly's icons are 16px squares.
pub(crate) const ICON_W: f32 = 16.0;

#[derive(Clone)]
pub(crate) enum Item {
    Label(String),
    /// A 16px icon in the first row: "mutator", "warning" or "comment".
    Icon(String),
    Field { text: String, dropdown: bool },
    /// A value input. `None` is an empty hole; `Some` is the block plugged in.
    /// `boolean` tells an empty hole's shape apart where the renderer cares.
    Socket(Option<BlockSpec>, bool),
}

pub(crate) struct Row {
    pub(crate) items: Vec<Item>,
    /// A value input that ends the row and is cut into the block's right edge.
    pub(crate) external: Option<Option<BlockSpec>>,
    /// Whether that external input is a boolean slot.
    pub(crate) external_boolean: bool,
}

pub(crate) struct Mouth {
    pub(crate) label: Option<String>,
    pub(crate) body: Vec<BlockSpec>,
}

pub(crate) struct Plan {
    pub(crate) rows: Vec<Row>,
    pub(crate) mouths: Vec<Mouth>,
    pub(crate) is_value: bool,
    pub(crate) top_notch: bool,
    pub(crate) bottom_notch: bool,
}

fn is_operator(text: &str) -> bool {
    matches!(
        text.trim(),
        "=" | "==" | "≠" | "!=" | "<" | "≤" | "<=" | ">" | "≥" | ">=" | "+" | "-" | "−" | "×" | "*" | "÷" | "/" | "^" | "und" | "oder"
    )
}

/// Blockly draws icons as text; jwinf uses the arrows in its labels.
fn icon_glyph(name: &str) -> &'static str {
    match name {
        "turnRight" | "turn-right" | "arrow-right" => "↻",
        "turnLeft" | "turn-left" | "arrow-left" => "↺",
        "greenFlag" | "flag" | "green-flag" => "⚑",
        _ => "•",
    }
}

/// A literal in a value slot is a shadow block: a number is a `math_number`
/// in the mathe colour, a string a `text` block. Inside, the literal is a
/// field, which is what `slots = ["field"]` says.
fn shadow(value: &str, kind: &str) -> BlockSpec {
    let category = match kind {
        "number" => "mathe",
        "boolean" => "logik",
        _ => "text",
    };
    BlockSpec {
        shape: "reporter".to_string(),
        category: category.to_string(),
        line_number: None,
        segments: vec![SegmentSpec::Input { input: kind.to_string(), value: value.to_string(), color: String::new(), nested: None }],
        body: vec![],
        else_body: vec![],
        else_segments: vec![],
        mouth: None,
        slots: vec!["field".to_string()],
        inline: None,
        icon: None,
    }
}

pub(crate) fn plan(block: &BlockSpec) -> Plan {
    let mut items = Vec::new();
    // Icons come first in the first row. Today's Blockly no longer opens
    // a comment on a new procedure, so the modern layout skips that one.
    if let Some(icons) = &block.icon {
        for name in icons.split(',').map(str::trim).filter(|n| !n.is_empty()) {
            if name == "comment" && geometry().spacer_rows {
                continue;
            }
            items.push(Item::Icon(name.to_string()));
        }
    }
    let mut slot = 0usize;
    for segment in &block.segments {
        match segment {
            SegmentSpec::Text { value } => items.push(Item::Label(value.clone())),
            SegmentSpec::Icon { name } => items.push(Item::Label(icon_glyph(name).to_string())),
            SegmentSpec::Block { block: nested } => {
                let kind = block.slots.get(slot).map(String::as_str);
                slot += 1;
                // The parser hands a `(…)` that is not a number over as a
                // nested reporter. In a field slot — `setze Farbe (#f00)`,
                // `gehe (x) Schritte` — it is the text the author typed into
                // the field; `((…))` is how a block gets plugged in instead.
                let label_only = nested.category == "variables" && nested.segments.iter().all(|s| matches!(s, SegmentSpec::Text { .. }));
                if kind == Some("field") && label_only {
                    let text = nested.segments.iter().filter_map(|s| match s { SegmentSpec::Text { value } => Some(value.as_str()), _ => None }).collect::<Vec<_>>().join(" ");
                    items.push(Item::Field { text, dropdown: false });
                } else {
                    items.push(Item::Socket(Some(adopt((**nested).clone())), false));
                }
            }
            SegmentSpec::Input { input, value, nested, .. } => {
                let kind = block.slots.get(slot).map(String::as_str);
                slot += 1;
                if let Some(nested) = nested {
                    // The Scratch parser turns anything in parentheses that
                    // is not a number into a nested reporter. In a field slot
                    // — `setze Farbe (#f00)` — that reporter is just the
                    // text the author typed, so it becomes the field.
                    if kind == Some("field") && nested.category == "variables" && nested.segments.iter().all(|s| matches!(s, SegmentSpec::Text { .. })) {
                        let text = nested.segments.iter().filter_map(|s| match s { SegmentSpec::Text { value } => Some(value.as_str()), _ => None }).collect::<Vec<_>>().join(" ");
                        items.push(Item::Field { text, dropdown: false });
                    } else {
                        items.push(Item::Socket(Some(adopt((**nested).clone())), false));
                    }
                } else if input == "dropdown" || input == "dropdown-field" || kind == Some("dropdown") {
                    items.push(Item::Field { text: value.clone(), dropdown: true });
                } else if input == "boolean" || kind == Some("value") {
                    // A value slot: a literal becomes a shadow block, nothing
                    // stays an empty hole.
                    if value.is_empty() {
                        items.push(Item::Socket(None, input == "boolean"));
                    } else {
                        items.push(Item::Socket(Some(shadow(value, input)), false));
                    }
                } else {
                    items.push(Item::Field { text: value.clone(), dropdown: false });
                }
            }
        }
    }

    // The parser hands over one label per word; Blockly draws a run of words
    // as one label, ten pixels from its neighbours rather than from each
    // other. Merge the runs so "drehe um" is one label, measured as one.
    let mut merged: Vec<Item> = Vec::with_capacity(items.len());
    for item in items {
        match (merged.last_mut(), item) {
            (Some(Item::Label(previous)), Item::Label(next)) => {
                previous.push(' ');
                previous.push_str(&next);
            }
            (_, item) => merged.push(item),
        }
    }
    let mut items = merged;

    // An operator written between two values — `(1) = (2)`, `(a) + (b)`,
    // `<p> und <q>` — is Blockly's dropdown in the middle of a compare,
    // arithmetic or logic block, and is drawn as one.
    for index in 1..items.len().saturating_sub(1) {
        let between_sockets = matches!(items[index - 1], Item::Socket(..)) && matches!(items[index + 1], Item::Socket(..));
        if let Item::Label(text) = &items[index] {
            if between_sockets && is_operator(text) {
                items[index] = Item::Field { text: text.clone(), dropdown: true };
            }
        }
    }

    // Blockly's rule when nothing is declared: inline if a label follows a
    // value input, otherwise every value input ends a row of its own.
    let inline = block.inline.unwrap_or_else(|| {
        let mut seen_socket = false;
        let mut label_after = false;
        for item in &items {
            match item {
                Item::Socket(..) => seen_socket = true,
                Item::Label(_) | Item::Field { .. } if seen_socket => label_after = true,
                _ => {}
            }
        }
        label_after
    });

    let mut rows = Vec::new();
    if inline || !items.iter().any(|i| matches!(i, Item::Socket(..))) {
        rows.push(Row { items, external: None, external_boolean: false });
    } else {
        let mut current: Vec<Item> = Vec::new();
        for item in items {
            match item {
                Item::Socket(child, boolean) => {
                    rows.push(Row { items: std::mem::take(&mut current), external: Some(child), external_boolean: boolean });
                }
                other => current.push(other),
            }
        }
        if !current.is_empty() {
            rows.push(Row { items: current, external: None, external_boolean: false });
        }
    }

    let mut mouths = Vec::new();
    let has_mouth = block.shape.starts_with("c-block") || block.shape == "define-hat";
    if has_mouth {
        mouths.push(Mouth { label: block.mouth.clone(), body: block.body.clone() });
        let has_else = !block.else_body.is_empty() || !block.else_segments.is_empty();
        if has_else {
            let label = block
                .else_segments
                .iter()
                .find_map(|s| match s {
                    SegmentSpec::Text { value } => Some(value.clone()),
                    _ => None,
                });
            mouths.push(Mouth { label, body: block.else_body.clone() });
        }
    }

    let is_value = block.shape == "reporter" || block.shape == "boolean";
    Plan {
        rows,
        mouths,
        is_value,
        top_notch: !is_value && !block.shape.contains("hat"),
        bottom_notch: !is_value && !block.shape.contains("cap") && block.shape != "define-hat" && block.shape != "c-block hat",
    }
}

// ---------------------------------------------------------------------------
// Measuring
// ---------------------------------------------------------------------------

/// Height of a block's outline, without the 4px the bottom notch adds.
fn body_height(block: &BlockSpec) -> f32 {
    size(block).1
}

/// The block's own outline: width, and height without the bottom notch.
pub fn size(block: &BlockSpec) -> (f32, f32) {
    if geometry().zelos {
        return crate::blockly_zelos::size(block);
    }
    if geometry().spacer_rows {
        return crate::blockly_modern::size(block);
    }
    if let Some(inner) = unwrapped(block) {
        return size(&inner);
    }
    let p = plan(block);
    let g = geometry();
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;
    let row_count = p.rows.len();
    for (index, row) in p.rows.iter().enumerate() {
        let (w, h) = row_size(row, &g, index + 1 == row_count && p.mouths.is_empty(), !p.mouths.is_empty() && index + 1 == row_count, p.is_value);
        width = width.max(w);
        height += h;
    }
    if !p.mouths.is_empty() {
        width = width.max(statement_edge(&p.mouths) + g.notch_end);
    }
    for mouth in &p.mouths {
        height += mouth_height(mouth);
        height += ARM_H;
    }
    (width, height)
}

/// Where the mouths' inner wall sits. Blockly keeps one statement edge per
/// block: the widest of the mouth labels decides, and every mouth uses it.
fn statement_edge(mouths: &[Mouth]) -> f32 {
    mouths
        .iter()
        .map(|mouth| match mouth.label.as_deref() {
            Some(label) if !label.is_empty() => SEP_X + text_width(label) + SEP_X,
            _ => 0.0,
        })
        .fold(STATEMENT_EDGE_MIN, f32::max)
}

fn stack_height(blocks: &[BlockSpec]) -> f32 {
    blocks.iter().map(|b| body_height(b) + 1.0).sum()
}

fn mouth_height(mouth: &Mouth) -> f32 {
    let g = geometry();
    if mouth.body.is_empty() {
        g.row_height
    } else {
        g.row_height.max(stack_height(&mouth.body) + 4.0)
    }
}

pub(crate) fn field_advance(text: &str, dropdown: bool) -> f32 {
    text_width(text) + if dropdown { text_width(DROPDOWN_ARROW) } else { 0.0 }
}

/// Width and height of one row. `last` is the row right above the bottom
/// edge or a mouth, which Blockly draws one pixel shorter when it ends in an
/// external socket.
fn row_size(row: &Row, g: &Geometry, last: bool, before_mouth: bool, _is_value: bool) -> (f32, f32) {
    let mut cursor: f32 = 0.0;
    let mut tallest_socket: Option<f32> = None;
    let mut has_field = false;
    for item in &row.items {
        cursor += SEP_X;
        match item {
            Item::Label(text) => cursor += text_width(text),
            // 16 wide, and the old renderer adds a pixel after it.
            Item::Icon(_) => cursor += ICON_W + 1.0,
            Item::Field { text, dropdown } => {
                has_field = true;
                cursor += field_advance(text, *dropdown);
            }
            Item::Socket(child, _) => {
                let (socket_w, socket_h) = socket_size(child.as_ref());
                // The socket starts a tab's width past the gap, less the
                // pixel the cut-out overlaps the child by.
                cursor += TAB_W - 2.0;
                cursor += socket_w;
                tallest_socket = Some(tallest_socket.unwrap_or(0.0).max(socket_h));
            }
        }
    }
    let mut width = cursor + SEP_X;
    let height = match (&row.external, tallest_socket) {
        (Some(child), _) => {
            width = cursor + SEP_X + TAB_W;
            let child_h = child.as_ref().map(|c| body_height(c) + 1.0).unwrap_or(0.0);
            let h = g.row_height.max(child_h);
            if last || before_mouth { h - 1.0 } else { h }
        }
        (None, Some(socket_h)) => FIELD_Y + socket_h + 4.0,
        (None, None) if has_field => g.row_height,
        (None, None) => g.row_height - 1.0,
    };
    (width, height)
}

/// The cut-out an inline value sits in: width and height.
fn socket_size(child: Option<&BlockSpec>) -> (f32, f32) {
    match child {
        Some(c) => {
            let (w, h) = size(c);
            (w + 2.0, h + 2.0)
        }
        None => (EMPTY_SOCKET_W, 26.0),
    }
}

/// Size of a stack of blocks laid out one under the other, including
/// whatever hangs off their right edges or sits in their mouths.
pub fn stack_size(blocks: &[BlockSpec]) -> (f32, f32) {
    if geometry().zelos {
        return crate::blockly_zelos::stack_size(blocks);
    }
    if geometry().spacer_rows {
        return crate::blockly_modern::stack_size(blocks);
    }
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;
    for block in blocks {
        let (w, h) = extent(block);
        width = width.max(w);
        height += h + 1.0;
    }
    (width, height + 4.0)
}

/// The full footprint of a block: its outline plus external children and
/// the blocks in its mouths.
pub fn extent(block: &BlockSpec) -> (f32, f32) {
    if geometry().zelos {
        return crate::blockly_zelos::extent(block);
    }
    if geometry().spacer_rows {
        return crate::blockly_modern::extent(block);
    }
    if let Some(inner) = unwrapped(block) {
        return extent(&inner);
    }
    let p = plan(block);
    let (own_w, own_h) = size(block);
    let mut width = own_w;
    for row in &p.rows {
        if let Some(Some(child)) = &row.external {
            width = width.max(own_w + 1.0 + extent(child).0 + TAB_W);
        }
        for item in &row.items {
            if let Item::Socket(Some(child), _) = item {
                // Inline children are inside the outline already.
                let _ = child;
            }
        }
    }
    let edge = statement_edge(&p.mouths);
    for mouth in &p.mouths {
        for child in &mouth.body {
            width = width.max(edge + 1.0 + extent(child).0);
        }
    }
    (width, own_h)
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

fn darken(hex: &str, factor: f32) -> String {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return hex.to_string();
    }
    let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0) as f32;
    let f = |v: f32| ((v * factor).round().clamp(0.0, 255.0)) as u8;
    format!("#{:02x}{:02x}{:02x}", f(c(0)), f(c(2)), f(c(4)))
}

fn lighten(hex: &str, amount: f32) -> String {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return hex.to_string();
    }
    let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0) as f32;
    let f = |v: f32| ((v + (255.0 - v) * amount).round().clamp(0.0, 255.0)) as u8;
    format!("#{:02x}{:02x}{:02x}", f(c(0)), f(c(2)), f(c(4)))
}

/// Draw a stack: blocks one under the other, each told whether something is
/// attached above and below it, because that decides which left corners are
/// rounded.
pub fn render_stack(blocks: &[BlockSpec], theme: &str) -> (String, f32, f32) {
    if geometry().zelos {
        return crate::blockly_zelos::render_stack(blocks, theme);
    }
    if geometry().spacer_rows {
        return crate::blockly_modern::render_stack(blocks, theme);
    }
    let mut svg = String::new();
    let mut y: f32 = 0.0;
    let mut width: f32 = 0.0;
    let count = blocks.len();
    for (index, block) in blocks.iter().enumerate() {
        let (block_svg, w, body_h) = render_block(block, theme, index == 0, index + 1 == count);
        svg.push_str(&format!("<g transform=\"translate(0 {y})\">{block_svg}</g>"));
        width = width.max(w);
        y += body_h + 1.0;
    }
    (svg, width, y + 4.0)
}

/// Draw one block. Returns the SVG, the footprint width, and the outline
/// height without the bottom notch (what the next block in a stack is placed
/// after).
/// An unrecognised reporter comes out of the parser as a Scratch variable
/// getter, category "variables". The Blockly profiles have no such
/// category; a bare `(x)` on a jwinf sheet is a variable too, so it takes
/// the profile's own name for that.
fn adopt(mut block: BlockSpec) -> BlockSpec {
    if block.category == "variables" && !geometry().zelos {
        block.category = "variablen".to_string();
    }
    block
}

/// A line that is nothing but one value — `(Quadratwurzel (9)) ::mathe` —
/// arrives as a stack wrapped around the value block, with the category
/// suffix on the wrapper. There is no such stack in Blockly: use the value
/// itself, and let it take the wrapper's category if it has none.
pub(crate) fn unwrapped(block: &BlockSpec) -> Option<BlockSpec> {
    if block.shape != "stack" || block.segments.len() != 1 {
        return None;
    }
    let mut inner = match &block.segments[0] {
        SegmentSpec::Input { nested: Some(nested), .. } => (**nested).clone(),
        SegmentSpec::Block { block } => (**block).clone(),
        _ => return None,
    };
    // The suffix sits on the wrapper; an unrecognised inner value has only
    // the parser's default. The author's category wins.
    if block.category != "obsolete" && (inner.category == "obsolete" || inner.category == "variables" || inner.category.is_empty()) {
        inner.category = block.category.clone();
    }
    Some(adopt(inner))
}

pub fn render_block(block: &BlockSpec, theme: &str, first: bool, last: bool) -> (String, f32, f32) {
    if geometry().zelos {
        return crate::blockly_zelos::render_block(block, theme, first, last);
    }
    if geometry().spacer_rows {
        return crate::blockly_modern::render_block(block, theme, first, last);
    }
    if let Some(inner) = unwrapped(block) {
        return render_block(&inner, theme, first, last);
    }
    let g = geometry();
    let p = plan(block);
    let colors = colors_for(&block.category, theme);
    let (own_w, body_h) = size(block);
    let (extent_w, _) = extent(block);
    let r = g.corner_radius;

    // --- outline ---------------------------------------------------------
    // The right edge is collected as a list of vertical runs first: with
    // rounded right corners (today's Blockly) the top-right arc takes the
    // first `r` pixels of the first row and the bottom-right arc the last
    // `r` of whatever run ends the block, and it is easier to trim those
    // after the fact than to special-case every row kind.
    let rounded_right = !g.square_right_corners;
    let round_top_left = !p.is_value && first;
    let mut d = String::new();
    if p.is_value {
        if rounded_right {
            d.push_str(&format!("m 0,{r} A {r},{r} 0 0,1 {r},0"));
        } else {
            d.push_str("m 0,0");
        }
    } else if round_top_left {
        d.push_str(&format!("m 0,{r} A {r},{r} 0 0,1 {r},0"));
    } else {
        d.push_str("m 0,0");
    }
    if p.top_notch {
        d.push_str(&format!(" H {} {}", g.notch_start, notch(&g, 1.0)));
    }
    if rounded_right {
        d.push_str(&format!(" H {} a {r},{r} 0 0,1 {r},{r}", own_w - r));
    } else {
        d.push_str(&format!(" H {own_w}"));
    }

    // right edge, row by row: each entry is (prefix, trailing vertical run)
    let mut runs: Vec<(String, f32)> = Vec::new();
    let mut y: f32 = 0.0;
    let row_count = p.rows.len();
    let mut row_tops: Vec<f32> = Vec::new();
    let mut row_heights: Vec<f32> = Vec::new();
    for (index, row) in p.rows.iter().enumerate() {
        let (_, h) = row_size(row, &g, index + 1 == row_count && p.mouths.is_empty(), !p.mouths.is_empty() && index + 1 == row_count, p.is_value);
        row_tops.push(y);
        row_heights.push(h);
        if row.external.is_some() {
            runs.push((format!(" v {} {}", FIELD_Y, TAB_DOWN), h - FIELD_Y - 15.0));
        } else {
            runs.push((String::new(), h));
        }
        y += h;
    }
    // Every mouth shares the block's statement edge; the arm under a mouth
    // runs back out to the block's full width.
    let edge = statement_edge(&p.mouths);
    let mut mouth_tops: Vec<(f32, f32)> = Vec::new(); // (top, height)
    for mouth in &p.mouths {
        let mh = mouth_height(mouth);
        mouth_tops.push((y, mh));
        runs.push((
            format!(
                " H {} {} h -7 a {r},{r} 0 0,0 -{r},{r} v {} a {r},{r} 0 0,0 {r},{r} H {own_w}",
                edge + g.notch_end,
                notch(&g, -1.0),
                mh - 2.0 * r,
            ),
            ARM_H,
        ));
        y += mh + ARM_H;
    }
    if rounded_right {
        if let Some(first_run) = runs.first_mut() {
            first_run.1 -= r;
        }
        if let Some(last_run) = runs.last_mut() {
            last_run.1 -= r;
        }
    }
    for (prefix, run) in &runs {
        d.push_str(prefix);
        if *run != 0.0 {
            d.push_str(&format!(" v {run}"));
        }
    }
    if rounded_right {
        d.push_str(&format!(" a {r},{r} 0 0,1 -{r},{r}"));
    }
    // bottom edge
    if p.is_value {
        if rounded_right {
            d.push_str(&format!(" H {r} a {r},{r} 0 0,1 -{r},-{r} V {} {} z", FIELD_Y + 15.0, TAB_UP));
        } else {
            d.push_str(&format!(" H 0 V {} {} z", FIELD_Y + 15.0, TAB_UP));
        }
    } else {
        if p.bottom_notch {
            d.push_str(&format!(" H {} {}", g.notch_end - 0.5, notch(&g, -1.0)));
        }
        if last {
            d.push_str(&format!(" H {r} a {r},{r} 0 0,1 -{r},-{r} z"));
        } else {
            d.push_str(" H 0 z");
        }
    }
    // inline socket cut-outs, as sub-paths of the same outline, and the
    // light line along each cut-out's right wall and floor
    let mut cutouts = String::new();
    let mut highlight_inline = String::new();

    // --- items ----------------------------------------------------------
    let mut content = String::new();
    for (index, row) in p.rows.iter().enumerate() {
        let top = row_tops[index];
        let h = row_heights[index];
        let mut cursor: f32 = 0.0;
        let field_y = top + FIELD_Y + ((h - g.row_height).max(0.0) / 2.0).floor();
        let baseline = field_y + 12.5;
        for item in &row.items {
            cursor += SEP_X;
            match item {
                Item::Label(text) => {
                    content.push_str(&label(&colors, text, cursor, baseline, theme));
                    cursor += text_width(text);
                }
                Item::Icon(kind) => {
                    content.push_str(&icon_svg(kind, cursor, top + FIELD_Y, theme));
                    cursor += ICON_W + 1.0;
                }
                Item::Field { text, dropdown } => {
                    let shown = if *dropdown { format!("{text}{DROPDOWN_ARROW}") } else { text.clone() };
                    let advance = field_advance(text, *dropdown);
                    content.push_str(&format!(
                        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{FIELD_H}\" rx=\"4\" ry=\"4\" fill=\"#ffffff\"/>",
                        cursor - 5.0,
                        field_y,
                        advance + 10.0
                    ));
                    content.push_str(&format!(
                        "<text class=\"sb-input-text\" x=\"{}\" y=\"{}\" style=\"fill:#000000\">{}</text>",
                        cursor,
                        baseline,
                        escape_text(&shown)
                    ));
                    cursor += advance;
                }
                Item::Socket(child, _) => {
                    let (socket_w, socket_h) = socket_size(child.as_ref());
                    let left = cursor + TAB_W - 2.0;
                    let socket_top = top + FIELD_Y;
                    cutouts.push_str(&format!(
                        " M {},{socket_top} h -{socket_w} v 5 {TAB_DOWN} v {} h {socket_w} z",
                        left + socket_w,
                        socket_h - 20.0
                    ));
                    highlight_inline.push_str(&format!(
                        " M {},{} v {socket_h} h -{socket_w} M {},{} l {},-2.1",
                        left + socket_w + 0.5,
                        socket_top + 0.5,
                        left - 5.1,
                        socket_top + 15.0 + 5.0 - 0.7,
                        TAB_W * 0.46
                    ));
                    if let Some(child) = child {
                        let (child_svg, _, _) = render_block(child, theme, true, true);
                        content.push_str(&format!(
                            "<g transform=\"translate({} {})\">{child_svg}</g>",
                            left + 1.0,
                            socket_top + 1.0
                        ));
                    }
                    cursor = left + socket_w;
                }
            }
        }
        if let Some(Some(child)) = &row.external {
            let (child_svg, _, _) = render_block(child, theme, true, true);
            content.push_str(&format!("<g transform=\"translate({} {top})\">{child_svg}</g>", own_w + 1.0));
        }
    }
    for (index, mouth) in p.mouths.iter().enumerate() {
        let (top, _) = mouth_tops[index];
        if let Some(label_text) = mouth.label.as_deref().filter(|l| !l.is_empty()) {
            content.push_str(&label(&colors, label_text, SEP_X, top + FIELD_Y + 12.5, theme));
        }
        if !mouth.body.is_empty() {
            let (stack_svg, _, _) = render_stack(&mouth.body, theme);
            content.push_str(&format!("<g transform=\"translate({} {})\">{stack_svg}</g>", edge + 1.0, top + 1.0));
        }
    }

    // --- assemble --------------------------------------------------------
    let mut svg = String::new();
    let outline = format!("{d}{cutouts}");
    if g.bevel {
        // Classic Blockly: a darker copy one pixel down and right, the block
        // itself without a stroke, and a lighter line along the edges that
        // face the light — the top edge with its notch, the tab glints, the
        // floor of each mouth, and the left edge with its corners. The
        // highlight follows Blockly's block_render_svg.js line for line.
        let dark = darken(&colors.fill, 0.8);
        let light = lighten(&colors.fill, 0.3);
        svg.push_str(&format!("<path d=\"{outline}\" fill=\"{dark}\" fill-rule=\"evenodd\" transform=\"translate(1 1)\"/>"));
        svg.push_str(&format!("<path d=\"{outline}\" fill=\"{}\" fill-rule=\"evenodd\"/>", colors.fill));
        let d45_inside = (1.0 - std::f32::consts::FRAC_1_SQRT_2) * (r - 0.5) + 0.5;
        let d45_outside = (1.0 - std::f32::consts::FRAC_1_SQRT_2) * (r + 0.5) - 0.5;
        let mut hl = String::new();
        // top edge
        if round_top_left {
            hl.push_str(&format!("m 0.5,{} A {},{} 0 0,1 {r},0.5", r - 0.5, r - 0.5, r - 0.5));
        } else {
            hl.push_str("m 0.5,0.5");
        }
        if p.top_notch {
            hl.push_str(&format!(" H {} {}", g.notch_start, notch(&g, 1.0)));
        }
        hl.push_str(&format!(" H {}", own_w - 0.5));
        // a glint at the foot of every external tab
        for (index, row) in p.rows.iter().enumerate() {
            if row.external.is_some() {
                hl.push_str(&format!(" M {},{} l {},-2.1", own_w - 5.0, row_tops[index] + FIELD_Y + 15.0 - 0.7, TAB_W * 0.46));
            }
        }
        // the floor of each mouth, from its inner corner out to the edge
        for (top, mh) in &mouth_tops {
            hl.push_str(&format!(
                " M {},{} a {},{} 0 0,0 {},{} H {}",
                edge + d45_outside,
                top + mh - d45_outside,
                r + 0.5,
                r + 0.5,
                r - d45_outside,
                d45_outside + 0.5,
                own_w - 0.5
            ));
        }
        // bottom-left corner and the left edge, up to where the top began
        if p.is_value {
            hl.push_str(&format!(
                " M 0.5,{} V {} m {},-0.5 q {},-5.5 0,-11 m {},1 V 0.5 H 1",
                body_h - 0.5,
                FIELD_Y + 15.0 - 1.5,
                -TAB_W * 0.92,
                -TAB_W * 0.19,
                TAB_W * 0.92
            ));
        } else {
            let top_of_left = if round_top_left { r } else { 0.5 };
            if last {
                hl.push_str(&format!(
                    " M {},{} A {},{} 0 0,1 0.5,{} V {top_of_left}",
                    d45_inside,
                    body_h - d45_inside,
                    r - 0.5,
                    r - 0.5,
                    body_h - r
                ));
            } else {
                hl.push_str(&format!(" M 0.5,{} V {top_of_left}", body_h - 0.5));
            }
        }
        hl.push_str(&highlight_inline);
        svg.push_str(&format!("<path d=\"{hl}\" fill=\"none\" stroke=\"{light}\" stroke-width=\"1\"/>"));
    } else {
        svg.push_str(&format!("<path d=\"{outline}\" fill=\"{}\" stroke=\"{}\" fill-rule=\"evenodd\"/>", colors.fill, colors.stroke));
    }
    svg.push_str(&content);
    (svg, extent_w, body_h)
}

pub(crate) fn notch(g: &Geometry, dir: f32) -> String {
    let ramp = (g.notch_end - g.notch_start - g.notch_inner) / 2.0;
    format!("l {},{} {},0 {},-{}", dir * ramp, g.notch_depth, dir * g.notch_inner, dir * ramp, g.notch_depth)
}

/// One of Blockly's icons, drawn as the editor draws it: blue shape with a
/// white symbol, at 60% opacity because that is how an unhovered icon
/// looks. Markup copied from the editor's DOM.
pub(crate) fn icon_svg(kind: &str, x: f32, y: f32, theme: &str) -> String {
    let (shape, symbol) = if theme == "grayscale" || theme == "print" { ("#555555", "#ffffff") } else { ("#0000ff", "#ffffff") };
    let inner = match kind {
        "mutator" => format!(
            "<rect rx=\"4\" ry=\"4\" height=\"16\" width=\"16\" fill=\"{shape}\" stroke=\"#ffffff\"/>\
             <path fill=\"{symbol}\" d=\"m4.203,7.296 0,1.368 -0.92,0.677 -0.11,0.41 0.9,1.559 0.41,0.11 1.043,-0.457 1.187,0.683 0.127,1.134 0.3,0.3 1.8,0 0.3,-0.299 0.127,-1.138 1.185,-0.682 1.046,0.458 0.409,-0.11 0.9,-1.559 -0.11,-0.41 -0.92,-0.677 0,-1.366 0.92,-0.677 0.11,-0.41 -0.9,-1.559 -0.409,-0.109 -1.046,0.458 -1.185,-0.682 -0.127,-1.138 -0.3,-0.299 -1.8,0 -0.3,0.3 -0.126,1.135 -1.187,0.682 -1.043,-0.457 -0.41,0.11 -0.899,1.559 0.108,0.409z\"/>\
             <circle r=\"2.7\" cx=\"8\" cy=\"8\" fill=\"{shape}\" stroke=\"#ffffff\"/>"
        ),
        "warning" => format!(
            "<path fill=\"{shape}\" stroke=\"#ffffff\" d=\"M2,15Q-1,15 0.5,12L6.5,1.7Q8,-1 9.5,1.7L15.5,12Q17,15 14,15z\"/>\
             <path fill=\"{symbol}\" d=\"m7,4.8v3.16l0.27,2.27h1.46l0.27,-2.27v-3.16z\"/>\
             <rect fill=\"{symbol}\" x=\"7\" y=\"11\" height=\"2\" width=\"2\"/>"
        ),
        "comment" => format!(
            "<circle r=\"8\" cx=\"8\" cy=\"8\" fill=\"{shape}\" stroke=\"#ffffff\"/>\
             <path fill=\"{symbol}\" d=\"m6.8,10h2c0.003,-0.617 0.271,-0.962 0.633,-1.266 2.875,-2.405 0.607,-5.534 -3.765,-3.874v1.7c3.12,-1.657 3.698,0.118 2.336,1.25 -1.201,0.998 -1.201,1.528 -1.204,2.19z\"/>\
             <rect fill=\"{symbol}\" x=\"6.8\" y=\"10.78\" height=\"2\" width=\"2\"/>"
        ),
        _ => return String::new(),
    };
    format!("<g transform=\"translate({x} {y})\" opacity=\"0.6\">{inner}</g>")
}

pub(crate) fn label(colors: &CategoryColors, text: &str, x: f32, baseline: f32, theme: &str) -> String {
    let fill = if theme == "grayscale" { format!("style=\"fill:{}\"", colors.text) } else { format!("fill=\"{}\"", colors.text) };
    format!("<text class=\"sb-label\" x=\"{x}\" y=\"{baseline}\" {fill}>{}</text>", escape_text(text))
}
