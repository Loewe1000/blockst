//! The internal document the NEPO renderer draws.
//!
//! Blockst already has a document model for Scratch (`scratchblocks-wasm`).
//! This one is deliberately separate rather than a variant of it, because the
//! two dialects disagree about the thing the model is built around: a Scratch
//! block is a flat run of segments, a NEPO block is a stack of *rows*, and
//! each row carries at most one connection. Bolting rows onto the Scratch
//! `BlockSpec` would leave every Scratch block with an empty `rows` field and
//! every NEPO block with an empty `segments` field.
//!
//! `Dialect` is still carried through, so that if the two renderers ever move
//! into one crate the dispatch point already exists and is explicit.

use serde::{Deserialize, Deserializer, Serialize};

/// A connection's accepted data types.
///
/// Blockly lets `setCheck` take either one type or a list, and the difference
/// is visible: `drawInputType_` only paints a socket in its type colour when
/// there is exactly one type to name. "Zeige Text" accepts Number, Boolean and
/// String, so its empty socket stays plain — modelling that as a single
/// "String" would invent a hint Open Roberta does not give.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum RawCheck {
    One(String),
    Many(Vec<String>),
}

pub fn deserialize_checks<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Option::<RawCheck>::deserialize(deserializer)? {
        None => Vec::new(),
        Some(RawCheck::One(one)) => vec![one],
        Some(RawCheck::Many(many)) => many,
    })
}

/// The single type a connection names, or `None` when it accepts several and
/// therefore names none of them.
pub fn single_check(checks: &[String]) -> Option<&str> {
    match checks {
        [only] => Some(only.as_str()),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    Scratch,
    Nepo,
}

impl Default for Dialect {
    fn default() -> Self {
        Dialect::Nepo
    }
}

#[derive(Debug, Deserialize)]
pub struct DocumentSpec {
    #[serde(default)]
    pub dialect: Dialect,
    pub scale: Option<f32>,
    pub theme: Option<String>,
    #[serde(default = "default_font")]
    pub font: String,
    pub scripts: Vec<ScriptSpec>,
}

fn default_font() -> String {
    // Open Roberta's own stylesheet says `font-family: sans-serif` and nothing
    // more, so the reference rendering is whatever the browser's sans-serif
    // is. Helvetica/Arial is the closest stable stand-in.
    "Helvetica Neue, Helvetica, Arial, sans-serif".to_string()
}

#[derive(Debug, Deserialize)]
pub struct ScriptSpec {
    pub blocks: Vec<BlockSpec>,
}

/// One NEPO block.
///
/// `segments` exists only so a future merged model can hold a Scratch block in
/// the same type; the NEPO renderer never reads it.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BlockSpec {
    #[serde(default)]
    pub dialect: Option<String>,
    /// Catalog id, e.g. `mbedActions_display_text`. Kept for tests and for
    /// error messages.
    #[serde(default)]
    pub id: String,
    /// `start` | `statement` | `cap` | `value`
    pub shape: String,
    pub category: String,
    /// Data types this block reports, for `value` blocks: Number, String,
    /// Boolean, Colour, Image. A single type colours the output plug.
    #[serde(default, deserialize_with = "deserialize_checks")]
    pub check: Vec<String>,
    #[serde(default)]
    pub segments: Vec<SegmentSpec>,
    #[serde(default)]
    pub rows: Vec<RowSpec>,
}

/// A NEPO block is laid out row by row, top to bottom.
///
/// Blockly calls these "inputs": a `dummy` row is fields only, a `value` row
/// ends in a puzzle socket on the right edge, a `statement` row opens the
/// block's mouth. The connection lives on the row and not on the block
/// because one block can have several — the Start block grows a declaration
/// mouth, `if/else` has two — which is exactly what a single `body` field on
/// the block cannot express.
#[derive(Debug, Clone, Deserialize)]
pub struct RowSpec {
    #[serde(default = "default_row_kind")]
    pub kind: String,
    #[serde(default = "default_align")]
    pub align: String,
    #[serde(default)]
    pub fields: Vec<SegmentSpec>,
    /// Data types accepted by this row's socket.
    #[serde(default, deserialize_with = "deserialize_checks")]
    pub check: Vec<String>,
    /// Block plugged into a `value` row.
    #[serde(default)]
    pub value: Option<Box<BlockSpec>>,
    /// Stack nested in a `statement` row.
    #[serde(default)]
    pub body: Vec<BlockSpec>,
}

