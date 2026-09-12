//! Layout and drawing for zelos-style Blockly — the renderer MakeCode's
//! editors use (pxt extends Blockly's zelos with fonts and colours only).
//!
//! zelos works on a grid of 4: 4px corners, a 36px notch that starts at
//! x = 12, rows at least 32 high between a top row and a bottom row of 4,
//! statement blocks at least 48 high. Values have no puzzle tab: a reporter
//! is a pill, a boolean a hexagon, and the connection shape is as tall as the
//! block it belongs to. Literals are white shadow pills drawn on top of the
//! parent; empty sockets are holes in the block's darker tertiary colour.
//!
//! The rules are those of Blockly 13.1.1's zelos RenderInfo — spacing
//! between elements, spacer rows around mouths, the 48px bump past the
//! notch, negative edge spacing on reporters and the tight nesting of tall
//! booleans — checked against the live editors on makecode.microbit.org and
//! makecode.calliope.cc (`tests/fixtures/makecode-reference.json`).

use crate::blockly::{field_advance, plan, unwrapped, Item, Plan};
use crate::measure::text_width;
use crate::model::{BlockSpec, SegmentSpec};
use crate::palette::{colors_for, CategoryColors};
use crate::svg::escape_text;

const GRID: f32 = 4.0;
const CORNER: f32 = 4.0;
const NOTCH_W: f32 = 36.0;
const NOTCH_H: f32 = 8.0;
const NOTCH_OFFSET_LEFT: f32 = 12.0;
/// NOTCH_OFFSET_LEFT + NOTCH_WIDTH: nothing but a label may start left of it
/// in a row that has a notch above and below.
const NOTCH_TOTAL: f32 = 48.0;
const MIN_BLOCK_W: f32 = 8.0;
const EMPTY_STATEMENT_H: f32 = 24.0;
const TOP_ROW_MIN: f32 = 4.0;
const BOTTOM_ROW_MIN: f32 = 4.0;
const BOTTOM_AFTER_STATEMENT: f32 = 24.0;
const STATEMENT_PAD_LEFT: f32 = 16.0;
/// Where the mouth's notch starts, measured from the mouth's inner wall.
const STATEMENT_NOTCH_OFFSET: f32 = 12.0;
const INSIDE_CORNER: f32 = 4.0;
const EMPTY_INLINE_H: f32 = 32.0;
const EMPTY_INLINE_PAD: f32 = 16.0;
const DUMMY_MIN_H: f32 = 32.0;
const DUMMY_SHADOW_MIN_H: f32 = 24.0;
const FIELD_TEXT_H: f32 = 21.0;
const FIELD_RECT_H: f32 = 34.0;
const FIELD_X_PAD: f32 = 8.0;
const ARROW_W: f32 = 12.0;
const ARROW_PAD: f32 = 8.0;
const SMALL: f32 = 4.0;
const MEDIUM: f32 = 8.0;
const MAX_DYNAMIC_W: f32 = 48.0;
const ROUND_MAX_H: f32 = 96.0;
const STATEMENT_SPACER_MIN_W: f32 = 160.0;
const BUTTON: f32 = 24.0;
/// The LED matrix field: 25px cells on a 32px pitch, 7px in and 5px down.
const MATRIX_W: f32 = 167.0;
const MATRIX_H: f32 = 169.0;
const MATRIX_CELL: f32 = 25.0;
const MATRIX_PITCH: f32 = 32.0;
/// The melody editor pill and its eight 10x20 note cells.
const MELODY_W: f32 = 140.0;
const MELODY_H: f32 = 42.0;
const MELODY_FILL: &str = "#d9d9d9";
const NOTE_COLOURS: [&str; 8] = ["#a80000", "#d83b01", "#ffb900", "#107c10", "#008272", "#0078d7", "#5c2d91", "#b4009e"];
/// Room MakeCode leaves for each quote glyph beside a text literal.
const QUOTE_W: f32 = 3.0;
/// A non-shadow value at least this tall pulls the rows around it 4px closer.
const TIGHT_NESTING_MIN_H: f32 = 40.0;
/// The alphabetic baseline of a 12pt bold label sits this far below its centre.
const BASELINE_BELOW_CENTRE: f32 = 5.5;
const NOTCH_LEFT: &str = "c 2,0 3,1 4,2 l 4,4 c 1,1 2,2 4,2 h 12 c 2,0 3,-1 4,-2 l 4,-4 c 1,-1 2,-2 4,-2";
const NOTCH_RIGHT: &str = "c -2,0 -3,1 -4,2 l -4,4 c -1,1 -2,2 -4,2 h -12 c -2,0 -3,-1 -4,-2 l -4,-4 c -1,-1 -2,-2 -4,-2";
const HAT: &str = "c 25,-22 71,-22 96,0";
const HAT_H: f32 = 16.5;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Shape {
    Round,
    Hex,
}

