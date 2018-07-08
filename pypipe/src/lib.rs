#![feature(libc)]
extern crate libc;

use libc::{c_char, uint32_t};
use std::ffi::CString;

// #[no_mangle]
// pub extern "C" fn call_from_python(s: *const c_char) -> uint32_t {
//     let c_str = unsafe {
//         assert!(!s.is_null());

//         CStr::from_ptr(s)
//     };

//     let r_str = c_str.to_str().unwrap();
//     r_str.chars().count() as uint32_t
// }

#[no_mangle]
pub extern fn test_ffi(i: i32) -> bool  //*mut c_char
{
    if i > 0
    {
    	// let s = CString::new("hello").unwrap();
    	// s.into_raw()
    	true
    }
    else
    {
    	// let s = CString::new("error").unwrap();
    	// s.into_raw()
    	false
    }
}
