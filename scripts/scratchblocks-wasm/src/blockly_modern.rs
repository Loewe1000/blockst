//! Layout and drawing for today's Blockly — the base renderer that thrasos
//! is, as opposed to the pre-2019 one in `blockly.rs`.
//!
//! Where the old renderer builds a block out of 25px rows with everything
//! inside them, the current one stacks measured rows between a 5px top row
//! and a 5px bottom row (10 under a mouth), with spacer rows between: 10
//! between two plain rows, 4 before a mouth, 10 between two mouths. A row is
//! as tall as its tallest element — 18 for text and fields, 26 for an empty
//! inline socket, the child's height for a filled one, a child's height
//! less ten for an external one — and elements sit on the row's centreline,
//! except that text in a row with an inline socket or a mouth hangs five
//! pixels below the top instead. Horizontal gaps depend on what meets what:
//! 10 around labels, 5 around editable fields, 8 before an inline socket, 3
//! between a field and a socket, 20 before an unlabelled mouth.
//!
//! Every number here was read out of Blockly 12.3.1's RenderInfo with the
//! thrasos renderer (`tests/fixtures/blockly-modern-reference.json`).

use crate::blockly::{field_advance, icon_svg, label, notch, plan, unwrapped, Item, Mouth, Plan, Row, DROPDOWN_ARROW, ICON_W};
use crate::geometry::geometry;
use crate::measure::text_width;
use crate::model::BlockSpec;
use crate::palette::colors_for;
use crate::svg::escape_text;

const TOP_ROW: f32 = 5.0;
const TOP_ROW_BEFORE_MOUTH: f32 = 10.0;
const BOTTOM_ROW: f32 = 5.0;
const BOTTOM_ROW_AFTER_MOUTH: f32 = 10.0;
/// Text and field elements are 18 high; a field's box is drawn that tall.
const TEXT_H: f32 = 18.0;
const TAB_W: f32 = 8.0;
const TAB_H: f32 = 15.0;
const TAB_DOWN: &str = "c 0,10 -8,-8 -8,7.5 s 8,-2.5 8,7.5";
const TAB_UP: &str = "c 0,-10 -8,8 -8,-7.5 s 8,2.5 8,-7.5";
/// An external input is a tab plus two pixels of padding.
const EXTERNAL_W: f32 = TAB_W + 2.0;
const EMPTY_INLINE_W: f32 = 14.5;
const EMPTY_INLINE_H: f32 = 26.0;
const EMPTY_MOUTH_H: f32 = 24.0;
const STATEMENT_PAD_LEFT: f32 = 20.0;
const CORNER: f32 = 8.0;
/// Text in a row with an inline socket or a mouth sits this far below the
/// top instead of on the centreline.
const TALL_INPUT_FIELD_OFFSET_Y: f32 = 5.0;
/// Where the alphabetic baseline of an 11pt label sits below its centre.
const BASELINE_BELOW_CENTRE: f32 = 4.5;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Label,
    Icon,
    Editable,
    Inline,
    External,
    Statement,
}

/// thrasos' `getInRowSpacing_`, as measured.
fn spacing(prev: Option<Kind>, next: Option<Kind>) -> f32 {
    use Kind::*;
    match (prev, next) {
        (None, Some(Editable)) => 5.0,
        (None, Some(Inline)) => 8.0,
        (None, Some(Statement)) => STATEMENT_PAD_LEFT,
        (None, _) => 10.0,
        // an icon-only row gets extra room so the block shape stays clear
        (Some(Icon), None) => 21.0,
        (Some(Icon), _) => 10.0,
        (Some(Editable), None | Some(Statement)) => 5.0,
        (Some(Label), None | Some(Statement)) => 10.0,
        (Some(Inline), None) => 10.0,
        (Some(External), None) | (Some(Statement), None) => 0.0,
        (Some(Editable), Some(Inline | External)) => 3.0,
        (Some(Label), Some(Inline | External)) => 8.0,
        (Some(Inline), Some(Label)) => 10.0,
        (Some(Inline), Some(Editable)) => 5.0,
        (Some(Label), Some(Label)) | (Some(Editable), Some(Editable)) => 10.0,
        _ => 5.0,
    }
}

