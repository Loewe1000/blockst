//! Layout, ported from `Blockly.BlockSvg.prototype.renderCompute_` and the
//! cursor arithmetic in `renderDrawRight_`.
//!
//! Blockly measures first and draws second, and this module keeps that split:
//! it produces a `BlockLayout` tree that `render.rs` only has to walk. Sharing
//! one traversal for both would let the two drift, which is precisely the
//! class of error the overlay comparison is meant to catch.

use crate::model::{BlockSpec, SegmentSpec};
use crate::svg::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static WIDTHS: RefCell<Option<HashMap<String, f32>>> = const { RefCell::new(None) };
}

/// Install text widths measured by Typst in the real font.
pub fn set_widths(widths: HashMap<String, f32>) {
    WIDTHS.with(|cell| *cell.borrow_mut() = Some(widths));
}

pub fn clear_widths() {
    WIDTHS.with(|cell| *cell.borrow_mut() = None);
}

/// Key under which a string's measured width is stored.
///
/// The image block's rulers are `class="blocklyText monospace"`, so the same
/// characters have two different widths in one document. The key carries the
/// font with it so the host can measure each one correctly and the lookup
/// cannot silently pick the wrong entry.
pub fn width_key(text: &str, monospace: bool) -> String {
    if monospace {
        format!("mono\u{1}{text}")
    } else {
        format!("prop\u{1}{text}")
    }
}

/// The string a field actually paints.
///
/// `Blockly.FieldDropdown.prototype.render_` appends `' ▾'` to the option
/// text, so the arrow is part of the measured width rather than a separate
/// glyph placed afterwards.
pub fn display_text(segment: &SegmentSpec) -> String {
    match segment {
        SegmentSpec::Text { value, .. } => nbsp(value),
        SegmentSpec::Input { value } => nbsp(value),
        SegmentSpec::Dropdown { value } => format!("{} \u{25BE}", nbsp(value)),
        SegmentSpec::PixelCell { value } => value.clone(),
        SegmentSpec::ColourField { .. } => String::new(),
        SegmentSpec::Icon { .. } => String::new(),
        SegmentSpec::Block { .. } => String::new(),
        SegmentSpec::InlineValue { .. } => String::new(),
        SegmentSpec::PixelMatrix { .. } => String::new(),
        SegmentSpec::Image { .. } => String::new(),
    }
}

/// `Blockly.Field.prototype.updateTextNode_` swaps every space for a
/// non-breaking one before it hits the DOM.
///
/// Without it the image block's column ruler falls apart: its labels are
/// `"  1"`, `"  2"` and so on, and SVG collapses leading whitespace, so both
/// the measured width and the painted position would lose the indent.
fn nbsp(text: &str) -> String {
    text.chars()
        .map(|ch| if ch.is_whitespace() { '\u{00A0}' } else { ch })
        .collect()
}

fn is_monospace(segment: &SegmentSpec) -> bool {
    matches!(
        segment,
        SegmentSpec::Text {
            monospace: true,
            ..
        }
    )
}

/// Advance width of a string at `.blocklyText`'s 11pt.
///
/// Typst measures the real font and hands the results over; the table below is
/// only the fallback for tests and for a host that cannot measure. It is
/// Helvetica's advance widths scaled to 11pt, which is what a browser's
/// `sans-serif` resolves to on the machines Open Roberta is usually read on.
pub fn text_width(text: &str, monospace: bool) -> f32 {
    let key = width_key(text, monospace);
    if let Some(width) = WIDTHS.with(|cell| {
        cell.borrow()
            .as_ref()
            .and_then(|map| map.get(&key).copied())
    }) {
        return width;
    }

    let em = FONT_SIZE_PX;
    if monospace {
        // Courier New advances every glyph by 0.6em.
        return text.chars().count() as f32 * 0.6 * em;
    }

    text.chars()
        .map(|ch| {
            let ratio = match ch {
                // Fields reach SVG with ordinary spaces converted to NBSPs,
                // but both glyphs have the normal space advance in the
                // fallback font metrics.
                ' ' | '\u{00a0}' => 0.278,
                'i' | 'l' | 'j' | '.' | ',' | ':' | ';' | '|' | '!' | '\'' => 0.222,
                'f' | 't' | 'r' | '(' | ')' | '[' | ']' | '/' | '\\' => 0.278,
                'm' | 'w' => 0.833,
                'M' | 'W' => 0.833,
                '\u{25BE}' => 0.6,
                c if c.is_ascii_digit() => 0.556,
                c if c.is_ascii_uppercase() => 0.722,
                c if c.is_ascii_lowercase() => 0.556,
                _ => 0.556,
            };
            ratio * em
        })
        .sum()
}

