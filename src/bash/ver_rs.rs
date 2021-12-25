use std::os::raw::{c_char, c_int};

use itertools::Itertools;
use pkgcraft::bash::{parse, version_split};

use super::{args_to_vec, get_env};
use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

/// Perform string substitution on package version strings.
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
pub unsafe extern "C" fn ver_rs(argc: c_int, argv: &*mut *mut c_char) -> c_int {
    let mut args = unsafe { args_to_vec(argc, argv, 1) };
    let ver = match args.len() {
        n if n < 2 => {
            let err = Error::new(format!("requires 2 or more args, got {}", n));
            update_last_error(err);
            return -1;
        }

        // even number of args pulls the version from PV in the environment
        n if n % 2 == 0 => unwrap_or_return!(get_env("PV"), -1),

        // odd number of args uses the last arg as the version
        _ => args.pop().unwrap().to_string(),
    };

    // Split version string into separators and components, note that the version string doesn't
    // have to follow the spec since args like ".1.2.3" are allowed.
    let mut version_parts = version_split(&ver);

    // iterate over (range, separator) pairs
    let mut args_iter = args.iter();
    while let Some((range, sep)) = args_iter.next_tuple() {
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
