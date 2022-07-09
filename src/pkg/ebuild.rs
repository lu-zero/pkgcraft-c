use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

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
