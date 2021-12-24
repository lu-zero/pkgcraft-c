use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice;

use once_cell::sync::Lazy;
use regex::Regex;

pub mod ver_cut;
pub mod ver_test;
pub mod ver_rs;

/// Convert an array of string pointers into a vector of strings, skipping a given number of
/// elements at the start.
///
/// Note that this automatically frees the memory used for argv when it goes out of scope so the C
/// caller shouldn't try to free it.
///
/// # Safety
/// Behavior is undefined if argv is not a pointer to a length argc array of strings containing
/// valid UTF-8.
unsafe fn args_to_vec(argc: c_int, argv: &*mut *mut c_char, skip: usize) -> Vec<&str> {
    let args_len: usize = argc.try_into().unwrap();
    let args: Vec<&str> = unsafe { slice::from_raw_parts(*argv, args_len) }
        .iter()
        .skip(skip)
        .map(|s| unsafe { CStr::from_ptr(*s).to_str().unwrap() })
        .collect();
    args
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