impl Shape {
    fn width(self, h: f32) -> f32 {
        match self {
            Shape::Round => h.min(ROUND_MAX_H) / 2.0,
            Shape::Hex => (h / 2.0).min(MAX_DYNAMIC_W),
        }
    }
    fn right_down(self, h: f32) -> String {
        match self {
            Shape::Round => {
                let r = self.width(h);
                format!("a {r} {r} 0 0,1 {r},{r} v {} a {r} {r} 0 0,1 -{r},{r}", h - 2.0 * r)
            }
            Shape::Hex => {
                let w = self.width(h);
                format!("l {w},{w} l -{w},{w}")
            }
        }
    }
    fn up(self, h: f32) -> String {
        match self {
            Shape::Round => {
                let r = self.width(h);
                format!("a {r} {r} 0 0,1 -{r},-{r} v -{} a {r} {r} 0 0,1 {r},-{r}", h - 2.0 * r)
            }
            Shape::Hex => {
                let w = self.width(h);
                format!("l -{w},-{w} l {w},-{w}")
            }
        }
    }
    /// SHAPE_IN_SHAPE_PADDING: how close the outer shape hugs what sits at
    /// its edge. `inner` None is a field.
    fn hug(self, inner: Option<Shape>) -> f32 {
        match (self, inner) {
            (Shape::Hex, None) => 5.0 * GRID,
            (Shape::Hex, Some(Shape::Hex)) => 2.0 * GRID,
            (Shape::Hex, Some(Shape::Round)) => 5.0 * GRID,
            (Shape::Round, None) => 3.0 * GRID,
            (Shape::Round, Some(Shape::Hex)) => 3.0 * GRID,
            (Shape::Round, Some(Shape::Round)) => GRID,
        }
    }
}

fn shape_of(block: &BlockSpec) -> Shape {
    if block.shape == "boolean" {
        Shape::Hex
    } else {
        Shape::Round
    }
}

/// A literal in a value slot — what `shadow()` builds in `plan()`: one bare
/// input segment. MakeCode draws it as a white pill.
pub(crate) fn is_literal(block: &BlockSpec) -> bool {
    block.shape == "reporter"
        && block.segments.len() == 1
        && matches!(&block.segments[0], SegmentSpec::Input { nested: None, input, .. } if input != "dropdown" && input != "dropdown-field")
}

/// `<wahr>` / `<true>`: the boolean literal, a shadow in the editor — a
/// 32px hexagon in the logic colour with the word at x = 20 and no frame.
fn is_boolean_literal(block: &BlockSpec) -> bool {
    block.shape == "boolean"
        && block.segments.len() == 1
        && matches!(&block.segments[0], SegmentSpec::Text { value } if matches!(value.as_str(), "wahr" | "falsch" | "true" | "false"))
}

/// A block built around one of MakeCode's editor fields (the melody).
fn holds_editor(block: &BlockSpec) -> bool {
    block.segments.iter().any(|s| matches!(s, SegmentSpec::Input { input, .. } if input == "string")) && {
        let p = plan(block);
        p.rows.iter().any(|r| r.items.iter().any(|i| matches!(i, Item::Editor { kind, .. } if kind == "melody")))
    }
}

