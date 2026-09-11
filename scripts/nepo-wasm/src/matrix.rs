//! The 5x5 LED matrix — the one place the prototype allows special code.
//!
//! Open Roberta does not have a "matrix field". `mbedImage_image` is an
//! ordinary Blockly block that happens to be built out of six right-aligned
//! dummy rows: one monospace column ruler, then five rows of a monospace row
//! label followed by five `FieldPixelbox` fields (`blocks/mbedImage.js`).
//!
//! So instead of teaching the layout engine about matrices, this module
//! rewrites the catalog's single `PixelMatrix` segment into exactly those six
//! rows. Everything downstream — measurement, alignment, the outline, the
//! typed output plug — then treats the image block like any other block.
//!
//! That containment is the interesting result: the special case is 60 lines of
//! *data construction* and zero lines in `measure.rs` or `render.rs`.

use crate::model::{BlockSpec, RowSpec, SegmentSpec};

/// Grid size for Calliope mini and micro:bit. `blocks/mbedImage.js` picks
/// 16x8 for mBot; the prototype only claims the 5x5 devices.
pub const SIZE: usize = 5;

/// Rewrite any `PixelMatrix` segment into Open Roberta's real row structure.
pub fn expand(block: &mut BlockSpec) {
    let mut expanded: Vec<RowSpec> = Vec::new();

    for row in std::mem::take(&mut block.rows) {
        let matrix = row.fields.iter().find_map(|field| match field {
            SegmentSpec::PixelMatrix { rows } => Some(rows.clone()),
            _ => None,
        });

        match matrix {
            Some(cells) => expanded.extend(matrix_rows(&cells)),
            None => {
                let mut row = row;
                for field in row.fields.iter_mut() {
                    if let SegmentSpec::Block { block } = field {
                        expand(block);
                    }
                    if let SegmentSpec::InlineValue {
                        value: Some(block), ..
                    } = field
                    {
                        expand(block);
                    }
                }
                if let Some(value) = row.value.as_mut() {
                    expand(value);
                }
                for child in row.body.iter_mut() {
                    expand(child);
                }
                expanded.push(row);
            }
        }
    }

    block.rows = expanded;
}

fn matrix_rows(cells: &[String]) -> Vec<RowSpec> {
    let mut rows = Vec::with_capacity(SIZE + 1);

    // Column ruler: "0", then "  1" .. "  4" (two leading spaces, monospace).
    let mut ruler = vec![SegmentSpec::Text {
        value: "0".to_string(),
        monospace: true,
    }];
    for column in 1..SIZE {
        ruler.push(SegmentSpec::Text {
            value: format!("  {column}"),
            monospace: true,
        });
    }
    rows.push(dummy_row(ruler));

    for index in 0..SIZE {
        let mut fields = vec![SegmentSpec::Text {
            value: index.to_string(),
            monospace: true,
        }];
        let source: Vec<char> = cells
            .get(index)
            .map(|row| row.chars().collect())
            .unwrap_or_default();
        for column in 0..SIZE {
            fields.push(SegmentSpec::PixelCell {
                value: cell_value(source.get(column).copied()),
            });
        }
        rows.push(dummy_row(fields));
    }

    rows
}

/// `Blockly.FieldPixelbox.validate_`: a blank cell is off, `#` is fully on,
/// and `1`-`8` are the Calliope's intermediate brightness steps. `9` is
/// normalised to `#` and `0` to blank, exactly as the validator does.
fn cell_value(source: Option<char>) -> String {
    match source {
        Some('#') | Some('9') => "#".to_string(),
        Some(c) if c.is_ascii_digit() && c != '0' => c.to_string(),
        _ => String::new(),
    }
}

fn dummy_row(fields: Vec<SegmentSpec>) -> RowSpec {
    RowSpec {
        kind: "dummy".to_string(),
        // `setAlign(Blockly.ALIGN_RIGHT)` on every row of the image block.
        align: "right".to_string(),
        fields,
        check: Vec::new(),
        value: None,
        body: Vec::new(),
    }
}
