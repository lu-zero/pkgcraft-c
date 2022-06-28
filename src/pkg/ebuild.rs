use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::NonNull;

pub use pkgcraft::pkg::ebuild::Pkg as EbuildPkg;

/// Return a given ebuild's DESCRIPTION.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_description(p: NonNull<EbuildPkg>) -> *mut c_char {
    let pkg = unsafe { p.as_ref() };
    CString::new(pkg.description()).unwrap().into_raw()
}

/// Return a given ebuild's slot.
///
/// # Safety
/// The argument must be a non-null EbuildPkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_pkg_slot(p: NonNull<EbuildPkg>) -> *mut c_char {
    let pkg = unsafe { p.as_ref() };
    CString::new(pkg.slot()).unwrap().into_raw()
}