// The actual notch is 14.5px wide, but its left-side tab reaches six pixels
// into the field's allocation. Blockly therefore reserves 20.5px before the
// next field (see an empty `logic_operation`).
const EMPTY_INLINE_SOCKET_WIDTH: f32 = 20.5;
// `renderCompute_` gives a connected inline reporter six pixels above and
// five below its child. This is why a comparison around a 25px sensor is 36px
// high, and a logical operation around that comparison is 47px high.
const INLINE_CHILD_TOP: f32 = 6.0;
const INLINE_CHILD_BOTTOM: f32 = 5.0;

/// `field.getSize().height`. Blockly's default is 25 and every field here
/// keeps it — except the picture dropdown, which asks for 34 and makes its
/// row that tall.
fn field_height(segment: &SegmentSpec) -> f32 {
    match segment {
        SegmentSpec::Image { .. } => IMAGE_FIELD_HEIGHT,
        _ => FIELD_HEIGHT,
    }
}

fn field_width(segment: &SegmentSpec) -> f32 {
    match segment {
        // `Blockly.FieldPixelbox` pins its own width regardless of content.
        SegmentSpec::PixelCell { .. } => PIXEL_BOX_SIZE,
        // The colour picker has no text, but its painted swatch still needs
        // a fixed 22px target. `render_field` adds FIELD_BOX_PAD itself.
        SegmentSpec::ColourField { .. } => COLOUR_FIELD_WIDTH - FIELD_BOX_PAD,
        SegmentSpec::Image { .. } => IMAGE_FIELD_WIDTH,
        SegmentSpec::InlineValue { .. } => EMPTY_INLINE_SOCKET_WIDTH,
        // `Blockly.Icon.prototype.SIZE`. An icon is not a field in Blockly —
        // `render()` walks it before `renderCompute_` and shifts the first
        // row by `SIZE + SEP_SPACE_X`. Modelling it as a leading,uneditable
        // field of width SIZE produces the same arithmetic, because the row
        // then adds exactly one SEP_SPACE_X after it. The drawn symbol is
        // 16 wide; the space it claims is 17.
        SegmentSpec::Icon { name } if name == "plus" || name == "minus" => 17.0,
        SegmentSpec::Icon { name } if matches!(name.as_str(), "quote_open" | "quote_close") => 12.0,
        other => text_width(&display_text(other), is_monospace(other)),
    }
}

#[derive(Debug, Clone)]
pub struct FieldLayout {
    /// x of the text origin, in block coordinates.
    pub x: f32,
    pub y: f32,
    pub width: f32,
    /// Layout of a reporting block connected to an inline value socket.
    pub inline_value: Option<Box<BlockLayout>>,
}

