use std::os::raw::{c_char, c_int};
use std::str::FromStr;

use pkgcraft::atom::Version;

use super::{args_to_vec, get_env};
use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

/// Perform version testing as defined in the spec.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Operates on argc and argv passed directly from C.
///
/// Returns 0 if the specified test is true, 1 otherwise.
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if argv is not a pointer to a length argc array of strings containing
/// valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn ver_test(argc: c_int, argv: &*mut *mut c_char) -> c_int {
    let args = unsafe { &args_to_vec(argc, argv)[1..] };
    let (lhs, op, rhs) = match args.len() {
        2 => {
            let pvr = unwrap_or_return!(get_env("PVR"), -1);
            (pvr, args[0].to_string(), args[1].to_string())
        }
        3 => (
            args[0].to_string(),
            args[1].to_string(),
            args[2].to_string(),
        ),
        n => {
            let err = Error::new(format!("only accepts 2 or 3 args, got {}", n));
            update_last_error(err);
            return -1;
        }
    };

    let ver_lhs = unwrap_or_return!(Version::from_str(&lhs), -1);
    let ver_rhs = unwrap_or_return!(Version::from_str(&rhs), -1);

    let ret = match op.as_ref() {
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