fn kind(item: &Item) -> Kind {
    match item {
        Item::Label(_) => Kind::Label,
        Item::Icon(_) => Kind::Icon,
        Item::Field { .. } | Item::Editor { .. } => Kind::Editable,
        Item::Socket(..) => Kind::Inline,
    }
}

fn item_size(item: &Item) -> (f32, f32) {
    match item {
        Item::Label(text) => (text_width(text), TEXT_H),
        // measured as 17 by 17 in RenderInfo, drawn 16 wide
        Item::Icon(_) => (ICON_W + 1.0, ICON_W + 1.0),
        Item::Field { text, dropdown } => (field_advance(text, *dropdown) + 10.0, TEXT_H),
        Item::Editor { text, .. } => (field_advance(text, false) + 10.0, TEXT_H),
        Item::Socket(None, _) => (EMPTY_INLINE_W + TAB_W, EMPTY_INLINE_H),
        Item::Socket(Some(child), _) => {
            let (w, h) = size(child);
            (w + TAB_W, h)
        }
    }
}

struct PlacedItem {
    x: f32,
    w: f32,
    h: f32,
}

struct RowLayout {
    y: f32,
    h: f32,
    /// Natural width, before the row is stretched to the block's width.
    w: f32,
    items: Vec<PlacedItem>,
    tall: bool,
}

struct MouthLayout {
    y: f32,
    h: f32,
}

struct Layout {
    rows: Vec<RowLayout>,
    mouths: Vec<MouthLayout>,
    edge: f32,
    w: f32,
    body_h: f32,
}

fn measure_row(row: &Row) -> RowLayout {
    let mut x: f32 = 0.0;
    let mut h: f32 = 0.0;
    let mut prev: Option<Kind> = None;
    let mut items = Vec::with_capacity(row.items.len());
    let tall = row.items.iter().any(|i| matches!(i, Item::Socket(..)));
    for item in &row.items {
        let k = kind(item);
        x += spacing(prev, Some(k));
        let (iw, ih) = item_size(item);
        items.push(PlacedItem { x, w: iw, h: ih });
        x += iw;
        h = h.max(ih);
        prev = Some(k);
    }
    match &row.external {
        Some(child) => {
            x += spacing(prev, Some(Kind::External));
            x += EXTERNAL_W;
            let eh = match child {
                Some(c) => size(c).1 - 2.0 * TOP_ROW,
                None => TAB_H,
            };
            h = h.max(eh);
        }
        None => x += spacing(prev, None),
    }
    RowLayout { y: 0.0, h, w: x, items, tall }
}

fn statement_edge(mouths: &[Mouth]) -> f32 {
    mouths
        .iter()
        .map(|m| match m.label.as_deref() {
            Some(l) if !l.is_empty() => spacing(None, Some(Kind::Label)) + text_width(l) + spacing(Some(Kind::Label), Some(Kind::Statement)),
            _ => STATEMENT_PAD_LEFT,
        })
        .fold(0.0, f32::max)
}

fn layout(block: &BlockSpec, p: &Plan) -> Layout {
    let _ = block;
    let g = geometry();
    let mut rows: Vec<RowLayout> = p.rows.iter().map(measure_row).collect();
    let edge = statement_edge(&p.mouths);
    let mut w = rows.iter().map(|r| r.w).fold(0.0, f32::max);
    if !p.mouths.is_empty() {
        w = w.max(edge + g.notch_end);
    }
    let mut y = if rows.is_empty() && !p.mouths.is_empty() { TOP_ROW_BEFORE_MOUTH } else { TOP_ROW };
    for (index, row) in rows.iter_mut().enumerate() {
        if index > 0 {
            y += 10.0;
        }
        row.y = y;
        y += row.h;
    }
    let mut mouths = Vec::with_capacity(p.mouths.len());
    for (index, mouth) in p.mouths.iter().enumerate() {
        y += if index == 0 { if rows.is_empty() { 0.0 } else { 4.0 } } else { 10.0 };
        let h = if mouth.body.is_empty() { EMPTY_MOUTH_H } else { stack_size(&mouth.body).1 };
        mouths.push(MouthLayout { y, h });
        y += h;
    }
    y += if p.mouths.is_empty() { BOTTOM_ROW } else { BOTTOM_ROW_AFTER_MOUTH };
    Layout { rows, mouths, edge, w, body_h: y }
}

