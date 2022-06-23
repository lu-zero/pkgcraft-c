use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::{self, NonNull};

use pkgcraft::{atom, eapi};

use crate::macros::unwrap_or_return;

/// Parse an atom string.
///
/// Returns NULL on error.
///
/// # Safety
/// The atom argument should be a UTF-8 string while eapi can be a string or may be
/// NULL to use the default EAPI.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_atom(
    atom: NonNull<c_char>,
    eapi: *const c_char,
) -> *mut c_char {
    let atom = atom.as_ptr();
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(atom).to_str(), ptr::null_mut()) };
    let eapi = unwrap_or_return!(eapi::IntoEapi::into_eapi(eapi), ptr::null_mut());
    unwrap_or_return!(atom::Atom::valid(s, eapi), ptr::null_mut());
    atom
}

/// Parse an atom category string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_category(s: *const c_char) -> *const c_char {
    let val = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::parse::category(val), ptr::null_mut());
    s
}

/// Parse an atom package string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_package(s: *const c_char) -> *const c_char {
    let val = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::parse::package(val), ptr::null_mut());
    s
}

/// Parse an atom version string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_version(s: *const c_char) -> *const c_char {
    let val = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::Version::valid(val), ptr::null_mut());
    s
}

/// Parse an atom repo string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_repo(s: *const c_char) -> *const c_char {
    let val = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::parse::repo(val), ptr::null_mut());
    s
}

/// Parse an atom cpv string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_cpv(s: *const c_char) -> *const c_char {
    let val = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::Atom::valid_cpv(val), ptr::null_mut());
    s
}
