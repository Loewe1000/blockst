#[cfg(target_arch = "wasm32")]
pub fn read_args(total_len: usize) -> Vec<u8> {
    let mut buf = vec![0; total_len];
    if total_len > 0 {
        unsafe {
            wasm_minimal_protocol_write_args_to_buffer(buf.as_mut_ptr());
        }
    }
    buf
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_args(_total_len: usize) -> Vec<u8> {
    vec![]
}

#[cfg(target_arch = "wasm32")]
pub fn send_bytes(bytes: Vec<u8>) -> i32 {
    let ptr = bytes.as_ptr();
    let len = bytes.len();
    unsafe {
        wasm_minimal_protocol_send_result_to_host(ptr, len);
    }
    std::mem::forget(bytes);
    0
}

#[cfg(not(target_arch = "wasm32"))]
pub fn send_bytes(_bytes: Vec<u8>) -> i32 {
    0
}

pub fn send_string(value: String) -> i32 {
    send_bytes(value.into_bytes())
}

#[cfg(target_arch = "wasm32")]
pub fn send_error(message: impl Into<String>) -> i32 {
    let bytes = message.into().into_bytes();
    let ptr = bytes.as_ptr();
    let len = bytes.len();
    unsafe {
        wasm_minimal_protocol_send_result_to_host(ptr, len);
    }
    std::mem::forget(bytes);
    1
}

#[cfg(not(target_arch = "wasm32"))]
pub fn send_error(_message: impl Into<String>) -> i32 {
    1
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "typst_env")]
extern "C" {
    fn wasm_minimal_protocol_write_args_to_buffer(ptr: *mut u8);
    fn wasm_minimal_protocol_send_result_to_host(ptr: *const u8, len: usize);
}
