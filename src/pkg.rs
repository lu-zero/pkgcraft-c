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
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_atom(p: NonNull<pkg::Pkg>) -> *const atom::Atom {
    let pkg = unsafe { p.as_ref() };
    pkg.atom()
}

/// Compare two packages returning -1, 0, or 1 if the first package is less than, equal to, or
/// greater than the second package, respectively.
///
/// # Safety
/// The arguments must be non-null Pkg pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_cmp<'a>(
    p1: NonNull<pkg::Pkg<'a>>,
    p2: NonNull<pkg::Pkg<'a>>,
) -> c_int {
    let (pkg1, pkg2) = unsafe { (p1.as_ref(), p2.as_ref()) };

    match pkg1.cmp(pkg2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return the hash value for a given package.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_hash(p: NonNull<pkg::Pkg>) -> u64 {
    let pkg = unsafe { p.as_ref() };
    hash(pkg)
}

/// Free an package.
///
/// # Safety
/// The argument must be a non-null Pkg pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_free(p: *mut pkg::Pkg) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(p)) };
    }
}
