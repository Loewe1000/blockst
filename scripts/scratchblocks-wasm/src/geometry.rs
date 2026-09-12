//! The measurements that decide what a block looks like.
//!
//! These numbers used to sit as literals wherever they were needed, which
//! meant the same value was written down in three files: `svg.rs` drew the
//! notch at 48, `measure.rs` reserved space up to 48, and `render.rs` placed
//! the first label after 48. Nothing said they were the same 48 — and two of
//! them are not. The row height is also 48 in Scratch, purely by coincidence,
//! and a block language where the notch sits at 30 and the row is 25 high
//! would have had to change all three and hope.
//!
//! So they are named here instead. A profile picks a `Geometry`, the document
//! installs it, and the drawing code asks for measurements rather than
//! repeating them.
//!
//! What is deliberately *not* here yet: the per-shape paddings, and the notch
//! curve itself. Those belong to the shape functions that draw them, and they
//! get parameterized when the second set of shapes is written — a number that
//! only one function reads is not yet worth a field.

use std::cell::RefCell;

/// Which family of outlines to draw. The measurements below differ per
/// profile; the *shapes* differ per family, and there are only two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shapes {
    Scratch,
    Blockly,
}

#[derive(Clone, Debug)]
pub struct Geometry {
    pub shapes: Shapes,
    /// Height of a plain block row, and the floor it may not scale below.
    pub row_height: f32,
    pub row_height_min: f32,
    /// Radius of the outer corners.
    pub corner_radius: f32,
    /// Where the top/bottom notch begins and ends, measured from the left.
    /// In Scratch these are 12 and 48; the distance between them is the
    /// notch, everything outside it is straight edge.
    pub notch_start: f32,
    pub notch_end: f32,
    /// Narrowest a C-block's mouth may be, and its floor under scaling.
    pub mouth_min_width: f32,
    pub mouth_min_width_min: f32,
    /// How far the body of a C-block is indented from the block's edge.
    pub body_indent: f32,
    /// Corner radius of the hat over a procedure definition, which is
    /// rounder than an ordinary block.
    pub proc_hat_radius: f32,

    // --- Blockly only ---
    /// Flat run in the middle of the notch. Blockly's notch is a trapezium:
    /// two ramps of `(notch_width - notch_inner) / 2` around this.
    pub notch_inner: f32,
    pub notch_depth: f32,
    /// The puzzle tab that a value block plugs in with, and how far below the
    /// top edge it sits. Scratch has no equivalent — its reporters are pills.
    pub tab_width: f32,
    pub tab_height: f32,
    pub tab_offset_from_top: f32,
    /// Width of the dome over a hat block, and the height it bulges. Blockly
    /// draws the curve `height` high but reserves only three quarters of it.
    pub hat_width: f32,
    pub hat_curve_height: f32,
    /// Draw the raised edge that makes a block look embossed. The classic
    /// Blockly look has it; thrasos and Scratch are flat.
    pub bevel: bool,
    /// Classic Blockly rounds only the left corners; the right ones are
    /// square. Scratch and today's Blockly round all four.
    pub square_right_corners: bool,
    /// Today's Blockly lays rows out between 5px top and bottom rows with
    /// spacer rows between them (`blockly_modern.rs`); the pre-2019
    /// renderer packs everything into 25px rows (`blockly.rs`).
    pub spacer_rows: bool,
    /// MakeCode's renderer: Blockly's zelos, rows on a grid of four with
    /// pills and hexagons instead of puzzle tabs (`blockly_zelos.rs`).
    pub zelos: bool,
    /// The label font. Scratch draws 12pt medium; Blockly 11pt regular, and
    /// the widths are measured with the same face, so this decides the
    /// layout as much as the look.
    pub font_size_pt: f32,
    pub font_weight: u16,

    /// Input fields. Scratch draws a pill — a radius of half the height —
    /// while Blockly draws a small rounded rectangle. `field_radius` of
    /// `None` means "half the height, so it stays a pill".
    pub field_height: f32,
    pub field_height_min: f32,
    pub field_radius: Option<f32>,

    /// What a row is built out of: the height of its tallest ordinary
    /// content, plus the padding each block shape adds around it. Scratch
    /// puts 32 of content in a 48 row; Blockly puts 16 in a 24 row.
    pub content_height: f32,
    pub stack_padding: f32,
    pub cap_padding: f32,
    pub hat_padding: f32,
    /// Narrowest a value block may be.
    pub value_min_width: f32,
    pub value_min_width_min: f32,
}