fn default_row_kind() -> String {
    "dummy".to_string()
}

fn default_align() -> String {
    "left".to_string()
}

impl RowSpec {
    pub fn is_value(&self) -> bool {
        self.kind == "value"
    }
    pub fn is_statement(&self) -> bool {
        self.kind == "statement"
    }
    pub fn is_dummy(&self) -> bool {
        self.kind == "dummy"
    }
}

/// The pieces a row is made of.
///
/// This is the union the prototype set out to test: `Text`/`Icon`/`Input`/
/// `Block` carry over from the Scratch model, `Dropdown`, `ColourField` and
/// `PixelMatrix` are the typed NEPO fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum SegmentSpec {
    /// Static label text.
    #[serde(rename = "text")]
    Text {
        value: String,
        /// Open Roberta sets `class="blocklyText monospace"` on the image
        /// block's row and column rulers.
        #[serde(default)]
        monospace: bool,
    },
    #[serde(rename = "icon")]
    Icon { name: String },
    /// An editable text field drawn as a white rounded box.
    #[serde(rename = "input")]
    Input { value: String },
    /// An inline nested block. Unused by the six prototype blocks — NEPO
    /// blocks connect externally — but kept so the variant list matches the
    /// Scratch model.
    #[serde(rename = "block")]
    Block { block: Box<BlockSpec> },
    /// A value socket that lives among a row's fields, as used by arithmetic
    /// and comparison blocks. Unlike `RowSpec::value`, this is not a notch on
    /// the right edge of the surrounding block.
    #[serde(rename = "inline_value")]
    InlineValue {
        #[serde(default, deserialize_with = "deserialize_checks")]
        check: Vec<String>,
        #[serde(default)]
        value: Option<Box<BlockSpec>>,
    },
    #[serde(rename = "dropdown")]
    Dropdown { value: String },
    #[serde(rename = "colour")]
    ColourField { colour: String },
    /// The 5x5 LED matrix. `rows` holds one string per matrix row; `.` or a
    /// space is off, `#` is fully on, `1`-`9` are Calliope brightness steps.
    ///
    /// This is the *catalog-facing* form. `matrix::expand` rewrites it into
    /// the six ruler-and-cell rows Open Roberta actually builds, so that
    /// layout and rendering never learn about matrices at all.
    #[serde(rename = "matrix")]
    PixelMatrix { rows: Vec<String> },
    /// One `Blockly.FieldPixelbox`: a fixed 16x16 box holding a single
    /// brightness character. Produced by `matrix::expand`, never written by
    /// hand in the catalog.
    #[serde(rename = "pixel")]
    PixelCell { value: String },
    /// A compact predefined 5x5 image selected by Open Roberta's image
    /// dropdown. The renderer draws a small LED preview rather than relying
    /// on the lab's PNG asset bundle.
    #[serde(rename = "image")]
    Image { value: String },
}

impl SegmentSpec {
    /// Blockly's `field.EDITABLE`, which decides whether a separating
    /// `SEP_SPACE_X` is inserted between two neighbouring fields.
    pub fn is_editable(&self) -> bool {
        matches!(
            self,
            SegmentSpec::Input { .. }
                | SegmentSpec::Dropdown { .. }
                | SegmentSpec::ColourField { .. }
                | SegmentSpec::PixelMatrix { .. }
                | SegmentSpec::PixelCell { .. }
                | SegmentSpec::Image { .. }
        )
    }
}
