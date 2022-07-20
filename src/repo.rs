use std::cmp::Ordering;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::Arc;

use pkgcraft::repo::Repository;
use pkgcraft::{pkg, repo, restrict, utils::hash, Error};

use crate::macros::*;

pub mod ebuild;

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Repo objects.
pub struct Repo;
/// Opaque wrapper for PkgIter objects.
pub struct PkgIter;
/// Opaque wrapper for RestrictPkgIter objects.
pub struct RestrictPkgIter;

#[repr(C)]
pub enum RepoFormat {
    Ebuild,
    Fake,
    Empty,
}

impl From<&repo::Repo> for RepoFormat {
    fn from(repo: &repo::Repo) -> Self {
        match repo {
            repo::Repo::Ebuild(_) => Self::Ebuild,
            repo::Repo::Fake(_) => Self::Fake,
            repo::Repo::Unsynced(_) => Self::Empty,
        }
    }
}

/// Return a given repo's id.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_id(r: *mut repo::Repo) -> *mut c_char {
    let repo = null_ptr_check!(r.as_ref());
    CString::new(repo.id()).unwrap().into_raw()
}

/// Return a given repo's length.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_len(r: *mut repo::Repo) -> usize {
    let repo = null_ptr_check!(r.as_ref());
    repo.len()
}

/// Compare two repos returning -1, 0, or 1 if the first repo is less than, equal to, or greater
/// than the second repo, respectively.
///
/// # Safety
/// The arguments must be non-null Repo pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_cmp(r1: *mut repo::Repo, r2: *mut repo::Repo) -> c_int {
    let repo1 = null_ptr_check!(r1.as_ref());
    let repo2 = null_ptr_check!(r2.as_ref());

    match repo1.cmp(repo2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Convert a Repo into an EbuildRepo.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_as_ebuild(r: *mut repo::Repo) -> *const ebuild::EbuildRepo {
    let repo = null_ptr_check!(r.as_ref());
    let result = repo
        .as_ebuild()
        .ok_or_else(|| Error::InvalidValue("invalid repo format".to_string()));
    let repo = unwrap_or_return!(result, ptr::null());
    Arc::as_ptr(repo)
}

/// Return the hash value for a given repo.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_hash(r: *mut repo::Repo) -> u64 {
    let repo = null_ptr_check!(r.as_ref());
    hash(repo)
}

/// Free a repo.
///
/// # Safety
/// The argument must be a Repo pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_free(r: *mut repo::Repo) {
    if !r.is_null() {
        unsafe { drop(Box::from_raw(r)) };
    }
}

/// Return a package iterator for a given repo.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_iter<'a>(r: *mut repo::Repo) -> *mut repo::PkgIter<'a> {
    let repo = null_ptr_check!(r.as_ref());
    Box::into_raw(Box::new(repo.iter()))
}

/// Return the next package from a given package iterator.
///
/// Returns NULL when the iterator is empty.
///
/// # Safety
/// The argument must be a non-null PkgIter pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_iter_next(i: *mut repo::PkgIter) -> *mut pkg::Pkg {
    let iter = null_ptr_check!(i.as_mut());
    match iter.next() {
        None => ptr::null_mut(),
        Some(p) => Box::into_raw(Box::new(p)),
    }
}

/// Free a repo iterator.
///
/// # Safety
/// The argument must be a non-null PkgIter pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_iter_free(i: *mut repo::PkgIter) {
    if !i.is_null() {
        unsafe { drop(Box::from_raw(i)) };
    }
}

/// Return a restriction package iterator for a given repo.
///
/// # Safety
/// The repo argument must be a non-null Repo pointer and the restrict argument must be a non-null
/// Restrict pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_restrict_iter<'a>(
    repo: *mut repo::Repo,
    restrict: *mut restrict::Restrict,
) -> *mut repo::RestrictPkgIter<'a> {
    let repo = null_ptr_check!(repo.as_ref());
    let restrict = null_ptr_check!(restrict.as_ref());
    Box::into_raw(Box::new(repo.iter_restrict(restrict.clone())))
}

/// Return the next package from a given restriction package iterator.
///
/// Returns NULL when the iterator is empty.
///
/// # Safety
/// The argument must be a non-null RestrictPkgIter pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_restrict_iter_next(
    i: *mut repo::RestrictPkgIter,
) -> *mut pkg::Pkg {
    let iter = null_ptr_check!(i.as_mut());
    match iter.next() {
        None => ptr::null_mut(),
        Some(p) => Box::into_raw(Box::new(p)),
    }
}

/// Free a repo iterator.
///
/// # Safety
/// The argument must be a non-null RestrictPkgIter pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_restrict_iter_free(i: *mut repo::RestrictPkgIter) {
    if !i.is_null() {
        unsafe { drop(Box::from_raw(i)) };
    }
}
