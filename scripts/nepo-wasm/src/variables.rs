//! Variable types, from the declaration to every use.
//!
//! Open Roberta types a variable once, in the declaration inside the Start
//! block, and every other block then follows: `variables_get` reports that
//! type (`setType`), `variables_set` accepts only that type in its socket
//! (`getInput('VALUE').setCheck`). Those are `onchange` handlers in Blockly,
//! running on a live workspace; here the same information arrives in one pass
//! over the parsed document, because a rendered document is the finished
//! workspace.
//!
//! Why it is worth doing at all: the type is *visible*. A typed plug is drawn
//! in the data-type colour, and an empty socket announces what belongs in it.
//! Without this pass every variable would be violet and every empty setter
//! socket blank, which is precisely the hint a worksheet is trying to give.

use crate::model::{BlockSpec, SegmentSpec};
use std::collections::HashMap;

pub const DECLARE: &str = "robGlobalVariables_declare";

/// Type every variable use in the document, from the declarations it contains.
///
/// Declarations are global — Blockly puts them in the Start block, not in the
/// script that uses them — so one table serves every script on the page.
pub fn apply_types(scripts: &mut [Vec<BlockSpec>]) {
    let mut types: HashMap<String, String> = HashMap::new();
    for blocks in scripts.iter() {
        collect(blocks, &mut types);
    }
    if types.is_empty() {
        return;
    }
    for blocks in scripts.iter_mut() {
        apply(blocks, &types);
    }
    for blocks in scripts.iter_mut() {
        retype_lists(blocks);
    }
}

/// What a list block hands on, once its list is known.
///
/// `robLists_getIndex.onchange` reads the type off whatever is plugged into
/// the list socket and re-types itself: an `Array_String` in, a `String` out.
/// `robLists_setIndex` does the same to the socket it writes into. Both run
/// after the variables pass, because it is usually a declared variable that
/// supplies the list in the first place.
fn retype_lists(blocks: &mut [BlockSpec]) {
    for block in blocks.iter_mut() {
        for row in block.rows.iter_mut() {
            for field in row.fields.iter_mut() {
                if let SegmentSpec::InlineValue {
                    value: Some(child), ..
                } = field
                {
                    retype_lists(std::slice::from_mut(child));
                }
            }
            if let Some(value) = row.value.as_deref_mut() {
                retype_lists(std::slice::from_mut(value));
            }
            retype_lists(&mut row.body);
        }

        let Some(item) = list_item_type(block) else {
            continue;
        };
        match block.id.as_str() {
            "robLists_getIndex" => block.check = vec![item],
            "robLists_setIndex" => {
                // The value written in is the last socket on the row.
                if let Some(SegmentSpec::InlineValue { check, .. }) = block
                    .rows
                    .iter_mut()
                    .flat_map(|row| row.fields.iter_mut())
                    .filter(|field| matches!(field, SegmentSpec::InlineValue { .. }))
                    .last()
                {
                    *check = vec![item];
                }
            }
            _ => {}
        }
    }
}

/// The element type of the list plugged into a block's first socket.
fn list_item_type(block: &BlockSpec) -> Option<String> {
    let list = block
        .rows
        .iter()
        .flat_map(|row| &row.fields)
        .find_map(|field| match field {
            SegmentSpec::InlineValue {
                value: Some(child), ..
            } => Some(child),
            _ => None,
        })?;
    let [only] = list.check.as_slice() else {
        return None;
    };
    only.strip_prefix("Array_").map(str::to_string)
}

fn collect(blocks: &[BlockSpec], types: &mut HashMap<String, String>) {
    for block in blocks {
        if block.id == DECLARE {
            if let (Some(name), Some(kind)) = (declared_name(block), declared_type(block)) {
                types.insert(name, kind);
            }
        }
        for row in &block.rows {
            for field in &row.fields {
                match field {
                    SegmentSpec::Block { block } => collect(std::slice::from_ref(block), types),
                    SegmentSpec::InlineValue {
                        value: Some(block), ..
                    } => collect(std::slice::from_ref(block), types),
                    _ => {}
                }
            }
            if let Some(value) = row.value.as_deref() {
                collect(std::slice::from_ref(value), types);
            }
            collect(&row.body, types);
        }
    }
}

/// The name field of a declaration: its one editable text box.
fn declared_name(block: &BlockSpec) -> Option<String> {
    block.rows.iter().flat_map(|row| &row.fields).find_map(|f| {
        match f {
            SegmentSpec::Input { value } if !value.is_empty() => Some(value.clone()),
            _ => None,
        }
    })
}

/// The type is already on the declaration's socket: `check_from` put it there
/// when the block was built, which is the same thing `updateShape_` does.
fn declared_type(block: &BlockSpec) -> Option<String> {
    block
        .rows
        .iter()
        .find(|row| row.is_value())
        .and_then(|row| row.check.first().cloned())
}

fn apply(blocks: &mut [BlockSpec], types: &HashMap<String, String>) {
    for block in blocks.iter_mut() {
        match block.id.as_str() {
            // `variables_get.setType`: the reporter's output plug takes the
            // declared type, and with it the colour that names it.
            "variables_get" => {
                if let Some(kind) = used_name(block).and_then(|name| types.get(&name)) {
                    block.check = vec![kind.clone()];
                }
            }
            // `variables_set.setType`: the socket accepts the declared type,
            // so an empty setter shows which kind of value is missing.
            "variables_set" => {
                if let Some(kind) = used_name(block).and_then(|name| types.get(&name)).cloned() {
                    if let Some(row) = block.rows.iter_mut().find(|row| row.is_value()) {
                        row.check = vec![kind];
                    }
                }
            }
            _ => {}
        }

        for row in block.rows.iter_mut() {
            for field in row.fields.iter_mut() {
                match field {
                    SegmentSpec::Block { block } => apply(std::slice::from_mut(block), types),
                    SegmentSpec::InlineValue {
                        value: Some(block), ..
                    } => apply(std::slice::from_mut(block), types),
                    _ => {}
                }
            }
            if let Some(value) = row.value.as_deref_mut() {
                apply(std::slice::from_mut(value), types);
            }
            apply(&mut row.body, types);
        }
    }
}

/// The name a getter or setter points at: its `FieldVariable`, which the
/// catalog builds as a dropdown.
fn used_name(block: &BlockSpec) -> Option<String> {
    block
        .rows
        .iter()
        .flat_map(|row| &row.fields)
        .find_map(|field| match field {
            SegmentSpec::Dropdown { value } => Some(value.clone()),
            _ => None,
        })
}
