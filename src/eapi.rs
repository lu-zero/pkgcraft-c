use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;

use pkgcraft::eapi;

use crate::macros::*;

/// Opaque wrapper for Eapi objects.
pub struct Eapi;

/// Get an EAPI given its identifier.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument must be a non-null string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_get_eapi(s: *const c_char) -> *const eapi::Eapi {
    let s = null_ptr_check!(s.as_ref());
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null()) };
    unwrap_or_return!(eapi::get_eapi(s), ptr::null())
}

/// Check if an EAPI has a given feature.
///
/// # Safety
/// The arguments must be a non-null Eapi pointer and non-null string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_eapi_has(eapi: *const eapi::Eapi, s: *const c_char) -> bool {
    let eapi = null_ptr_check!(eapi.as_ref());
    let s = null_ptr_check!(s.as_ref());
    let s = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), false) };
    let feature = unwrap_or_return!(eapi::Feature::from_str(s), false);
    eapi.has(feature)
}
