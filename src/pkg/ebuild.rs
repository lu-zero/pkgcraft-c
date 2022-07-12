use std::ffi::CString;
use std::os::raw::c_char;
use std::{mem, ptr};

pub use pkgcraft::pkg::ebuild::Pkg as EbuildPkg;

use crate::macros::*;

/// Return a package's ebuild file content.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_ebuild(p: *mut EbuildPkg) -> *mut c_char {
    let pkg = null_ptr_check!(p.as_ref());
    let s = unwrap_or_return!(pkg.ebuild(), ptr::null_mut());
    let cstring = unwrap_or_return!(CString::new(s), ptr::null_mut());
    cstring.into_raw()
}

/// Return a package's description.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_description(p: *mut EbuildPkg) -> *mut c_char {
    let pkg = null_ptr_check!(p.as_ref());
    CString::new(pkg.description()).unwrap().into_raw()
}

/// Return a package's slot.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_slot(p: *mut EbuildPkg) -> *mut c_char {
    let pkg = null_ptr_check!(p.as_ref());
    CString::new(pkg.slot()).unwrap().into_raw()
}

/// Return a package's homepage.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_homepage(
    p: *mut EbuildPkg,
    len: *mut usize,
) -> *mut *mut c_char {
    let pkg = null_ptr_check!(p.as_ref());
    let mut ptrs: Vec<_> = pkg
        .homepage()
        .iter()
        .map(|&s| CString::new(s).unwrap().into_raw())
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let ptr = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    ptr
}

/// Return a package's keywords.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_keywords(
    p: *mut EbuildPkg,
    len: *mut usize,
) -> *mut *mut c_char {
    let pkg = null_ptr_check!(p.as_ref());
    let mut ptrs: Vec<_> = pkg
        .keywords()
        .iter()
        .map(|&s| CString::new(s).unwrap().into_raw())
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let ptr = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    ptr
}

/// Return a package's iuse.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_iuse(
    p: *mut EbuildPkg,
    len: *mut usize,
) -> *mut *mut c_char {
    let pkg = null_ptr_check!(p.as_ref());
    let mut ptrs: Vec<_> = pkg
        .iuse()
        .iter()
        .map(|&s| CString::new(s).unwrap().into_raw())
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let ptr = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    ptr
}