impl Geometry {
    /// Scratch 3, exactly as the renderer always drew it.
    pub fn scratch() -> Self {
        Self {
            shapes: Shapes::Scratch,
            row_height: 48.0,
            row_height_min: 36.0,
            corner_radius: 4.0,
            notch_start: 12.0,
            notch_end: 48.0,
            mouth_min_width: 96.0,
            mouth_min_width_min: 72.0,
            body_indent: 16.0,
            proc_hat_radius: 20.0,
            notch_inner: 3.0,
            notch_depth: 4.0,
            tab_width: 8.0,
            tab_height: 15.0,
            tab_offset_from_top: 5.0,
            hat_width: 100.0,
            hat_curve_height: 15.0,
            bevel: false,
            square_right_corners: false,
            spacer_rows: false,
            zelos: false,
            font_size_pt: 12.0,
            font_weight: 500,
            field_height: 32.0,
            field_height_min: 24.0,
            field_radius: None,
            content_height: 32.0,
            stack_padding: 16.0,
            cap_padding: 8.0,
            hat_padding: 32.0,
            value_min_width: 40.0,
            value_min_width_min: 30.0,
        }
    }

    /// Blockly as it looks today, in the thrasos flavour: flat, solid edges.
    ///
    /// thrasos defines no constants of its own — it uses Blockly's base
    /// `ConstantProvider` and differs from geras only in how it measures
    /// rows. It has no drawer either, which is why there is no bevel here.
    pub fn blockly_modern() -> Self {
        Self {
            shapes: Shapes::Blockly,
            row_height: 24.0,
            row_height_min: 18.0,
            corner_radius: 8.0,
            // NOTCH_OFFSET_LEFT = 15, NOTCH_WIDTH = 15.
            notch_start: 15.0,
            notch_end: 30.0,
            mouth_min_width: 40.0,
            mouth_min_width_min: 30.0,
            body_indent: 20.0,
            proc_hat_radius: 8.0,
            notch_inner: 3.0,
            notch_depth: 4.0,
            tab_width: 8.0,
            tab_height: 15.0,
            tab_offset_from_top: 5.0,
            hat_width: 100.0,
            hat_curve_height: 15.0,
            bevel: false,
            square_right_corners: true,
            spacer_rows: true,
            zelos: false,
            font_size_pt: 11.0,
            font_weight: 400,
            field_height: 16.0,
            field_height_min: 12.0,
            field_radius: Some(4.0),
            // A Blockly row is 24 high: a 16px field with 4px above and
            // below. The hat adds the three quarters of its curve that
            // Blockly reserves.
            content_height: 16.0,
            stack_padding: 8.0,
            cap_padding: 8.0,
            hat_padding: 19.0,
            value_min_width: 24.0,
            value_min_width_min: 18.0,
        }
    }

    /// Blockly before 2019 — the generation jwinf.de loads.
    ///
    /// The notch is the same shape in both generations: the modern formula
    /// reproduces the old literal `l 6,4 3,0 6,-4` exactly, and both sit
    /// between 15 and 30. What actually differs is the taller puzzle tab, the
    /// slightly taller row, the flat 10px spacing, and the embossed edge.
    pub fn blockly_classic() -> Self {
        Self {
            row_height: 25.0,
            row_height_min: 19.0,
            tab_height: 20.0,
            bevel: true,
            square_right_corners: true,
            spacer_rows: false,
            zelos: false,
            ..Self::blockly_modern()
        }
    }

    /// MakeCode (pxt): Blockly's zelos renderer with a 12pt semibold
    /// monospace label. The zelos numbers live in `blockly_zelos.rs`; what
    /// the shared code needs is the font and the flag.
    pub fn makecode() -> Self {
        Self {
            row_height: 32.0,
            row_height_min: 24.0,
            corner_radius: 4.0,
            notch_start: 12.0,
            notch_end: 48.0,
            notch_inner: 12.0,
            notch_depth: 8.0,
            bevel: false,
            square_right_corners: false,
            spacer_rows: false,
            zelos: true,
            font_size_pt: 12.0,
            font_weight: 600,
            field_height: 34.0,
            field_height_min: 24.0,
            field_radius: Some(4.0),
            ..Self::blockly_modern()
        }
    }
}

/// Look a profile up by name. An unknown name falls back to Scratch rather
/// than failing, so a document written against a newer version still renders.
pub fn for_profile(name: Option<&str>) -> Geometry {
    match name.unwrap_or("scratch") {
        "makecode" | "makecode-calliope" => Geometry::makecode(),
        "blockly" | "blockly-modern" => Geometry::blockly_modern(),
        "blockly-klassisch" | "blockly-classic" | "jwinf" | "jwinf-turtle" => Geometry::blockly_classic(),
        _ => Geometry::scratch(),
    }
}

thread_local! {
    static CURRENT: RefCell<Option<Geometry>> = const { RefCell::new(None) };
}

/// Install the geometry for the document about to be rendered.
pub fn set_geometry(geometry: Geometry) {
    CURRENT.with(|current| *current.borrow_mut() = Some(geometry));
}

pub fn clear_geometry() {
    CURRENT.with(|current| *current.borrow_mut() = None);
}

/// The geometry in force, defaulting to Scratch's.
pub fn geometry() -> Geometry {
    CURRENT.with(|current| {
        let mut borrowed = current.borrow_mut();
        if borrowed.is_none() {
            *borrowed = Some(Geometry::scratch());
        }
        borrowed.as_ref().unwrap().clone()
    })
}
