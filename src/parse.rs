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
/// The argument should point to a valid UTF-8 string.
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
/// The argument should point to a valid UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_category(cstr: *const c_char) -> *const c_char {
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(cstr).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::parse::category(s), ptr::null_mut());
    cstr
}

/// Parse an atom package string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a valid UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_package(cstr: *const c_char) -> *const c_char {
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(cstr).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::parse::package(s), ptr::null_mut());
    cstr
}

/// Parse an atom version string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a valid UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_version(cstr: *const c_char) -> *const c_char {
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(cstr).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::Version::valid(s), ptr::null_mut());
    cstr
}

/// Parse an atom repo string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a valid UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_repo(cstr: *const c_char) -> *const c_char {
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(cstr).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::parse::repo(s), ptr::null_mut());
    cstr
}

/// Parse an atom cpv string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should point to a valid UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_parse_cpv(cstr: *const c_char) -> *const c_char {
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(cstr).to_str(), ptr::null_mut()) };
    unwrap_or_return!(atom::Atom::valid_cpv(s), ptr::null_mut());
    cstr
}
