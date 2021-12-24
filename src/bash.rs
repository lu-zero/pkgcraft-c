use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::str::FromStr;
use std::{cmp, env};

use itertools::Itertools;
use once_cell::sync::Lazy;
use pkgcraft::atom::Version;
use regex::Regex;

use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

/// Convert an array of string pointers into a vector of strings, skipping a given number of
/// elements at the start.
///
/// Note that this automatically frees the memory used for argv when it goes out of scope so the C
/// caller shouldn't try to free it.
fn args_to_vec<'a>(argc: c_int, argv: *mut *mut c_char, skip: usize) -> Vec<&'a str> {
    let args_len: usize = argc.try_into().unwrap();
    let cargs = unsafe { Vec::from_raw_parts(argv, args_len, args_len) };
    let args: Vec<&str> = cargs
        .iter()
        .skip(skip)
        .map(|s| unsafe { CStr::from_ptr(*s).to_str().unwrap() })
        .collect();
    args
}

/// Perform version testing as defined in the spec.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Operates on argc and argv passed directly from C and handles freeing argv.
///
/// Returns 0 if the specified test is true, 1 otherwise.
/// Returns -1 if an error occurred.
#[no_mangle]
pub extern "C" fn ver_test(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // skip the initial program name in argv[0]
    let args = args_to_vec(argc, argv, 1);
    let (lhs, op, rhs) = match args.len() {
        2 => {
            let varname = "PVR";
            let pvr = match env::var(varname) {
                Ok(v) => v,
                Err(e) => {
                    // PVR variable is invalid or missing from the environment
                    let err = Error::new(format!("{}: {:?}", e, varname));
                    update_last_error(err);
                    return -1;
                }
            };
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

peg::parser! {
    pub grammar cmd() for str {
        // Parse ranges used with the ver_rs and ver_cut commands.
        pub rule range(max: usize) -> (usize, usize)
            = start_s:$(['0'..='9']+) "-" end_s:$(['0'..='9']+) {
                let start = start_s.parse::<usize>().unwrap();
                let end = end_s.parse::<usize>().unwrap();
                (start, end)
            } / start_s:$(['0'..='9']+) "-" {
                match start_s.parse::<usize>().unwrap() {
                    start if start <= max => (start, max),
                    start => (start, start),
                }
            } / start_s:$(['0'..='9']+) {
                let start = start_s.parse::<usize>().unwrap();
                (start, start)
            }
    }
}

// provide public parsing functionality while converting error types
pub mod parse {
    use pkgcraft::peg::peg_error;

    use super::cmd;

    #[inline]
    pub fn range(s: &str, max: usize) -> pkgcraft::Result<(usize, usize)> {
        let (start, end) =
            cmd::range(s, max).map_err(|e| peg_error(format!("invalid range: {:?}", s), s, e))?;
        if end < start {
            return Err(pkgcraft::Error::InvalidValue(format!(
                "start of range ({}) is greater than end ({})",
                start, end
            )));
        }
        Ok((start, end))
    }
}

static VERSION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<sep>[[:^alnum:]]+)?(?P<comp>[[:digit:]]+|[[:alpha:]]+)?").unwrap()
});

/// Split version string into ordered vector of separators and components.
fn version_split(ver: &str) -> Vec<&str> {
    let mut version_parts = Vec::new();
    for caps in VERSION_RE.captures_iter(ver) {
        let sep = caps.name("sep").map_or("", |m| m.as_str());
        let comp = caps.name("comp").map_or("", |m| m.as_str());
        version_parts.extend([sep, comp]);
    }
    version_parts
}

/// Perform string substitution on package version strings.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Operates on argc and argv passed directly from C and handles freeing argv.
///
/// Returns -1 if an error occurred.
#[no_mangle]
pub extern "C" fn ver_rs(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // skip the initial program name in argv[0]
    let mut args = args_to_vec(argc, argv, 1);
    let ver = match args.len() {
        n if n < 2 => {
            let err = Error::new(format!("requires 2 or more args, got {}", n));
            update_last_error(err);
            return -1;
        }

        // even number of args pulls the version from PV in the environment
        n if n % 2 == 0 => {
            let varname = "PV";
            let pv = match env::var(varname) {
                Ok(v) => v,
                Err(e) => {
                    // PV variable is invalid or missing from the environment
                    let err = Error::new(format!("{}: {:?}", e, varname));
                    update_last_error(err);
                    return -1;
                }
            };
            pv
        }

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

/// Output substring from package version string and range arguments.
/// https://projects.gentoo.org/pms/latest/pms.html#x1-13400012.3.14
///
/// Operates on argc and argv passed directly from C and handles freeing argv.
///
/// Returns -1 if an error occurred.
#[no_mangle]
pub extern "C" fn ver_cut(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // skip the initial program name in argv[0]
    let args = args_to_vec(argc, argv, 1);
    let (range, ver) = match args.len() {
        1 => {
            let varname = "PV";
            let pv = match env::var(varname) {
                Ok(v) => v,
                Err(e) => {
                    // PV variable is invalid or missing from the environment
                    let err = Error::new(format!("{}: {:?}", e, varname));
                    update_last_error(err);
                    return -1;
                }
            };
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