/// The block's own outline: width and height, without the tab on the left
/// of a value block and without the notch under a statement block.
pub fn size(block: &BlockSpec) -> (f32, f32) {
    if let Some(inner) = unwrapped(block) {
        return size(&inner);
    }
    let p = plan(block);
    let l = layout(block, &p);
    (l.w, l.body_h)
}

/// A stack's width and height, the height including the notch under it.
pub fn stack_size(blocks: &[BlockSpec]) -> (f32, f32) {
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;
    for block in blocks {
        let (w, h) = extent(block);
        width = width.max(w);
        height += h;
    }
    (width, height + 4.0)
}

/// The full footprint of a block: outline plus external children and the
/// stacks in its mouths. A value block's own tab is counted in.
pub fn extent(block: &BlockSpec) -> (f32, f32) {
    if let Some(inner) = unwrapped(block) {
        return extent(&inner);
    }
    let p = plan(block);
    let l = layout(block, &p);
    let mut width = l.w;
    for row in &p.rows {
        if let Some(Some(child)) = &row.external {
            width = width.max(l.w + extent(child).0);
        }
    }
    for mouth in &p.mouths {
        for child in &mouth.body {
            width = width.max(l.edge + extent(child).0);
        }
    }
    if p.is_value {
        width += TAB_W;
    }
    (width, l.body_h)
}

pub fn render_stack(blocks: &[BlockSpec], theme: &str) -> (String, f32, f32) {
    let mut svg = String::new();
    let mut y: f32 = 0.0;
    let mut width: f32 = 0.0;
    let count = blocks.len();
    for (index, block) in blocks.iter().enumerate() {
        let (block_svg, w, body_h) = render_block(block, theme, index == 0, index + 1 == count);
        svg.push_str(&format!("<g transform=\"translate(0 {y})\">{block_svg}</g>"));
        width = width.max(w);
        y += body_h;
    }
    (svg, width, y + 4.0)
}

