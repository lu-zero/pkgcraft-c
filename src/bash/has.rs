use std::os::raw::{c_char, c_int};

use super::args_to_vec;
use crate::error::update_last_error;
use crate::Error;

/// Returns 0 if the first argument is found in the list of subsequent arguments, 1 otherwise.
///
/// Operates on argc and argv passed directly from C.
///
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if argv is not a pointer to a length argc array of strings containing
/// valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn has(argc: c_int, argv: &*mut *mut c_char) -> c_int {
    let args = unsafe { &args_to_vec(argc, argv)[1..] };

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
