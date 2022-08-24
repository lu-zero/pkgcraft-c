use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;
use std::{mem, ptr};

use pkgcraft::eapi;

use crate::macros::*;

/// Opaque wrapper for Eapi objects.
pub struct Eapi;

/// Get all known EAPIS.
///
/// # Safety
/// The returned array must be freed via pkgcraft_eapis_free().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_eapis(len: *mut usize) -> *mut *const eapi::Eapi {
    let mut ptrs: Vec<_> = eapi::EAPIS
        .values()
        .copied()
        .map(|e| e as *const _)
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let p = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    p
}

/// Get all official EAPIS.
///
/// # Safety
/// The returned array must be freed via pkgcraft_eapis_free().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_eapis_official(len: *mut usize) -> *mut *const eapi::Eapi {
    let mut ptrs: Vec<_> = eapi::EAPIS_OFFICIAL
        .values()
        .copied()
        .map(|e| e as *const _)
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let p = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    p
}

/// Free an array of borrowed Eapi pointers.
///
/// # Safety
/// The argument must be the value received from pkgcraft_eapis(), pkgcraft_eapis_official(), or
/// NULL along with the length of the array.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_eapis_free(eapis: *mut *const eapi::Eapi, len: usize) {
    if !eapis.is_null() {
        unsafe { Vec::from_raw_parts(eapis, len, len) };
    }
}

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

/// Return an EAPI's identifier.
///
/// # Safety
/// The arguments must be a non-null Eapi pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_eapi_as_str(eapi: *const eapi::Eapi) -> *mut c_char {
    let eapi = null_ptr_check!(eapi.as_ref());
    CString::new(eapi.as_str()).unwrap().into_raw()
}
