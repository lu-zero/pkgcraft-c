use std::cmp;
use std::os::raw::{c_char, c_int};

use pkgcraft::bash::{parse, version_split};

use super::{args_to_vec, get_env};
use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

/// Output substring from package version string and range arguments.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Operates on argc and argv passed directly from C and handles freeing argv.
///
/// Returns -1 if an error occurred.
///
/// # Safety
/// Behavior is undefined if argv is not a pointer to a length argc array of strings containing
/// valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn ver_cut(argc: c_int, argv: &*mut *mut c_char) -> c_int {
    let args = unsafe { &args_to_vec(argc, argv)[1..] };
    let (range, ver) = match args.len() {
        1 => {
            let pv = unwrap_or_return!(get_env("PV"), -1);
            (args[0].to_string(), pv)
        }
        2 => (args[0].to_string(), args[1].to_string()),
        n => {
            let err = Error::new(format!("requires 1 or 2 args, got {}", n));
            update_last_error(err);
            return -1;
        }
    };

    let version_parts = version_split(&ver);
    let max_idx = version_parts.len();
    let (start, end) = unwrap_or_return!(parse::range(&range, version_parts.len() / 2), -1);
    let start_idx = match start {
        0 => 0,
        n => cmp::min(n * 2 - 1, max_idx),
    };
    let end_idx = cmp::min(end * 2, max_idx);
    println!("{}", &version_parts[start_idx..end_idx].join(""));

    0
}
