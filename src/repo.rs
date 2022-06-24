use std::cmp::Ordering;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr::{self, NonNull};

use pkgcraft::repo::Repository;
use pkgcraft::{pkg, repo, utils::hash};

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Repo objects.
pub struct Repo;
/// Opaque wrapper for PkgIter objects.
pub struct PkgIter;

/// Return a given repo's id.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_id(r: NonNull<repo::Repo>) -> *mut c_char {
    let repo = unsafe { r.as_ref() };
    CString::new(repo.id()).unwrap().into_raw()
}

/// Return a given repo's length.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_len(r: NonNull<repo::Repo>) -> usize {
    let repo = unsafe { r.as_ref() };
    repo.len()
}

/// Compare two repos returning -1, 0, or 1 if the first repo is less than, equal to, or greater
/// than the second repo, respectively.
///
/// # Safety
/// The arguments must be non-null Repo pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_cmp(
    r1: NonNull<repo::Repo>,
    r2: NonNull<repo::Repo>,
) -> c_int {
    let (repo1, repo2) = unsafe { (r1.as_ref(), r2.as_ref()) };

    match repo1.cmp(repo2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return the hash value for a given repo.
///
/// # Safety
/// The argument must be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_hash(r: NonNull<repo::Repo>) -> u64 {
    let repo = unsafe { r.as_ref() };
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
pub unsafe extern "C" fn pkgcraft_repo_iter<'a>(r: NonNull<repo::Repo>) -> *mut repo::PkgIter<'a> {
    let repo = unsafe { r.as_ref() };
    Box::into_raw(Box::new(repo.iter()))
}

/// Return the next package from a given package iterator.
///
/// Returns NULL when the iterator is empty.
///
/// # Safety
/// The argument must be a non-null PkgIter pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_iter_next(mut i: NonNull<repo::PkgIter>) -> *mut pkg::Pkg {
    let iter = unsafe { i.as_mut() };
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
