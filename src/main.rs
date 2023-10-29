use std::str;

pub const FATAL: i32 = 3;
pub const ERROR: i32 = 2;
pub const WARN: i32 = 1;
pub const INFO: i32 = 0;
pub const DEBUG: i32 = -1;

pub const REQUEST_HEADER: u32 = 0;
pub const RESPONSE_HEADER: u32 = 1;

pub const REQUEST_BODY: u32 = 0;
pub const RESPONSE_BODY: u32 = 1;

fn main() {}

#[export_name="handle_request"]
pub fn http_request() -> u64 {
    let res: Vec<u8> = get_conf();
    send_log(DEBUG, format!("config {:?}", String::from_utf8(res).unwrap()).as_str());
    set_request_uri("/hello");

    writebody(REQUEST_BODY, "foobar");
    return 1 as u64;
}

#[export_name="handle_response"]
fn http_response(req_ctx: i32, is_error: i32) {
    //let code = status_code();
    //send_log(DEBUG, format!("code {:?}", code).as_str());
    println!("http_response: {0} {1}", req_ctx, is_error);
}

#[link(wasm_import_module = "http_handler")]
extern "C" {
    fn log(level: i32, message: *const u8, message_len: u32);
    fn get_config(buf: *const u8, buf_limit: i32) -> i32;
    fn get_method(buf: *const u8, buf_limit: i32) -> i32;
    fn set_method(ptr: *const u8, message_len: u32);
    fn get_uri(ptr: *const u8, message_len: u32) -> i32;
    fn set_uri(ptr: *const u8, message_len: u32); // TODO not working
    fn get_protocol_version(ptr: *const u8, message_len: u32) -> i32;
    fn add_header_value(header_kind: u32, name_ptr: *const u8, name_len: u32, value_ptr: *const u8, value_len: u32);
    fn set_header_value(header_kind: u32, name_ptr: *const u8, name_len: u32, value_ptr: *const u8, value_len: u32);
    fn remove_header(header_kind: u32, name_ptr: *const u8, name_len: u32);
    fn get_header_names(header_kind: u32, buf: *const u8, buf_limit: i32) -> i64;
    fn get_header_values(header_kind: u32, name_ptr: *const u8, name_len: u32, buf: *const u8, buf_limit: i32) -> i64;
    fn log_enabled(level: i32) -> i32;
    fn write_body(body_kind: u32, ptr: *const u8, message_len: u32); // TODO: not working
    fn get_status_code() -> i32; // TODO: not working
    fn set_status_code(code: i32);
}

pub fn set_code(code: i32) {
    unsafe { set_status_code(code)};
}

pub fn status_code() -> i32 {
    unsafe { 
        match get_status_code() {
            res=> return res
        }
    };
}

pub fn writebody(kind: u32, message: &str) {
    unsafe { write_body(kind, message.as_ptr(), message.len() as u32)};
}

pub fn log_enabl(level: i32) -> i32 {
    unsafe { 
        match log_enabled(level) {
            res => return res
        }
    };
}

pub fn get_header_val(kind: u32, name: &str) -> Vec<String> {
    let read_buf: [u8; 2048] = [0; 2048];
    unsafe { 
        match get_header_values(kind, name.as_ptr(), name.len() as u32, read_buf.as_ptr(), 2048) {
            len => {
                let data: &[u8] = &read_buf[0 .. len as usize];
                return str_array_from_u8_nul_utf8_unchecked(data);
            }
        }
    };
}

pub fn get_headers(kind: u32) -> Vec<String> {
    let read_buf: [u8; 2048] = [0; 2048];
    unsafe { 
        match get_header_names(kind, read_buf.as_ptr(), 2048) {
            len => {
                let data: &[u8] = &read_buf[0 .. len as usize];
                return str_array_from_u8_nul_utf8_unchecked(data);
            }
        }
    };
}

pub fn rem_header(kind: u32, name: &str) {
    unsafe { remove_header(kind, name.as_ptr(), name.len() as u32) };
}

pub fn set_header(kind: u32, name: &str, value: &str) {
    unsafe { set_header_value(kind, name.as_ptr(), name.len() as u32, value.as_ptr(), value.len() as u32) };
}

pub fn add_header(kind: u32, name: &str, value: &str) {
    unsafe { add_header_value(kind, name.as_ptr(), name.len() as u32, value.as_ptr(), value.len() as u32) };
}

pub fn send_log(level: i32, message: &str) {
    unsafe { log(level, message.as_ptr(), message.len() as u32) };
}

pub fn get_conf() -> Vec<u8> {
    let read_buf: [u8; 2048] = [0; 2048];
    unsafe { 
        match get_config(read_buf.as_ptr(), 2048) {
            len => {
                return read_buf[0 .. len as usize].to_vec();
            }
        }
    };
}

pub fn get_request_method() -> String {
    let read_buf: [u8; 2048] = [0; 2048];
    unsafe { 
        match get_method(read_buf.as_ptr(), 2048) {
            len => {
                return str::from_utf8(&read_buf[0 .. len as usize]).unwrap().to_string();
            }
        }
    };
}

pub fn set_request_method(method: &str) {
    unsafe { set_method(method.as_ptr(), method.len() as u32)};
}

pub fn get_request_uri() -> String {
    let read_buf: [u8; 2048] = [0; 2048];
    unsafe { 
        match get_uri(read_buf.as_ptr(), 2048) {
            len => {
                return str::from_utf8(&read_buf[0 .. len as usize]).unwrap().to_string();
            }
        }
    };
}

pub fn set_request_uri(uri: &str) {
    unsafe { set_uri(uri.as_ptr(), uri.len() as u32)};
}

pub fn get_request_protocol_version() -> String {
    let read_buf: [u8; 2048] = [0; 2048];
    unsafe { 
        match get_protocol_version(read_buf.as_ptr(), 2048) {
            len => {
                return str::from_utf8(&read_buf[0 .. len as usize]).unwrap().to_string();
            }
        }
    };
}

pub unsafe fn str_array_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut start_index: usize = 0;
    for (i, n) in utf8_src.iter().enumerate() {
        if *n == b'\0' {
            out.push(::std::str::from_utf8_unchecked(&utf8_src[start_index..i]).to_string());
            start_index = i + 1; // skip NUL byte
        }
    }
    return out
}