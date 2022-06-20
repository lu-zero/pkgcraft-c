use std::cmp::Ordering;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;

use pkgcraft::repo::Repository;
use pkgcraft::{repo, utils::hash};

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Repo objects.
pub struct Repo;

/// Return a given repo's id.
///
/// # Safety
/// The repo argument should be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_id(repo: NonNull<repo::Repo>) -> *mut c_char {
    let repo = unsafe { repo.as_ref() };
    CString::new(repo.id()).unwrap().into_raw()
}

/// Compare two repos returning -1, 0, or 1 if the first repo is less than, equal to, or greater
/// than the second repo, respectively.
///
/// # Safety
/// The repo arguments should be non-null Repo pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_cmp(
    ptr1: NonNull<repo::Repo>,
    ptr2: NonNull<repo::Repo>,
) -> c_int {
    let (repo1, repo2) = unsafe { (ptr1.as_ref(), ptr2.as_ref()) };

    match repo1.cmp(repo2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return the hash value for a given repo.
///
/// # Safety
/// The repo argument should be a non-null Repo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_hash(repo: NonNull<repo::Repo>) -> u64 {
    let repo = unsafe { repo.as_ref() };
    hash(repo)
}
