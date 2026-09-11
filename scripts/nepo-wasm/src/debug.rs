//! A JSON view of the parsed blocks.
//!
//! `BlockSpec` is a deserialise-only type, so tests and callers that want the
//! AST get this mirror instead of `#[derive(Serialize)]` on the model. Keeping
//! them apart means the wire format for tests can change without disturbing
//! what the renderer reads.

use crate::model::{BlockSpec, SegmentSpec};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Node {
    pub id: String,
    pub shape: String,
    pub category: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub check: Vec<String>,
    pub rows: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct Row {
    pub kind: String,
    /// `setAlign`. Only the image block and the list block use anything but
    /// the default, so it is left out when it says nothing.
    #[serde(skip_serializing_if = "is_left")]
    pub align: String,
    pub fields: Vec<Field>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub check: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub body: Vec<Node>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum Field {
    #[serde(rename = "text")]
    Text { value: String },
    #[serde(rename = "dropdown")]
    Dropdown { value: String },
    #[serde(rename = "input")]
    Input { value: String },
    #[serde(rename = "colour")]
    Colour { value: String },
    #[serde(rename = "matrix")]
    Matrix { rows: Vec<String> },
    #[serde(rename = "pixel")]
    Pixel { value: String },
    #[serde(rename = "image")]
    Image { value: String },
    #[serde(rename = "inline_value")]
    InlineValue {
        check: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Box<Node>>,
    },
    #[serde(rename = "other")]
    Other,
}

fn is_left(align: &str) -> bool {
    align == "left"
}

pub fn describe(scripts: &[Vec<BlockSpec>]) -> Vec<Vec<Node>> {
    scripts
        .iter()
        .map(|blocks| describe_stack(blocks))
        .collect()
}

fn describe_stack(blocks: &[BlockSpec]) -> Vec<Node> {
    blocks.iter().map(describe_block).collect()
}

fn describe_block(block: &BlockSpec) -> Node {
    Node {
        id: block.id.clone(),
        shape: block.shape.clone(),
        category: block.category.clone(),
        check: block.check.clone(),
        rows: block
            .rows
            .iter()
            .map(|row| Row {
                kind: row.kind.clone(),
                align: row.align.clone(),
                fields: row.fields.iter().map(describe_field).collect(),
                check: row.check.clone(),
                value: row.value.as_ref().map(|v| Box::new(describe_block(v))),
                body: describe_stack(&row.body),
            })
            .collect(),
    }
}

fn describe_field(field: &SegmentSpec) -> Field {
    match field {
        SegmentSpec::Text { value, .. } => Field::Text {
            value: value.clone(),
        },
        SegmentSpec::Dropdown { value } => Field::Dropdown {
            value: value.clone(),
        },
        SegmentSpec::Input { value } => Field::Input {
            value: value.clone(),
        },
        SegmentSpec::ColourField { colour } => Field::Colour {
            value: colour.clone(),
        },
        SegmentSpec::PixelMatrix { rows } => Field::Matrix { rows: rows.clone() },
        SegmentSpec::PixelCell { value } => Field::Pixel {
            value: value.clone(),
        },
        SegmentSpec::Image { value } => Field::Image {
            value: value.clone(),
        },
        SegmentSpec::InlineValue { check, value } => Field::InlineValue {
            check: check.clone(),
            value: value.as_ref().map(|block| Box::new(describe_block(block))),
        },
        _ => Field::Other,
    }
}
