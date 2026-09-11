//! NEPO (Open Roberta) block rendering for Blockst.
//!
//! Deliberately a second plugin rather than a second mode of the Scratch one.
//! The two dialects share nothing at the geometry level — different notches,
//! different corner radii, different idea of what a block is made of — and the
//! prototype has to leave the existing Scratch output untouched to be worth
//! anything. What they do share sits above this crate, in Typst: the font
//! measurement pass and the image wrapper.

// The modules are public so the geometry cross-check in `tests/` can compare
// layout numbers against the independent JavaScript reference. Nothing outside
// this crate is meant to depend on them.
pub mod catalog;
pub mod debug;
pub mod matrix;
pub mod measure;
pub mod model;
pub mod parser;
pub mod variables;
mod protocol;
pub mod render;
pub mod svg;
pub mod theme;

pub use parser::{extract_texts, parse_request, render_request};

use protocol::{read_args, send_error, send_string};

fn input_string(json_len: u32, entry: &str) -> Result<String, i32> {
    let bytes = read_args(json_len as usize);
    match String::from_utf8(bytes) {
        Ok(text) => Ok(text),
        Err(_) => Err(send_error(format!(
            "nepo-wasm: {entry} expected UTF-8 bytes."
        ))),
    }
}

#[no_mangle]
pub extern "C" fn render_code_json(json_len: u32) -> i32 {
    let input = match input_string(json_len, "render_code_json") {
        Ok(text) => text,
        Err(code) => return code,
    };
    match render_request(&input) {
        Ok(svg) => send_string(svg),
        Err(err) => send_error(err),
    }
}

#[no_mangle]
pub extern "C" fn parse_json(json_len: u32) -> i32 {
    let input = match input_string(json_len, "parse_json") {
        Ok(text) => text,
        Err(code) => return code,
    };
    match parse_request(&input) {
        Ok(json) => send_string(json),
        Err(err) => send_error(err),
    }
}

#[no_mangle]
pub extern "C" fn extract_texts_json(json_len: u32) -> i32 {
    let input = match input_string(json_len, "extract_texts_json") {
        Ok(text) => text,
        Err(code) => return code,
    };
    match extract_texts(&input) {
        Ok(json) => send_string(json),
        Err(err) => send_error(err),
    }
}
