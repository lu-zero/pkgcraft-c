use std::cmp::Ordering;
use std::os::raw::c_int;
use std::ptr::NonNull;

use pkgcraft::pkg::Package;
use pkgcraft::{atom, pkg, utils::hash};

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

/// Compare two packages returning -1, 0, or 1 if the first package is less than, equal to, or
/// greater than the second package, respectively.
///
/// # Safety
/// The ptr arguments should be non-null Pkg pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_cmp<'a>(
    ptr1: NonNull<pkg::Pkg<'a>>,
    ptr2: NonNull<pkg::Pkg<'a>>,
) -> c_int {
    let (obj1, obj2) = unsafe { (ptr1.as_ref(), ptr2.as_ref()) };

    match obj1.cmp(obj2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return the hash value for a given package.
///
/// # Safety
/// The ptr argument should be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_hash(ptr: NonNull<pkg::Pkg>) -> u64 {
    let pkg = unsafe { ptr.as_ref() };
    hash(pkg)
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
