use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice;

pub mod ver_cut;
pub mod ver_rs;
pub mod ver_test;

/// Convert an array of string pointers into a vector of strings, skipping a given number of
/// elements at the start.
///
/// Note that this automatically frees the memory used for argv when it goes out of scope so the C
/// caller shouldn't try to free it.
///
/// # Safety
/// Behavior is undefined if argv is not a pointer to a length argc array of strings containing
/// valid UTF-8.
unsafe fn args_to_vec(argc: c_int, argv: &*mut *mut c_char, skip: usize) -> Vec<&str> {
    let args_len: usize = argc.try_into().unwrap();
    let args: Vec<&str> = unsafe { slice::from_raw_parts(*argv, args_len) }
        .iter()
        .skip(skip)
        .map(|s| unsafe { CStr::from_ptr(*s).to_str().unwrap() })
        .collect();
    args
}
