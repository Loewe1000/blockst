mod measure;
mod model;
mod generated;
mod parser;
mod protocol;
mod render;
mod svg;
mod theme;

use model::DocumentSpec;
pub use parser::{parse_request_json, render_request_json, extract_texts_json};
use protocol::{read_args, send_error, send_string};
use render::render_document;

#[no_mangle]
pub extern "C" fn render_json(json_len: u32) -> i32 {
    let input = read_args(json_len as usize);
    let input = match std::str::from_utf8(&input) {
        Ok(text) => text,
        Err(_) => return send_error("scratchblocks-wasm: render_json expected UTF-8 bytes."),
    };

    let document: DocumentSpec = match serde_json::from_str(input) {
        Ok(document) => document,
        Err(err) => return send_error(format!("scratchblocks-wasm: invalid JSON input: {err}")),
    };

    send_string(render_document(&document))
}

#[no_mangle]
pub extern "C" fn parse_json(json_len: u32) -> i32 {
    let input = read_args(json_len as usize);
    let input = match std::str::from_utf8(&input) {
        Ok(text) => text,
        Err(_) => return send_error("scratchblocks-wasm: parse_json expected UTF-8 bytes."),
    };

    match parse_request_json(input) {
        Ok(result) => send_string(result),
        Err(err) => send_error(err),
    }
}

#[no_mangle]
pub extern "C" fn render_code_json(json_len: u32) -> i32 {
    let input = read_args(json_len as usize);
    let input = match std::str::from_utf8(&input) {
        Ok(text) => text,
        Err(_) => return send_error("scratchblocks-wasm: render_code_json expected UTF-8 bytes."),
    };

    match render_request_json(input) {
        Ok(result) => send_string(result),
        Err(err) => send_error(err),
    }
}

#[no_mangle]
pub extern "C" fn extract_texts(json_len: u32) -> i32 {
    let input = read_args(json_len as usize);
    let input = match std::str::from_utf8(&input) {
        Ok(text) => text,
        Err(_) => return send_error("scratchblocks-wasm: extract_texts expected UTF-8 bytes."),
    };

    match extract_texts_json(input) {
        Ok(result) => send_string(result),
        Err(err) => send_error(err),
    }
}
