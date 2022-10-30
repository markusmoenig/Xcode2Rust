use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn shipping_rust_addition(a: c_int, b: c_int) -> c_int {
    a + b
}

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn foo_new(pstext: *mut u8, length: u32) {
    let slice = unsafe { std::slice::from_raw_parts_mut(pstext, length as usize) };
    //let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(slice) };

    for i in 0..length as usize {
        slice[i] = 255;
    }
}

#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}