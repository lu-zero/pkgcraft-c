use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::str::FromStr;

use pkgcraft::atom::Version;

use super::args_to_vec;
use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

/// Perform version testing as defined in the spec.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Returns 0 if the specified test is true, 1 otherwise.
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if args is not a pointer to a length args_len array of
/// valid UTF-8 strings.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ver_test(
    args: &*mut *mut c_char,
    args_len: c_int,
    pvr_ptr: &*mut c_char,
) -> c_int {
    let args = unsafe { args_to_vec(args, args_len) };
    let pvr = match pvr_ptr.is_null() {
        true => "",
        false => unsafe { CStr::from_ptr(*pvr_ptr).to_str().unwrap() },
    };

    let (lhs, op, rhs) = match args.len() {
        2 => {
            if pvr.is_empty() {
                let err = Error::new("$PVR is undefined");
                update_last_error(err);
                return -1;
            }
            (pvr, args[0], args[1])
        }
        3 => (args[0], args[1], args[2]),
        n => {
            let err = Error::new(format!("only accepts 2 or 3 args, got {}", n));
            update_last_error(err);
            return -1;
        }
    };

    let ver_lhs = unwrap_or_return!(Version::from_str(lhs), -1);
    let ver_rhs = unwrap_or_return!(Version::from_str(rhs), -1);

    let ret = match op {
        "-eq" => ver_lhs == ver_rhs,
        "-ne" => ver_lhs != ver_rhs,
        "-lt" => ver_lhs < ver_rhs,
        "-gt" => ver_lhs > ver_rhs,
        "-le" => ver_lhs <= ver_rhs,
        "-ge" => ver_lhs >= ver_rhs,
        _ => {
            let err = Error::new(format!("invalid operator: {:?}", op));
            update_last_error(err);
            return -1;
        }
    };

    !ret as c_int
}
