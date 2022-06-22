use std::ptr::NonNull;

use pkgcraft::pkg::Package;
use pkgcraft::{atom, pkg};

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Pkg objects.
pub struct Pkg;

/// Return a given package's atom.
///
/// # Safety
/// The ptr argument should be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_atom(ptr: NonNull<pkg::Pkg>) -> *const atom::Atom {
    let pkg = unsafe { ptr.as_ref() };
    pkg.atom()
}

/// Free an package.
///
/// # Safety
/// The ptr argument should be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_free(ptr: *mut pkg::Pkg) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}