/// Draw one block. Returns the SVG, the footprint width and the outline
/// height (what the next block in a stack is placed after). A value block
/// is drawn with its body at x = 0 and its tab hanging out to the left.
pub fn render_block(block: &BlockSpec, theme: &str, first: bool, last: bool) -> (String, f32, f32) {
    if let Some(inner) = unwrapped(block) {
        return render_block(&inner, theme, first, last);
    }
    let g = geometry();
    let p = plan(block);
    let l = layout(block, &p);
    let colors = colors_for(&block.category, theme);
    let w = l.w;
    let r = CORNER;

    // --- outline ---------------------------------------------------------
    let mut d = String::new();
    let round_top_left = !p.is_value && first;
    if round_top_left {
        d.push_str(&format!("m 0,{r} a {r} {r} 0 0,1 {r},-{r}"));
    } else {
        d.push_str("m 0,0");
    }
    if p.top_notch {
        d.push_str(&format!(" H {} {}", g.notch_start, notch(&g, 1.0)));
    }
    d.push_str(&format!(" H {w}"));
    let mut cutouts = String::new();
    for (row, row_l) in p.rows.iter().zip(&l.rows) {
        if row.external.is_some() {
            d.push_str(&format!(" V {} {TAB_DOWN} v {}", row_l.y, row_l.h - TAB_H));
        }
        // Stretching a row to the block's width puts the slack before an
        // external input, so items keep their natural positions.
        for (item, placed) in row.items.iter().zip(&row_l.items) {
            if let Item::Socket(child, _) = item {
                let (ew, eh) = (placed.w, placed.h);
                let top = row_l.y + ((row_l.h - eh) / 2.0).floor();
                let _ = child;
                cutouts.push_str(&format!(" M {},{top} v 5 {TAB_DOWN} v {} h {} v -{eh} z", placed.x + TAB_W, eh - 20.0, ew - TAB_W));
            }
        }
    }
    for m in &l.mouths {
        d.push_str(&format!(
            " V {} H {} {} h -7 a {r} {r} 0 0,0 -{r},{r} v {} a {r} {r} 0 0,0 {r},{r} H {w}",
            m.y,
            l.edge + g.notch_end,
            notch(&g, -1.0),
            m.h - 2.0 * r
        ));
    }
    d.push_str(&format!(" V {}", l.body_h));
    if p.is_value {
        d.push_str(&format!(" H 0 V {} {TAB_UP} z", TOP_ROW + TAB_H));
    } else {
        if p.bottom_notch {
            d.push_str(&format!(" H {} {}", g.notch_end, notch(&g, -1.0)));
        }
        if last {
            d.push_str(&format!(" H {r} a {r} {r} 0 0,1 -{r},-{r} z"));
        } else {
            d.push_str(" H 0 z");
        }
    }

    // --- content ---------------------------------------------------------
    let mut content = String::new();
    for (row, row_l) in p.rows.iter().zip(&l.rows) {
        for (item, placed) in row.items.iter().zip(&row_l.items) {
            let centre = if row_l.tall && placed.h + TALL_INPUT_FIELD_OFFSET_Y <= row_l.h {
                row_l.y + placed.h / 2.0 + TALL_INPUT_FIELD_OFFSET_Y
            } else {
                row_l.y + row_l.h / 2.0
            };
            match item {
                Item::Label(text) => content.push_str(&label(&colors, text, placed.x, centre + BASELINE_BELOW_CENTRE, theme)),
                Item::Icon(kind) => content.push_str(&icon_svg(kind, placed.x, centre - placed.h / 2.0, theme)),
                Item::Editor { text, .. } => {
                    content.push_str(&format!(
                        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{TEXT_H}\" rx=\"4\" ry=\"4\" fill=\"#ffffff\"/>",
                        placed.x,
                        centre - TEXT_H / 2.0,
                        placed.w
                    ));
                    content.push_str(&format!("<text class=\"sb-input-text\" x=\"{}\" y=\"{}\" style=\"fill:#000000\"{}>{}</text>", placed.x + 5.0, centre + BASELINE_BELOW_CENTRE, crate::measure::fit(text), escape_text(text)));
                }
                Item::Field { text, dropdown } => {
                    let shown = if *dropdown { format!("{text}{DROPDOWN_ARROW}") } else { text.clone() };
                    content.push_str(&format!(
                        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{TEXT_H}\" rx=\"4\" ry=\"4\" fill=\"#ffffff\"/>",
                        placed.x,
                        centre - TEXT_H / 2.0,
                        placed.w
                    ));
                    content.push_str(&format!(
                        "<text class=\"sb-input-text\" x=\"{}\" y=\"{}\" style=\"fill:#000000\">{}</text>",
                        placed.x + 5.0,
                        centre + BASELINE_BELOW_CENTRE,
                        escape_text(&shown)
                    ));
                }
                Item::Socket(Some(child), _) => {
                    let top = row_l.y + ((row_l.h - placed.h) / 2.0).floor();
                    let (child_svg, _, _) = render_block(child, theme, true, true);
                    content.push_str(&format!("<g transform=\"translate({} {top})\">{child_svg}</g>", placed.x + TAB_W));
                }
                Item::Socket(None, _) => {}
            }
        }
        if let Some(Some(child)) = &row.external {
            let (child_svg, _, _) = render_block(child, theme, true, true);
            content.push_str(&format!("<g transform=\"translate({w} {})\">{child_svg}</g>", row_l.y - TOP_ROW));
        }
    }
    for (mouth, m) in p.mouths.iter().zip(&l.mouths) {
        if let Some(text) = mouth.label.as_deref().filter(|t| !t.is_empty()) {
            let centre = m.y + TEXT_H / 2.0 + TALL_INPUT_FIELD_OFFSET_Y;
            content.push_str(&label(&colors, text, spacing(None, Some(Kind::Label)), centre + BASELINE_BELOW_CENTRE, theme));
        }
        if !mouth.body.is_empty() {
            let (stack_svg, _, _) = render_stack(&mouth.body, theme);
            content.push_str(&format!("<g transform=\"translate({} {})\">{stack_svg}</g>", l.edge, m.y));
        }
    }

    let mut svg = String::new();
    svg.push_str(&format!(
        "<path d=\"{d}{cutouts}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\" fill-rule=\"evenodd\"/>",
        colors.fill, colors.stroke
    ));
    svg.push_str(&content);
    let (extent_w, _) = extent(block);
    (svg, extent_w, l.body_h)
}