fn literal_text(block: &BlockSpec) -> (String, bool) {
    match &block.segments[0] {
        SegmentSpec::Input { input, value, .. } => (value.clone(), input == "string" || input == "text"),
        _ => (String::new(), false),
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ElemKind {
    Label,
    /// An editable field: dropdown or text box, drawn with a 34px frame.
    Field,
    /// The 5x5 LED matrix editor, 167 x 169.
    Matrix,
    /// The melody editor: an 8-note grid in a grey pill, 140 x 42.
    Melody,
    /// A 24px image button (the +/- of an if block).
    Button,
    Inline,
    Statement,
}

struct Elem {
    kind: ElemKind,
    x: f32,
    w: f32,
    h: f32,
    /// Which item of the plan's row this is, for drawing.
    item: Option<usize>,
    /// For inline inputs: the child's output shape (its own shape when empty).
    shape: Shape,
    /// For inline inputs: a non-shadow child at least 40 tall.
    tall_child: bool,
    /// A text box rather than a label (the field of a literal pill).
    text_input: bool,
}

struct ZRow {
    y: f32,
    h: f32,
    natural_w: f32,
    w: f32,
    statement: bool,
    /// Index into plan.rows or plan.mouths.
    source: Source,
    elems: Vec<Elem>,
    /// Spacer above this row (set after the rows are built).
    spacer_above: f32,
}

#[derive(Clone, Copy)]
enum Source {
    Row(usize),
    /// A mouth's own statement row.
    Mouth(usize),
    /// The label row above a second or later mouth ("ansonsten").
    MouthLabel(usize),
    /// The trailing "+" row of an if block.
    AddRow,
}

struct Layout {
    rows: Vec<ZRow>,
    /// Spacer between the last row and the bottom row.
    bottom_spacer: f32,
    /// Height of the outline, what the next block is placed after.
    body_h: f32,
    /// Bottom notch below the outline.
    descender: f32,
    w: f32,
    /// Output blocks: the connection shape's width on each side.
    conn_w: f32,
    shape: Option<Shape>,
    has_prev: bool,
    has_next: bool,
    hat: bool,
}

fn measure_child(child: &BlockSpec) -> (f32, f32) {
    size(child)
}

fn elem_for_item(item: &Item, shadow_block: bool, string_literal: bool) -> Elem {
    match item {
        Item::Label(text) => Elem { kind: ElemKind::Label, x: 0.0, w: text_width(text), h: FIELD_TEXT_H, item: None, shape: Shape::Round, tall_child: false, text_input: false },
        Item::Icon(_) => Elem { kind: ElemKind::Button, x: 0.0, w: BUTTON, h: BUTTON + 1.0, item: None, shape: Shape::Round, tall_child: false, text_input: false },
        Item::Field { text, dropdown } => {
            let mut w = field_advance(text, false);
            let h = if shadow_block { FIELD_TEXT_H } else { FIELD_RECT_H };
            if string_literal {
                // the quote glyphs on either side of a text pill
                w += 2.0 * QUOTE_W;
            }
            if !shadow_block {
                w += 2.0 * FIELD_X_PAD;
                if *dropdown {
                    w += ARROW_W + ARROW_PAD;
                }
            }
            Elem { kind: ElemKind::Field, x: 0.0, w, h, item: None, shape: Shape::Round, tall_child: false, text_input: false }
        }
        Item::Editor { kind, .. } => {
            if kind == "matrix" {
                Elem { kind: ElemKind::Matrix, x: 0.0, w: MATRIX_W, h: MATRIX_H, item: None, shape: Shape::Round, tall_child: false, text_input: false }
            } else {
                Elem { kind: ElemKind::Melody, x: 0.0, w: MELODY_W, h: MELODY_H, item: None, shape: Shape::Round, tall_child: false, text_input: false }
            }
        }
        Item::Socket(None, boolean) => {
            let shape = if *boolean { Shape::Hex } else { Shape::Round };
            Elem { kind: ElemKind::Inline, x: 0.0, w: EMPTY_INLINE_PAD + 2.0 * shape.width(EMPTY_INLINE_H), h: EMPTY_INLINE_H, item: None, shape, tall_child: false, text_input: false }
        }
        Item::Socket(Some(child), _) => {
            let (w, h) = measure_child(child);
            // shadows never trigger tight nesting: literals, and the editor
            // blocks (the melody) that only ever sit in their parent's slot
            let shadow = is_literal(child) || is_boolean_literal(child) || holds_editor(child);
            let tall = !shadow && h >= TIGHT_NESTING_MIN_H;
            Elem { kind: ElemKind::Inline, x: 0.0, w, h, item: None, shape: shape_of(child), tall_child: tall, text_input: false }
        }
    }
}

fn layout(block: &BlockSpec, p: &Plan) -> Layout {
    let is_output = p.is_value;
    let shadow_block = is_output && (is_literal(block) || is_boolean_literal(block));
    let string_literal = shadow_block && literal_text(block).1;
    let out_shape = if is_output { Some(shape_of(block)) } else { None };
    let has_prev = p.top_notch;
    let has_next = p.bottom_notch;
    let hat = block.shape == "define-hat";
    let has_statement = !p.mouths.is_empty();
    // The +/- buttons of an if block travel as the "mutator" icon.
    let if_buttons = has_statement && block.icon.as_deref().map(|i| i.contains("mutator")).unwrap_or(false);

    // --- rows ------------------------------------------------------------
    let mut rows: Vec<ZRow> = Vec::new();
    let dynamic_output = is_output && !has_statement && !has_next;
    let row_min = if shadow_block { DUMMY_SHADOW_MIN_H } else { DUMMY_MIN_H };
    let start_pad = if dynamic_output { 0.0 } else { MEDIUM };

    let push_input_row = |items: &[Item], source: Source, trailing_button: bool, rows: &mut Vec<ZRow>| {
        let mut elems: Vec<Elem> = Vec::new();
        let mut x = start_pad;
        for (index, item) in items.iter().enumerate() {
            // Blockly's icons have no place here; the if block's +/- buttons
            // come from `if_buttons` below.
            if matches!(item, Item::Icon(_)) {
                continue;
            }
            let mut e = elem_for_item(item, shadow_block, string_literal);
            e.item = Some(index);
            if !elems.is_empty() {
                x += MEDIUM;
            }
            e.x = x;
            x += e.w;
            elems.push(e);
        }
        if trailing_button {
            if !elems.is_empty() {
                x += MEDIUM;
            }
            elems.push(Elem { kind: ElemKind::Button, x, w: BUTTON, h: BUTTON + 1.0, item: None, shape: Shape::Round, tall_child: false, text_input: false });
            x += BUTTON;
        }
        let end_pad = if dynamic_output { 0.0 } else { MEDIUM };
        let natural_w = x + end_pad;
        let mut h = elems.iter().map(|e| e.h).fold(0.0, f32::max).max(row_min);
        let single_image = elems.len() == 1 && elems[0].kind == ElemKind::Button;
        if single_image {
            h = h.max(DUMMY_MIN_H);
        }
        rows.push(ZRow { y: 0.0, h, natural_w, w: natural_w, statement: false, source, elems, spacer_above: 0.0 });
    };

    for (index, row) in p.rows.iter().enumerate() {
        // The "-" button sits on the else row, the "+" on a row of its own.
        let button = false;
        let _ = (if_buttons, index);
        // zelos has no puzzle tab: a value that ends a row of a block that is
        // not inputsInline is still drawn as an inline socket, at the row's end.
        let mut items: Vec<Item> = row.items.clone();
        if let Some(external) = &row.external {
            items.push(Item::Socket(external.clone(), row.external_boolean));
        }
        push_input_row(&items, Source::Row(index), button, &mut rows);
    }
    for (index, mouth) in p.mouths.iter().enumerate() {
        let label = mouth.label.as_deref().filter(|l| !l.is_empty());
        if index > 0 {
            if let Some(text) = label {
                push_input_row(&[Item::Label(text.to_string())], Source::MouthLabel(index), if_buttons, &mut rows);
            }
        }
        // The statement row: a label beside the mouth for the first one.
        let mut elems: Vec<Elem> = Vec::new();
        let mut x;
        if index == 0 {
            if let Some(text) = label {
                x = MEDIUM;
                elems.push(Elem { kind: ElemKind::Label, x, w: text_width(text), h: FIELD_TEXT_H, item: None, shape: Shape::Round, tall_child: false, text_input: false });
                x += text_width(text) + MEDIUM;
            } else {
                x = STATEMENT_PAD_LEFT;
            }
        } else {
            x = STATEMENT_PAD_LEFT;
        }
        let stack_h = if mouth.body.is_empty() { EMPTY_STATEMENT_H } else { stack_full_height(&mouth.body) - NOTCH_H };
        let h = stack_h.max(EMPTY_STATEMENT_H);
        elems.push(Elem { kind: ElemKind::Statement, x, w: STATEMENT_NOTCH_OFFSET + NOTCH_W, h, item: None, shape: Shape::Round, tall_child: false, text_input: false });
        let natural_w = x + STATEMENT_NOTCH_OFFSET + NOTCH_W + INSIDE_CORNER;
        rows.push(ZRow { y: 0.0, h, natural_w, w: natural_w, statement: true, source: Source::Mouth(index), elems, spacer_above: 0.0 });
    }
    if if_buttons {
        push_input_row(&[], Source::AddRow, true, &mut rows);
    }

    // --- top and bottom rows, spacers -----------------------------------------
    let top_h = if hat { HAT_H + TOP_ROW_MIN } else if has_prev { NOTCH_H } else { TOP_ROW_MIN };
    let top_min = if has_prev { NOTCH_H } else { TOP_ROW_MIN };
    let n = rows.len();
    let last_is_statement = rows.last().map(|r| r.statement).unwrap_or(false);
    let bottom_h = if last_is_statement { BOTTOM_AFTER_STATEMENT } else { BOTTOM_ROW_MIN };
    let descender = if has_next { NOTCH_H } else { 0.0 };

    // spacer after the top row
    let mut top_spacer = if n == 0 {
        16.0
    } else if !has_prev && (!is_output || has_statement) {
        (NOTCH_H - CORNER).abs()
    } else {
        0.0
    };
    let mut spacers: Vec<f32> = Vec::with_capacity(n.saturating_sub(1));
    for i in 1..n {
        let follows = rows[i - 1].statement;
        let precedes = rows[i].statement;
        spacers.push(if precedes && follows {
            NOTCH_H.max(INSIDE_CORNER).max(DUMMY_MIN_H)
        } else if precedes || follows {
            NOTCH_H.max(INSIDE_CORNER)
        } else {
            MEDIUM
        });
    }
    let mut bottom_spacer = if last_is_statement {
        NOTCH_H.max(INSIDE_CORNER)
    } else if !is_output {
        top_min.max(NOTCH_H.max(CORNER)) - CORNER
    } else if !has_next && has_statement {
        (NOTCH_H - CORNER).abs()
    } else {
        0.0
    };

    // finalizeVerticalAlignment_: tight nesting and single-image rows, only on
    // rows with a notch above and below.
    if !is_output {
        for i in 0..n {
            if rows[i].statement {
                continue;
            }
            let first = i == 0;
            let has_prev_notch = if first { has_prev } else { rows[i - 1].statement };
            let has_next_notch = if i + 1 == n { has_next } else { rows[i + 1].statement };
            if !has_prev_notch {
                continue;
            }
            let single_image = rows[i].elems.len() == 1 && rows[i].elems[0].kind == ElemKind::Button;
            let (prev_delta, next_delta, row_delta) = if !first && single_image {
                (-SMALL, -SMALL, -MEDIUM)
            } else if !first && !has_next_notch {
                (SMALL, 0.0, 0.0)
            } else if has_next_notch && rows[i].elems.iter().any(|e| e.kind == ElemKind::Inline && e.tall_child) {
                (-SMALL, -SMALL, 0.0)
            } else {
                (0.0, 0.0, 0.0)
            };
            if first {
                top_spacer += prev_delta;
            } else {
                spacers[i - 1] += prev_delta;
            }
            if i + 1 == n {
                bottom_spacer += next_delta;
            } else {
                spacers[i] += next_delta;
            }
            rows[i].h += row_delta;
        }
    }
    // --- widths ------------------------------------------------------------
    let conn_w = match out_shape {
        Some(_) => 0.0, // set after the height is known
        None => 0.0,
    };
    let mut width = rows.iter().map(|r| r.natural_w).fold(0.0, f32::max).max(MIN_BLOCK_W);
    if has_statement {
        width = width.max(STATEMENT_SPACER_MIN_W);
    }
    if !is_output {
        width = width.max(NOTCH_TOTAL + MEDIUM);
    }

    // adjustXPosition_: in a row with a notch above and below, nothing but a
    // label may start left of x = 48.
    if !is_output {
        for i in 0..n {
            if rows[i].statement {
                continue;
            }
            let first = i == 0;
            let has_prev_notch = if first { has_prev } else { rows[i - 1].statement };
            let has_next_notch = if i + 1 == n { has_next } else { rows[i + 1].statement };
            if !(has_prev_notch && (first || has_next_notch)) {
                continue;
            }
            let mut shift = 0.0;
            for e in rows[i].elems.iter_mut() {
                e.x += shift;
                if e.x < NOTCH_TOTAL && e.kind != ElemKind::Label && e.kind != ElemKind::Button {
                    let d = NOTCH_TOTAL - e.x;
                    e.x += d;
                    shift += d;
                }
            }
            rows[i].natural_w += shift;
            rows[i].w = rows[i].natural_w;
            width = width.max(rows[i].natural_w);
        }
    }

    // vertical positions
    let mut y = top_h + top_spacer;
    for i in 0..n {
        if i > 0 {
            y += spacers[i - 1];
            rows[i].spacer_above = spacers[i - 1];
        }
        rows[i].y = y;
        y += rows[i].h;
    }
    y += bottom_spacer;
    let content_h = y;
    let body_h = content_h + bottom_h;

    // --- output connection and negative edge spacing ------------------------
    let mut conn = conn_w;
    if let Some(shape) = out_shape {
        let block_h = body_h;
        conn = shape.width(block_h);
        // rows start after the left connection shape
        for r in rows.iter_mut() {
            for e in r.elems.iter_mut() {
                e.x += conn;
            }
        }
        width += conn;
        let right_conn = if dynamic_output { conn } else { 0.0 };
        width += right_conn;
        if dynamic_output {
            let multi_row = rows.len() > 1;
            for r in rows.iter_mut() {
                let first = r.elems.first();
                let last = r.elems.last();
                let mut left = negative_spacing(shape, conn, first, multi_row, block_h);
                let mut right = negative_spacing(shape, conn, last, multi_row, block_h);
                let min_w = MIN_BLOCK_W + 2.0 * conn;
                let mut total = left + right;
                if width - total < min_w {
                    total = width - min_w;
                    left = total / 2.0;
                    right = total / 2.0;
                }
                for e in r.elems.iter_mut() {
                    e.x -= left;
                }
                let _ = right;
                r.w = width - total;
            }
            let total = rows.first().map(|r| width - r.w).unwrap_or(0.0);
            width -= total;
        }
    }
    for r in rows.iter_mut() {
        if r.statement {
            r.w = width - INSIDE_CORNER;
        } else if !is_output {
            r.w = width;
        }
        // the "-" of an else row is a right-aligned dummy input: its spacer
        // takes up the slack
        if matches!(r.source, Source::MouthLabel(_)) {
            if let Some(last) = r.elems.last_mut() {
                if last.kind == ElemKind::Button {
                    last.x = width - MEDIUM - BUTTON;
                }
            }
        }
    }

    Layout { rows, bottom_spacer, body_h, descender, w: width, conn_w: conn, shape: out_shape, has_prev, has_next, hat }
}

/// getNegativeSpacing_: how far the first or last element may move into the
/// output connection's shape.
fn negative_spacing(outer: Shape, conn_w: f32, elem: Option<&Elem>, multi_row: bool, block_h: f32) -> f32 {
    let Some(elem) = elem else { return 0.0 };
    if multi_row {
        return match outer {
            Shape::Round => {
                let width = (block_h / 2.0).min(MAX_DYNAMIC_W);
                let round_padding = width * (1.0 - ((width - SMALL) / width).acos().sin());
                conn_w - round_padding
            }
            Shape::Hex => 0.0,
        };
    }
    match elem.kind {
        ElemKind::Inline => {
            let inner = elem.shape;
            if outer == Shape::Hex && inner != Shape::Hex {
                return 0.0;
            }
            conn_w - outer.hug(Some(inner))
        }
        ElemKind::Field if outer == Shape::Round && elem.text_input => conn_w - 2.75 * GRID,
        ElemKind::Field | ElemKind::Label | ElemKind::Matrix | ElemKind::Melody => conn_w - outer.hug(None),
        ElemKind::Button => SMALL,
        ElemKind::Statement => 0.0,
    }
}

// ---------------------------------------------------------------------------
// Public API, mirroring blockly.rs
// ---------------------------------------------------------------------------

/// Outline width and height (without the notch under it). For a value block
/// the width includes both connection shapes.
pub fn size(block: &BlockSpec) -> (f32, f32) {
    if let Some(inner) = unwrapped(block) {
        return size(&inner);
    }
    let p = plan(block);
    let l = layout(block, &p);
    (l.w, l.body_h)
}

fn stack_full_height(blocks: &[BlockSpec]) -> f32 {
    let mut h = 0.0;
    for (index, b) in blocks.iter().enumerate() {
        let p = plan(b);
        let l = layout(b, &p);
        h += l.body_h;
        if index + 1 == blocks.len() {
            h += l.descender;
        }
    }
    h
}

pub fn stack_size(blocks: &[BlockSpec]) -> (f32, f32) {
    let mut width: f32 = 0.0;
    for b in blocks {
        width = width.max(extent(b).0);
    }
    (width, stack_full_height(blocks))
}

pub fn extent(block: &BlockSpec) -> (f32, f32) {
    if let Some(inner) = unwrapped(block) {
        return extent(&inner);
    }
    let p = plan(block);
    let l = layout(block, &p);
    let mut width = l.w;
    for (index, mouth) in p.mouths.iter().enumerate() {
        let edge = l.rows.iter().find(|r| matches!(r.source, Source::Mouth(i) if i == index)).and_then(|r| r.elems.last()).map(|e| e.x).unwrap_or(STATEMENT_PAD_LEFT);
        for child in &mouth.body {
            width = width.max(edge + extent(child).0);
        }
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
    (svg, width, y + NOTCH_H)
}

fn tertiary(fill: &str) -> String {
    blend("#000000", fill, 0.25)
}

fn parse(hex: &str) -> Option<(f32, f32, f32)> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| v as f32);
    Some((c(0)?, c(2)?, c(4)?))
}

/// Blockly.utils.colour.blend(colour1, colour2, weight): weight of colour1.
fn blend(c1: &str, c2: &str, weight: f32) -> String {
    let (Some(a), Some(b)) = (parse(c1), parse(c2)) else { return c2.to_string() };
    let mix = |x: f32, y: f32| (x * weight + y * (1.0 - weight)).round().clamp(0.0, 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", mix(a.0, b.0), mix(a.1, b.1), mix(a.2, b.2))
}

/// pxt's calculateLuminance: the weighted sum without gamma correction.
fn luminance(hex: &str) -> f32 {
    let Some((r, g, b)) = parse(hex) else { return 0.0 };
    (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0
}

/// Stroke of a white shadow pill: its own grey, or a lighter tint of the
/// parent's border when that border is very dark (pxt's applyColour).
fn shadow_stroke(parent_tertiary: Option<&str>) -> String {
    if let Some(p) = parent_tertiary {
        if luminance(p) < 0.15 {
            return blend("#ffffff", p, 0.3);
        }
    }
    "#bfbfbf".to_string()
}

pub fn render_block(block: &BlockSpec, theme: &str, first: bool, last: bool) -> (String, f32, f32) {
    render_block_in(block, theme, first, last, None)
}

fn render_block_in(block: &BlockSpec, theme: &str, _first: bool, _last: bool, parent_tertiary: Option<&str>) -> (String, f32, f32) {
    if let Some(inner) = unwrapped(block) {
        return render_block_in(&inner, theme, _first, _last, parent_tertiary);
    }
    let p = plan(block);
    let l = layout(block, &p);
    let literal = p.is_value && is_literal(block);
    let base = colors_for(&block.category, theme);
    let plain_theme = theme != "grayscale" && theme != "print";
    let holds_melody = p.rows.iter().any(|r| r.items.iter().any(|i| matches!(i, Item::Editor { kind, .. } if kind == "melody")));
    let (fill, stroke, colors) = if holds_melody && plain_theme {
        // the melody block is a shadow drawn in the category's secondary colour
        let secondary = blend("#000000", &base.fill, 0.15);
        (secondary.clone(), tertiary(&base.fill), CategoryColors::new(&secondary, &base.stroke, "#ffffff", &base.alt))
    } else if literal {
        let stroke = if plain_theme { shadow_stroke(parent_tertiary) } else { base.stroke.clone() };
        ("#ffffff".to_string(), stroke, CategoryColors::new("#ffffff", &base.stroke, "#000000", "#ffffff"))
    } else if plain_theme {
        (base.fill.clone(), tertiary(&base.fill), base.clone())
    } else {
        (base.fill.clone(), base.stroke.clone(), base.clone())
    };
    let w = l.w;
    let r = CORNER;

    // --- outline -----------------------------------------------------------
    let mut d = String::new();
    let mut holes = String::new();
    match l.shape {
        Some(shape) if p.mouths.is_empty() && !l.has_next => {
            let cw = l.conn_w;
            d.push_str(&format!("m {cw},0 h {} {} V {} h -{} {} z", w - 2.0 * cw, shape.right_down(l.body_h), l.body_h, w - 2.0 * cw, shape.up(l.body_h)));
        }
        _ => {
            // top-left corner and top edge
            if l.hat {
                d.push_str(&format!("m 0,{HAT_H} {HAT} H {} a {r} {r} 0 0,1 {r},{r}", w - r));
            } else {
                d.push_str(&format!("m 0,{r} a {r} {r} 0 0,1 {r},-{r}"));
                if l.has_prev {
                    d.push_str(&format!(" h {} {} H {}", NOTCH_OFFSET_LEFT - r, NOTCH_LEFT, w - r));
                } else {
                    d.push_str(&format!(" H {}", w - r));
                }
                d.push_str(&format!(" a {r} {r} 0 0,1 {r},{r}"));
            }
            // right side, row by row
            let mut prev_statement = false;
            for (index, row) in l.rows.iter().enumerate() {
                let spacer = row.spacer_above;
                if index > 0 {
                    let sp_y = row.y - spacer;
                    if row.statement || prev_statement {
                        // spacer beside a mouth: inside corners
                        if prev_statement {
                            d.push_str(&format!(" a {r} {r} 0 0,1 {r},{r}"));
                        }
                        let remaining = spacer - if row.statement { INSIDE_CORNER } else { 0.0 };
                        let target = sp_y + remaining;
                        if remaining > 0.0 {
                            d.push_str(&format!(" V {target}"));
                        }
                        if row.statement {
                            d.push_str(&format!(" a {r} {r} 0 0,1 -{r},{r}"));
                        }
                    } else if spacer > 0.0 {
                        d.push_str(&format!(" V {}", row.y));
                    }
                }
                if row.statement {
                    let stmt = row.elems.last().unwrap();
                    let x = stmt.x + STATEMENT_NOTCH_OFFSET + NOTCH_W;
                    d.push_str(&format!(
                        " H {x} {NOTCH_RIGHT} h -{} a {r} {r} 0 0,0 -{r},{r} v {} a {r} {r} 0 0,0 {r},{r} h {} {NOTCH_LEFT} H {}",
                        STATEMENT_NOTCH_OFFSET - INSIDE_CORNER,
                        row.h - 2.0 * INSIDE_CORNER,
                        STATEMENT_NOTCH_OFFSET - INSIDE_CORNER,
                        w - INSIDE_CORNER
                    ));
                } else {
                    d.push_str(&format!(" V {}", row.y + row.h));
                }
                prev_statement = row.statement;
            }
            if prev_statement {
                d.push_str(&format!(" a {r} {r} 0 0,1 {r},{r}"));
                let last = l.rows.last().unwrap();
                let end = last.y + last.h + l.bottom_spacer;
                if l.bottom_spacer > 0.0 {
                    d.push_str(&format!(" V {end}"));
                }
            }
            // bottom row
            d.push_str(&format!(" V {} a {r} {r} 0 0,1 -{r},{r}", l.body_h - r));
            if l.has_next {
                d.push_str(&format!(" H {} {NOTCH_RIGHT} h -{} a {r} {r} 0 0,1 -{r},-{r} z", NOTCH_TOTAL, NOTCH_OFFSET_LEFT - r));
            } else {
                d.push_str(&format!(" H {r} a {r} {r} 0 0,1 -{r},-{r} z"));
            }
        }
    }

    // --- content -----------------------------------------------------------
    let mut content = String::new();
    let stroke_colour = stroke.clone();
    for row in &l.rows {
        let items: Vec<Item> = match row.source {
            Source::Row(i) => {
                let mut items = p.rows[i].items.clone();
                if let Some(external) = &p.rows[i].external {
                    items.push(Item::Socket(external.clone(), p.rows[i].external_boolean));
                }
                items
            }
            _ => Vec::new(),
        };
        for e in &row.elems {
            let centre = if row.statement && e.kind != ElemKind::Statement { row.y + EMPTY_STATEMENT_H / 2.0 } else { row.y + row.h / 2.0 };
            match e.kind {
                ElemKind::Label => {
                    let text = match (e.item, row.source) {
                        (Some(i), Source::Row(_)) => match &items[i] { Item::Label(t) => t.clone(), _ => String::new() },
                        (_, Source::Mouth(i)) | (_, Source::MouthLabel(i)) => p.mouths[i].label.clone().unwrap_or_default(),
                        _ => String::new(),
                    };
                    content.push_str(&label_at(&colors, &text, e.x, centre + BASELINE_BELOW_CENTRE, theme));
                }
                ElemKind::Field => {
                    let (text, dropdown) = match e.item.map(|i| &items[i]) {
                        Some(Item::Field { text, dropdown }) => (text.clone(), *dropdown),
                        _ => (String::new(), false),
                    };
                    if literal {
                        let (_, is_string) = literal_text(block);
                        let tx = if is_string { e.x + QUOTE_W } else { e.x };
                        if is_string {
                            content.push_str(&format!("<text class=\"sb-input-text\" x=\"{}\" y=\"{}\" style=\"fill:#000000;font-size:10px\">\"</text>", e.x - 1.0, centre + 1.0));
                            content.push_str(&format!("<text class=\"sb-input-text\" x=\"{}\" y=\"{}\" style=\"fill:#000000;font-size:10px\">\"</text>", e.x + e.w - QUOTE_W - 1.0, centre + 1.0));
                        }
                        content.push_str(&format!("<text class=\"sb-input-text\" x=\"{tx}\" y=\"{}\" style=\"fill:#000000\">{}</text>", centre + BASELINE_BELOW_CENTRE, escape_text(&text)));
                    } else {
                        content.push_str(&format!(
                            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{FIELD_RECT_H}\" rx=\"{r}\" ry=\"{r}\" fill=\"transparent\" stroke=\"{stroke_colour}\"/>",
                            e.x,
                            centre - FIELD_RECT_H / 2.0,
                            e.w
                        ));
                        content.push_str(&label_at(&colors, &text, e.x + FIELD_X_PAD, centre + BASELINE_BELOW_CENTRE, theme));
                        if dropdown {
                            let ax = e.x + e.w - FIELD_X_PAD - ARROW_W;
                            content.push_str(&format!(
                                "<path d=\"M {},{} l 6,5 l 6,-5\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
                                ax,
                                centre - 2.5,
                                colors.text
                            ));
                        }
                    }
                }
                ElemKind::Matrix => {
                    let text = match e.item.map(|i| &items[i]) { Some(Item::Editor { text, .. }) => text.clone(), _ => String::new() };
                    let top = centre - e.h / 2.0;
                    let lit: Vec<bool> = text.chars().filter_map(|c| match c { '#' | 'X' | 'x' | '1' => Some(true), '.' | '0' | '-' => Some(false), _ => None }).collect();
                    for cell in 0..25 {
                        let (col, row) = (cell % 5, cell / 5);
                        let on = lit.get(cell).copied().unwrap_or(false);
                        let x = e.x + 7.0 + MATRIX_PITCH * col as f32;
                        let y = top + 5.0 + MATRIX_PITCH * row as f32;
                        if on {
                            content.push_str(&format!("<rect x=\"{x}\" y=\"{y}\" width=\"{MATRIX_CELL}\" height=\"{MATRIX_CELL}\" rx=\"5\" fill=\"#ffffff\"/>"));
                        } else {
                            content.push_str(&format!("<rect x=\"{x}\" y=\"{y}\" width=\"{MATRIX_CELL}\" height=\"{MATRIX_CELL}\" rx=\"5\" fill=\"#000000\" fill-opacity=\"0.2\"/>"));
                        }
                    }
                }
                ElemKind::Melody => {
                    let text = match e.item.map(|i| &items[i]) { Some(Item::Editor { text, .. }) => text.clone(), _ => String::new() };
                    let top = centre - e.h / 2.0;
                    let r = MELODY_H / 2.0;
                    let inner = MELODY_W - 2.0 * r;
                    content.push_str(&format!(
                        "<path d=\"M {},{top} h {inner} a {r} {r} 0 0,1 {r},{r} v 0 a {r} {r} 0 0,1 -{r},{r} h -{inner} a {r} {r} 0 0,1 -{r},-{r} v 0 a {r} {r} 0 0,1 {r},-{r} z\" fill=\"{MELODY_FILL}\" stroke=\"#bfbfbf\" stroke-width=\"1\"/>",
                        e.x + r
                    ));
                    content.push_str(&format!("<text x=\"{}\" y=\"{}\" font-size=\"13\" fill=\"#575757\" text-anchor=\"middle\">\u{266a}</text>", e.x + 18.0, top + 25.0));
                    let notes: Vec<&str> = text.split_whitespace().collect();
                    for i in 0..8 {
                        let note = notes.get(i).copied().unwrap_or("-");
                        let colour = match note.chars().next().map(|c| c.to_ascii_uppercase()) {
                            Some('C') if note.len() > 1 && note.ends_with(|c: char| c.is_ascii_digit()) && note.chars().last() != Some('4') => NOTE_COLOURS[7],
                            Some('C') => NOTE_COLOURS[0],
                            Some('D') => NOTE_COLOURS[1],
                            Some('E') => NOTE_COLOURS[2],
                            Some('F') => NOTE_COLOURS[3],
                            Some('G') => NOTE_COLOURS[4],
                            Some('A') => NOTE_COLOURS[5],
                            Some('B') | Some('H') => NOTE_COLOURS[6],
                            _ => "#dcdcdc",
                        };
                        content.push_str(&format!(
                            "<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"20\" rx=\"3\" ry=\"2\" fill=\"{colour}\" stroke=\"#898989\" stroke-width=\"1\"/>",
                            e.x + 32.0 + 12.0 * i as f32,
                            top + 9.0
                        ));
                    }
                }
                ElemKind::Button => {
                    let cx = e.x + BUTTON / 2.0;
                    let is_add = matches!(row.source, Source::AddRow);
                    content.push_str(&format!("<circle cx=\"{cx}\" cy=\"{centre}\" r=\"10\" fill=\"#ffffff\" fill-opacity=\"0.9\"/>"));
                    content.push_str(&format!("<path d=\"M {},{centre} h 10\" stroke=\"{fill}\" stroke-width=\"2.5\" stroke-linecap=\"round\"/>", cx - 5.0));
                    if is_add {
                        content.push_str(&format!("<path d=\"M {cx},{} v 10\" stroke=\"{fill}\" stroke-width=\"2.5\" stroke-linecap=\"round\"/>", centre - 5.0));
                    }
                }
                ElemKind::Inline => {
                    let child = e.item.and_then(|i| match &items[i] { Item::Socket(c, _) => c.as_ref(), _ => None });
                    let top = centre - e.h / 2.0;
                    match child {
                        Some(child) => {
                            let (child_svg, _, _) = render_block_in(child, theme, true, true, Some(&stroke_colour));
                            content.push_str(&format!("<g transform=\"translate({} {top})\">{child_svg}</g>", e.x));
                        }
                        None => {
                            let cw = e.shape.width(e.h);
                            let inner = e.w - 2.0 * cw;
                            holes.push_str(&format!(
                                "<path d=\"M {},{top} h {inner} {} h -{inner} {} z\" fill=\"{}\"/>",
                                e.x + cw,
                                e.shape.right_down(e.h),
                                e.shape.up(e.h),
                                stroke_colour
                            ));
                        }
                    }
                }
                ElemKind::Statement => {
                    if let Source::Mouth(i) = row.source {
                        if !p.mouths[i].body.is_empty() {
                            let (stack_svg, _, _) = render_stack(&p.mouths[i].body, theme);
                            content.push_str(&format!("<g transform=\"translate({} {})\">{stack_svg}</g>", e.x, row.y));
                        }
                    }
                }
            }
        }
    }

    let mut svg = String::new();
    svg.push_str(&format!("<path d=\"{d}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"1\"/>"));
    svg.push_str(&holes);
    svg.push_str(&content);
    let (extent_w, _) = extent(block);
    (svg, extent_w, l.body_h)
}

fn label_at(colors: &CategoryColors, text: &str, x: f32, baseline: f32, theme: &str) -> String {
    crate::blockly::label(colors, text, x, baseline, theme)
}
