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

#[derive(Clone, Debug)]
pub struct Geometry {
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
}

impl Geometry {
    /// Scratch 3, exactly as the renderer always drew it.
    pub fn scratch() -> Self {
        Self {
            row_height: 48.0,
            row_height_min: 36.0,
            corner_radius: 4.0,
            notch_start: 12.0,
            notch_end: 48.0,
            mouth_min_width: 96.0,
            mouth_min_width_min: 72.0,
            body_indent: 16.0,
            proc_hat_radius: 20.0,
        }
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
