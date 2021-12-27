use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use pkgcraft::bash::{parse, version_split};

use super::args_to_vec;
use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

/// Perform string substitution on package version strings.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if args is not a pointer to a length args_len array of
/// valid UTF-8 strings.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ver_rs(
    args: &*mut *mut c_char,
    args_len: c_int,
    pv_ptr: &*mut c_char,
) -> c_int {
    let mut args = unsafe { args_to_vec(args, args_len) };
    let pv = match pv_ptr.is_null() {
        true => "",
        false => unsafe { CStr::from_ptr(*pv_ptr).to_str().unwrap() },
    };

    let ver = match args.len() {
        n if n < 2 => {
            let err = Error::new(format!("requires 2 or more args, got {}", n));
            update_last_error(err);
            return -1;
        }

        // even number of args uses $PV
        n if n % 2 == 0 => pv,

        // odd number of args uses the last arg as the version
        _ => args.pop().unwrap(),
    };

    // Split version string into separators and components, note that the version string doesn't
    // have to follow the spec since args like ".1.2.3" are allowed.
    let mut version_parts = version_split(ver);

    // iterate over (range, separator) pairs
    let mut args_iter = args.chunks_exact(2);
    while let Some(&[range, sep]) = args_iter.next() {
        let (start, end) = unwrap_or_return!(parse::range(range, version_parts.len() / 2), -1);
        for n in start..=end {
            let idx = n * 2;
            if idx < version_parts.len() {
                version_parts[idx] = sep;
            }
        }
    }

    println!("{}", version_parts.join(""));

    0
}
