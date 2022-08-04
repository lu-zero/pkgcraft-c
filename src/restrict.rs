use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use pkgcraft::restrict;

use crate::macros::*;

/// Opaque wrapper for Restrict objects.
pub struct Restrict;

/// Parse a restriction string.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument must be a non-null restriction string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_restrict_parse_dep(s: *const c_char) -> *mut restrict::Restrict {
    let s = null_ptr_check!(s.as_ref());
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    let restrict = unwrap_or_return!(restrict::parse::dep(s), ptr::null_mut());
    Box::into_raw(Box::new(restrict))
}

/// Free a restriction.
///
/// # Safety
/// The argument must be a Restrict pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_restrict_free(r: *mut restrict::Restrict) {
    if !r.is_null() {
        unsafe { drop(Box::from_raw(r)) };
    }
}
