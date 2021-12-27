use std::os::raw::{c_char, c_int};

use super::args_to_vec;
use crate::error::update_last_error;
use crate::Error;

/// Returns 0 if the first argument is found in the list of subsequent arguments, 1 otherwise.
///
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if args is not a pointer to a length args_len array of
/// valid UTF-8 strings.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_has(args: &*mut *mut c_char, args_len: c_int) -> c_int {
    let args = unsafe { args_to_vec(args, args_len) };

    let needle = match args.first() {
        Some(s) => s,
        None => {
            let err = Error::new("requires 1 or more args");
            update_last_error(err);
            return -1;
        }
    };

    let haystack = &args[1..];
    !haystack.contains(needle) as c_int
}

/// Returns 0 if the first argument is found in the list of subsequent arguments, 1 otherwise.
///
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if args is not a pointer to a length args_len array of
/// valid UTF-8 strings.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_hasv(args: &*mut *mut c_char, args_len: c_int) -> c_int {
    let args = unsafe { args_to_vec(args, args_len) };

    let needle = match args.first() {
        Some(s) => s,
        None => {
            let err = Error::new("requires 1 or more args");
            update_last_error(err);
            return -1;
        }
    };

    let haystack = &args[1..];
    if haystack.contains(needle) {
        println!("{}", needle);
        0
    } else {
        1
    }
}
