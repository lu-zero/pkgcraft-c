use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

pub use pkgcraft::repo::ebuild::Repo as EbuildRepo;
use pkgcraft::repo::Repo;

use crate::macros::*;

/// Return a given ebuild repos's category dirs.
///
/// # Safety
/// The argument must be a non-null EbuildRepo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_repo_category_dirs(
    r: *const EbuildRepo,
    len: *mut usize,
) -> *mut *mut c_char {
    let repo = null_ptr_check!(r.as_ref());
    let mut ptrs: Vec<_> = repo
        .category_dirs()
        .into_iter()
        .map(|s| CString::new(s).unwrap().into_raw())
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let ptr = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    ptr
}

/// Return a given ebuild repos's masters.
///
/// # Safety
/// The argument must be a non-null EbuildRepo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_repo_masters(
    r: *const EbuildRepo,
    len: *mut usize,
) -> *mut *mut Repo {
    let repo = null_ptr_check!(r.as_ref());
    let mut ptrs: Vec<_> = repo
        .masters()
        .iter()
        .map(|r| Box::into_raw(Box::new(Repo::Ebuild(r.clone()))))
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let ptr = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    ptr
}
