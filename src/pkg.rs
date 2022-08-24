use std::cmp::Ordering;
use std::os::raw::c_int;
use std::ptr;

use pkgcraft::pkg::Package;
use pkgcraft::{atom, eapi, pkg, repo, restrict, utils::hash, Error};

use crate::macros::*;

pub mod ebuild;

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Pkg objects.
pub struct Pkg;

/// Return a given package's atom.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_atom(p: *mut pkg::Pkg) -> *const atom::Atom {
    let pkg = null_ptr_check!(p.as_ref());
    pkg.atom()
}

/// Return a given package's repo.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_repo(p: *mut pkg::Pkg) -> *const repo::Repo {
    let pkg = null_ptr_check!(p.as_ref());
    pkg.repo()
}

/// Return a given package's EAPI.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_eapi(p: *mut pkg::Pkg) -> *const eapi::Eapi {
    let pkg = null_ptr_check!(p.as_ref());
    pkg.eapi()
}

/// Return a given package's version.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_version(p: *mut pkg::Pkg) -> *const atom::Version {
    let pkg = null_ptr_check!(p.as_ref());
    pkg.version()
}

/// Compare two packages returning -1, 0, or 1 if the first package is less than, equal to, or
/// greater than the second package, respectively.
///
/// # Safety
/// The arguments must be non-null Pkg pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_cmp<'a>(
    p1: *mut pkg::Pkg<'a>,
    p2: *mut pkg::Pkg<'a>,
) -> c_int {
    let pkg1 = null_ptr_check!(p1.as_ref());
    let pkg2 = null_ptr_check!(p2.as_ref());

    match pkg1.cmp(pkg2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Convert a Pkg into an EbuildPkg.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_as_ebuild(p: *mut pkg::Pkg) -> *const ebuild::EbuildPkg {
    let pkg = null_ptr_check!(p.as_ref());
    let result = match pkg.as_ebuild() {
        Some((pkg, _repo)) => Ok(pkg),
        None => Err(Error::InvalidValue("invalid pkg format".to_string())),
    };
    unwrap_or_return!(result, ptr::null())
}

/// Return the hash value for a given package.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_hash(p: *mut pkg::Pkg) -> u64 {
    let pkg = null_ptr_check!(p.as_ref());
    hash(pkg)
}

/// Return the restriction for a given package.
///
/// # Safety
/// The argument must be a non-null Pkg pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_pkg_restrict(p: *mut pkg::Pkg) -> *mut restrict::Restrict {
    let pkg = null_ptr_check!(p.as_ref());
    Box::into_raw(Box::new(pkg.into()))
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
