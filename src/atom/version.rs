use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use pkgcraft::{atom, utils::hash};

use crate::macros::*;

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Version objects.
pub struct Version;

/// Parse a string into a version.
///
/// Returns NULL on error.
///
/// # Safety
/// The version argument should point to a valid string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version(version: *const c_char) -> *mut atom::Version {
    let ver_str = unsafe { unwrap_or_return!(CStr::from_ptr(version).to_str(), ptr::null_mut()) };
    let ver = unwrap_or_return!(atom::Version::new(ver_str), ptr::null_mut());
    Box::into_raw(Box::new(ver))
}

/// Parse a string into a version with an operator.
///
/// Returns NULL on error.
///
/// # Safety
/// The version argument should point to a valid string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_with_op(version: *const c_char) -> *mut atom::Version {
    let ver_str = unsafe { unwrap_or_return!(CStr::from_ptr(version).to_str(), ptr::null_mut()) };
    let ver = unwrap_or_return!(atom::Version::new_with_op(ver_str), ptr::null_mut());
    Box::into_raw(Box::new(ver))
}

/// Compare two versions returning -1, 0, or 1 if the first version is less than, equal to, or greater
/// than the second version, respectively.
///
/// # Safety
/// The version arguments should be non-null Version pointers received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_cmp(
    v1: *mut atom::Version,
    v2: *mut atom::Version,
) -> c_int {
    let v1 = null_ptr_check!(v1.as_ref());
    let v2 = null_ptr_check!(v2.as_ref());

    match v1.cmp(v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return a given version's revision, e.g. the version "1-r2" has a revision of "2".
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_revision(version: *mut atom::Version) -> *mut c_char {
    let version = null_ptr_check!(version.as_ref());
    let s = version.revision().as_str();
    CString::new(s).unwrap().into_raw()
}

/// Return the string for a given version.
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_str(version: *mut atom::Version) -> *mut c_char {
    let version = null_ptr_check!(version.as_ref());
    CString::new(version.as_str()).unwrap().into_raw()
}

/// Free a version.
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_free(version: *mut atom::Version) {
    if !version.is_null() {
        let _ = unsafe { Box::from_raw(version) };
    }
}

/// Return the hash value for a given version.
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_hash(version: *mut atom::Version) -> u64 {
    let version = null_ptr_check!(version.as_ref());
    hash(version)
}
