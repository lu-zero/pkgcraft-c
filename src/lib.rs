#![allow(unreachable_pub)]

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::str::FromStr;
use std::{cmp, env, fmt, mem, ptr};

use itertools::Itertools;
use once_cell::sync::Lazy;
use pkgcraft::{atom, eapi, Error as PkgcraftError};
use regex::Regex;
use tracing::{error, warn};

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    fn new<S: Into<String>>(msg: S) -> Error {
        Error {
            message: msg.into(),
        }
    }
}

impl From<PkgcraftError> for Error {
    fn from(e: PkgcraftError) -> Self {
        Error::new(e.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

#[repr(C)]
pub struct Atom {
    string: *const c_char,
    eapi: *const c_char,
    category: *const c_char,
    package: *const c_char,
    version: *const c_char,
    slot: *const c_char,
    subslot: *const c_char,
    use_deps: *const *const c_char,
    // TODO: switch to c_size_t once it's non-experimental
    // https://doc.rust-lang.org/std/os/raw/type.c_size_t.html
    use_deps_len: usize,
    repo: *const c_char,
}

/// Parse a string into an atom using a specific EAPI. Pass a null pointer for the eapi argument in
/// order to parse using the latest EAPI with extensions (e.g. support for repo deps).
#[no_mangle]
pub extern "C" fn str_to_atom(atom: *const c_char, eapi: *const c_char) -> *mut Atom {
    if atom.is_null() {
        let err = Error::new("no atom string provided");
        update_last_error(err);
        return ptr::null_mut();
    }

    let atom_str = match unsafe { CStr::from_ptr(atom).to_str() } {
        Ok(s) => s,
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    let eapi = match eapi.is_null() {
        true => &eapi::EAPI_PKGCRAFT,
        false => match unsafe { CStr::from_ptr(eapi).to_str() } {
            Ok(s) => match eapi::get_eapi(s) {
                Ok(eapi) => eapi,
                Err(e) => {
                    update_last_error(e);
                    return ptr::null_mut();
                }
            },
            Err(e) => {
                update_last_error(e);
                return ptr::null_mut();
            }
        },
    };

    let atom = match atom::parse::dep(atom_str, eapi) {
        Ok(a) => a,
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    // parsing should catch errors so no need to check here
    let string = CString::new(atom_str).unwrap().into_raw();
    let eapi = CString::new(eapi.to_string()).unwrap().into_raw();
    let category = CString::new(atom.category).unwrap().into_raw();
    let package = CString::new(atom.package).unwrap().into_raw();

    let version = match atom.version {
        Some(s) => CString::new(format!("{}", s)).unwrap().into_raw(),
        None => ptr::null(),
    };

    let slot = match atom.slot {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => ptr::null(),
    };

    let subslot = match atom.subslot {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => ptr::null(),
    };

    let mut use_strs = vec![];
    if let Some(use_deps) = atom.use_deps {
        for u in use_deps.iter() {
            use_strs.push(CString::new(u.as_str()).unwrap().into_raw())
        }
    }
    let use_deps_len = use_strs.len();
    // TODO: switch to into_raw_parts() once it's non-experimental
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts
    let use_deps = Box::into_raw(use_strs.into_boxed_slice()).cast();

    let repo = match atom.repo {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => ptr::null(),
    };

    // create C-compatible struct
    let c_atom = Atom {
        string,
        eapi,
        category,
        package,
        version,
        slot,
        subslot,
        use_deps,
        use_deps_len,
        repo,
    };

    Box::into_raw(Box::new(c_atom))
}

/// Convert a C-compatible Atom struct to a rust Atom struct.
pub fn atom_to_rust(atom: *mut Atom) -> Result<atom::Atom, Error> {
    if atom.is_null() {
        return Err(Error::new("no atom provided"));
    }

    let atom = unsafe { Box::from_raw(atom) };
    let atom_str = unsafe { CStr::from_ptr(atom.string) }
        .to_str()
        .map_err(|e| Error {
            message: format!("invalid atom string: {:?}", e),
        })?;

    let eapi = match atom.eapi.is_null() {
        true => &eapi::EAPI_PKGCRAFT,
        false => {
            let eapi_str = unsafe { CStr::from_ptr(atom.eapi) }
                .to_str()
                .map_err(|e| Error {
                    message: format!("invalid eapi string: {:?}", e),
                })?;
            eapi::get_eapi(eapi_str)?
        }
    };

    // don't deallocate memory when `atom` is dropped
    mem::forget(atom);

    atom::parse::dep(atom_str, eapi).map_err(|e| Error {
        message: e.to_string(),
    })
}

/// Return a given atom's key, e.g. the atom "=cat/pkg-1-r2" has a key of "cat/pkg".
/// Returns a null pointer on error.
#[no_mangle]
pub extern "C" fn atom_key(atom: *mut Atom) -> *const c_char {
    let key = match atom_to_rust(atom) {
        Ok(a) => a.key(),
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    CString::new(key).unwrap().into_raw()
}

/// Return a given atom's cpv, e.g. the atom "=cat/pkg-1-r2" has a cpv of "cat/pkg-1-r2".
/// Returns a null pointer on error.
#[no_mangle]
pub extern "C" fn atom_cpv(atom: *mut Atom) -> *const c_char {
    let cpv = match atom_to_rust(atom) {
        Ok(a) => a.cpv(),
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    CString::new(cpv).unwrap().into_raw()
}

/// Free an atom.
#[no_mangle]
pub unsafe extern "C" fn atom_free(atom: *mut Atom) {
    if atom.is_null() {
        return;
    }

    let a = Box::from_raw(atom);
    drop(CString::from_raw(a.string as *mut _));
    drop(CString::from_raw(a.eapi as *mut _));
    drop(CString::from_raw(a.category as *mut _));
    drop(CString::from_raw(a.package as *mut _));
    if !a.version.is_null() {
        drop(CString::from_raw(a.version as *mut _));
    }
    if !a.slot.is_null() {
        drop(CString::from_raw(a.slot as *mut _));
    }
    if !a.subslot.is_null() {
        drop(CString::from_raw(a.subslot as *mut _));
    }
    if !a.use_deps.is_null() {
        let use_deps = Vec::from_raw_parts(a.use_deps as *mut _, a.use_deps_len, a.use_deps_len);
        for &u in use_deps.iter() {
            drop(CString::from_raw(u));
        }
    }
    if !a.repo.is_null() {
        drop(CString::from_raw(a.repo as *mut _));
    }
}

/// Unwrap the returned value of a given expression or return the given value.
macro_rules! unwrap_or_return {
    ( $e:expr, $v:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                update_last_error(e);
                return $v;
            }
        }
    };
}

/// Convert an array of string pointers into a vector of strings, skipping a given number of
/// elements at the start.
///
/// Note that this automatically frees the memory used for argv when it goes out of scope so the C
/// caller shouldn't try to free it.
pub fn args_to_vec<'a>(argc: c_int, argv: *mut *mut c_char, skip: usize) -> Vec<&'a str> {
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

    let ver_lhs = unwrap_or_return!(atom::Version::from_str(&lhs), -1);
    let ver_rhs = unwrap_or_return!(atom::Version::from_str(&rhs), -1);

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

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<dyn std::error::Error>>> = RefCell::new(None);
}

/// Update the most recent error, clearing the previous value.
pub fn update_last_error<E: std::error::Error + 'static>(err: E) {
    error!("Setting LAST_ERROR: {}", err);

    {
        // Print a pseudo-backtrace for this error, following back each error's
        // source until we reach the root error.
        let mut source = err.source();
        while let Some(parent_err) = source {
            warn!("Caused by: {}", parent_err);
            source = parent_err.source();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Get the most recent error message as a UTF-8 string, if none exists a null pointer is returned.
///
/// The caller is expected to free memory used by the string after they're finished using it.
#[no_mangle]
pub extern "C" fn last_error_message() -> *mut c_char {
    // Retrieve the most recent error, clearing it in the process.
    let last_error: Option<Box<dyn std::error::Error>> =
        LAST_ERROR.with(|prev| prev.borrow_mut().take());
    match last_error {
        Some(e) => CString::new(e.to_string()).unwrap().into_raw(),
        None => ptr::null_mut(),
    }
}