#[derive(Debug, Clone)]
pub struct RowLayout {
    pub kind: String,
    /// Top of the row in block coordinates, after any `SEP_SPACE_Y` padding a
    /// leading statement row inserts.
    pub y: f32,
    pub height: f32,
    pub fields: Vec<FieldLayout>,
    /// Padding Blockly adds above a statement row that opens the block.
    pub lead_gap: f32,
    /// Padding below a trailing statement row.
    pub trail_gap: f32,
    pub value: Option<Box<BlockLayout>>,
    pub value_origin: (f32, f32),
    pub body: Option<Box<StackLayout>>,
    pub body_origin: (f32, f32),
    pub check: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BlockLayout {
    pub right_edge: f32,
    pub statement_edge: f32,
    /// `cursorY` at the end of `renderDrawRight_`: the block's bottom edge.
    pub bottom: f32,
    /// Widest extent including blocks plugged into the right edge.
    pub width: f32,
    pub rows: Vec<RowLayout>,
    pub has_previous: bool,
    pub has_next: bool,
    pub has_output: bool,
    pub output_check: Vec<String>,
    pub square_top_left: bool,
    pub square_bottom_left: bool,
    pub category: String,
    pub id: String,
}

impl BlockLayout {
    /// `Blockly.Block.prototype.height`: the outline plus the shadow pixel,
    /// plus the notch that hangs below a block with a next connection.
    pub fn own_height(&self) -> f32 {
        self.bottom + 1.0 + if self.has_next { NOTCH_DEPTH } else { 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct StackLayout {
    /// Each block with its y offset inside the stack.
    pub blocks: Vec<(f32, BlockLayout)>,
    pub width: f32,
    pub height: f32,
}

fn connections(shape: &str) -> (bool, bool, bool) {
    match shape {
        // The Start block cannot be preceded by anything.
        "start" => (false, true, false),
        "cap" => (true, false, false),
        "value" => (false, false, true),
        _ => (true, true, false),
    }
}

pub fn layout_stack(blocks: &[BlockSpec]) -> StackLayout {
    let mut laid: Vec<(f32, BlockLayout)> = Vec::new();
    let mut y = 0.0f32;
    let mut width = 0.0f32;

    for (index, block) in blocks.iter().enumerate() {
        let layout = layout_block(block, index > 0, index + 1 < blocks.len());
        width = width.max(layout.width);
        let height_step = layout.bottom + 1.0;
        laid.push((y, layout));
        y += height_step;
    }

    // `getHeightWidth` keeps the trailing notch of the last block in the
    // stack's height.
    let height = match laid.last() {
        Some((_, last)) if last.has_next => y + NOTCH_DEPTH,
        Some(_) => y,
        None => 0.0,
    };

    StackLayout {
        blocks: laid,
        width,
        height,
    }
}

pub fn layout_block(
    block: &BlockSpec,
    connected_above: bool,
    connected_below: bool,
) -> BlockLayout {
    let (has_previous, has_next, has_output) = connections(&block.shape);

    let mut right_edge = SEP_SPACE_X * 2.0;
    if has_previous || has_next {
        right_edge = right_edge.max(NOTCH_WIDTH + SEP_SPACE_X);
    }

    let mut field_value_width = 0.0f32;
    let mut field_statement_width = 0.0f32;
    let mut has_value = false;
    let mut has_statement = false;
    let mut has_dummy = false;

    struct Measured {
        kind: String,
        align: String,
        height: f32,
        field_width: f32,
        widths: Vec<f32>,
        inline_values: Vec<Option<Box<BlockLayout>>>,
        seps: Vec<f32>,
        value: Option<Box<BlockLayout>>,
        body: Option<Box<StackLayout>>,
        check: Vec<String>,
    }

    let mut measured: Vec<Measured> = Vec::new();
    let row_count = block.rows.len();

    for (index, row) in block.rows.iter().enumerate() {
        let value = row
            .value
            .as_ref()
            .map(|child| Box::new(layout_block(child, false, false)));
        let body = if row.is_statement() && !row.body.is_empty() {
            Some(Box::new(layout_stack(&row.body)))
        } else {
            None
        };

        let mut render_height = MIN_BLOCK_Y;
        if let Some(child) = value.as_ref() {
            render_height = render_height.max(child.own_height());
        }
        if let Some(stack) = body.as_ref() {
            render_height = render_height.max(stack.height);
        }
        let has_inline = row
            .fields
            .iter()
            .any(|field| matches!(field, SegmentSpec::InlineValue { .. }));
        if has_inline {
            // An empty inline socket is 25px tall with five pixels of air
            // above and below. The final shadow-pixel adjustment below turns
            // this 36px intermediate value into Blockly's 35px outline.
            render_height =
                render_height.max(FIELD_HEIGHT + INLINE_CHILD_TOP + INLINE_CHILD_BOTTOM + 1.0);
        }
        for field in &row.fields {
            if let SegmentSpec::InlineValue {
                value: Some(child), ..
            } = field
            {
                let child = layout_block(child, false, false);
                render_height =
                    render_height.max(child.own_height() + INLINE_CHILD_TOP + INLINE_CHILD_BOTTOM);
            }
        }

        // "Blocks have a one pixel shadow that should sometimes overhang."
        let next_is_statement = block
            .rows
            .get(index + 1)
            .map(|next| next.is_statement())
            .unwrap_or(false);
        if index + 1 == row_count {
            render_height -= 1.0;
        } else if row.is_value() && next_is_statement {
            render_height -= 1.0;
        }

        let mut height = render_height;
        let mut field_width = 0.0f32;
        let mut widths = Vec::with_capacity(row.fields.len());
        let mut inline_values = Vec::with_capacity(row.fields.len());
        let mut seps = Vec::with_capacity(row.fields.len());
        let mut previous_editable = false;

        for (position, field) in row.fields.iter().enumerate() {
            if position != 0 {
                field_width += SEP_SPACE_X;
            }
            let inline = match field {
                SegmentSpec::InlineValue {
                    value: Some(child), ..
                } => Some(Box::new(layout_block(child, false, false))),
                _ => None,
            };
            let width = inline
                .as_ref()
                .map(|child| child.width)
                .unwrap_or_else(|| field_width_of(field));
            let sep = if previous_editable && field.is_editable() {
                SEP_SPACE_X
            } else {
                0.0
            };
            field_width += width + sep;
            height = height.max(field_height(field));
            if let Some(child) = inline.as_ref() {
                height = height.max(child.own_height());
            }
            widths.push(width);
            inline_values.push(inline);
            seps.push(sep);
            previous_editable = field.is_editable();
        }

        if row.is_statement() {
            has_statement = true;
            field_statement_width = field_statement_width.max(field_width);
        } else if row.is_value() {
            has_value = true;
            field_value_width = field_value_width.max(field_width);
        } else {
            has_dummy = true;
            field_value_width = field_value_width.max(field_width);
        }

        measured.push(Measured {
            kind: row.kind.clone(),
            align: row.align.clone(),
            height,
            field_width,
            widths,
            inline_values,
            seps,
            value,
            body,
            check: row.check.clone(),
        });
    }

    let statement_edge = 2.0 * SEP_SPACE_X + field_statement_width;
    if has_statement {
        right_edge = right_edge.max(statement_edge + NOTCH_WIDTH);
    }
    if has_value {
        right_edge = right_edge.max(field_value_width + SEP_SPACE_X * 2.0 + TAB_WIDTH);
    } else if has_dummy {
        right_edge = right_edge.max(field_value_width + SEP_SPACE_X * 2.0);
    }

    let mut width = right_edge;
    let mut rows: Vec<RowLayout> = Vec::with_capacity(measured.len());
    let mut cursor_y = 0.0f32;

    for (index, row) in measured.iter().enumerate() {
        let mut lead_gap = 0.0;
        let mut trail_gap = 0.0;
        let is_statement = row.kind == "statement";

        if is_statement && index == 0 {
            lead_gap = SEP_SPACE_Y;
            cursor_y += SEP_SPACE_Y;
        }

        let row_top = cursor_y;

        // Alignment, per `renderDrawRight_`.
        let mut field_x = SEP_SPACE_X;
        if row.align != "left" {
            let available = if is_statement {
                statement_edge - row.field_width - 2.0 * SEP_SPACE_X
            } else if row.kind == "value" {
                right_edge - row.field_width - TAB_WIDTH - 2.0 * SEP_SPACE_X
            } else {
                let mut space = right_edge - row.field_width - 2.0 * SEP_SPACE_X;
                if has_value {
                    space -= TAB_WIDTH;
                }
                space
            };
            field_x += match row.align.as_str() {
                "right" => available,
                "centre" | "center" => available / 2.0,
                _ => 0.0,
            };
        }

        let mut fields = Vec::with_capacity(row.widths.len());
        let mut cursor_x = field_x;
        let has_inline = block.rows[index]
            .fields
            .iter()
            .any(|field| matches!(field, SegmentSpec::InlineValue { .. }));
        let field_y = row_top
            + if has_inline {
                // Dropdowns in an inline expression sit at y=10 while their
                // nested reporters begin at y=6.
                INLINE_PADDING_Y + INLINE_CHILD_BOTTOM
            } else {
                INLINE_PADDING_Y
            };
        for (position, field_width) in row.widths.iter().enumerate() {
            let sep = row.seps[position];
            fields.push(FieldLayout {
                x: cursor_x + sep,
                y: field_y,
                width: *field_width,
                inline_value: row.inline_values[position].clone(),
            });
            if *field_width > 0.0 {
                cursor_x += sep + field_width + SEP_SPACE_X;
            }
        }

        let mut value_origin = (0.0, 0.0);
        if let Some(child) = row.value.as_ref() {
            value_origin = (right_edge + 1.0, row_top);
            width = width.max(right_edge + child.width - TAB_WIDTH + 1.0);
        }

        let mut body_origin = (0.0, 0.0);
        if let Some(stack) = row.body.as_ref() {
            body_origin = (statement_edge + 1.0, row_top + 1.0);
            width = width.max(statement_edge + stack.width);
        }

        cursor_y += row.height;

        let next_is_statement = measured
            .get(index + 1)
            .map(|next| next.kind == "statement")
            .unwrap_or(false);
        if is_statement && (index + 1 == measured.len() || next_is_statement) {
            trail_gap = SEP_SPACE_Y;
            cursor_y += SEP_SPACE_Y;
        }

        rows.push(RowLayout {
            kind: row.kind.clone(),
            y: row_top,
            height: row.height,
            fields,
            lead_gap,
            trail_gap,
            value: row.value.clone(),
            value_origin,
            body: row.body.clone(),
            body_origin,
            check: row.check.clone(),
        });
    }

    if rows.is_empty() {
        cursor_y = MIN_BLOCK_Y;
    }

    // `renderDrawLeft_`: a typed output plug is drawn as its own path hanging
    // into negative x, and Blockly widens the block to account for it. Parent
    // blocks then subtract it again when they place the child, so leaving it
    // out clips the plug off the right of the page.
    if has_output && !block.check.is_empty() {
        width += TAB_WIDTH;
    }

    BlockLayout {
        right_edge,
        statement_edge,
        bottom: cursor_y,
        width,
        rows,
        has_previous,
        has_next,
        has_output,
        output_check: block.check.clone(),
        square_top_left: has_output || (has_previous && connected_above),
        square_bottom_left: has_output || (has_next && connected_below),
        category: block.category.clone(),
        id: block.id.clone(),
    }
}

fn field_width_of(segment: &SegmentSpec) -> f32 {
    field_width(segment)
}

/// Every string the renderer will paint, so the host can measure them in the
/// real font before layout runs.
pub fn collect_texts(blocks: &[BlockSpec], out: &mut std::collections::BTreeSet<String>) {
    for block in blocks {
        for row in &block.rows {
            for field in &row.fields {
                let text = display_text(field);
                if !text.is_empty() {
                    out.insert(width_key(&text, is_monospace(field)));
                }
                if let SegmentSpec::Block { block } = field {
                    collect_texts(std::slice::from_ref(block.as_ref()), out);
                }
                if let SegmentSpec::InlineValue {
                    value: Some(block), ..
                } = field
                {
                    collect_texts(std::slice::from_ref(block.as_ref()), out);
                }
            }
            if let Some(value) = row.value.as_ref() {
                collect_texts(std::slice::from_ref(value.as_ref()), out);
            }
            collect_texts(&row.body, out);
        }
    }
}
