use std::env;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice;

use crate::{Error, Result};

pub mod ver_cut;
pub mod ver_rs;
pub mod ver_test;

/// Convert an array of string pointers into Vec<&str>.
///
/// # Safety
/// Behavior is undefined if args is not a pointer to a length args_len array of
/// valid UTF-8 strings.
unsafe fn args_to_vec(args: &*mut *mut c_char, args_len: c_int) -> Vec<&str> {
    let args_len: usize = args_len.try_into().unwrap();
    let args: Vec<&str> = unsafe { slice::from_raw_parts(*args, args_len) }
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
