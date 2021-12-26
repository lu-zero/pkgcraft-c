use std::env;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice;

use crate::{Error, Result};

pub mod ver_cut;
pub mod ver_rs;
pub mod ver_test;

/// Convert an array of string pointers into a vector of &str.
///
/// # Safety
/// Behavior is undefined if argv is not a pointer to a length argc array of strings containing
/// valid UTF-8.
unsafe fn args_to_vec(argc: c_int, argv: &*mut *mut c_char) -> Vec<&str> {
    let args_len: usize = argc.try_into().unwrap();
    let args: Vec<&str> = unsafe { slice::from_raw_parts(*argv, args_len) }
        .iter()
        .map(|s| unsafe { CStr::from_ptr(*s).to_str().unwrap() })
        .collect();
    args
}

/// Get the value of a given environment variable.
///
/// Returns an error when missing or invalid.
pub fn get_env(var: &str) -> Result<String> {
    match env::var(var) {
        Ok(v) => Ok(v),
        // variable is invalid or missing from the environment
        Err(e) => Err(Error::new(format!("{}: {:?}", e, var))),
    }
}
