use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::ptr::NonNull;

pub use pkgcraft::repo::ebuild::Repo as EbuildRepo;
use pkgcraft::repo::Repo;

/// Return a given ebuild repos's category dirs.
///
/// # Safety
/// The argument must be a non-null EbuildRepo pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_ebuild_repo_category_dirs(
    r: NonNull<EbuildRepo>,
    len: *mut usize,
) -> *mut *mut c_char {
    let repo = unsafe { r.as_ref() };
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
    r: NonNull<EbuildRepo>,
    len: *mut usize,
) -> *mut *mut Repo {
    let repo = unsafe { r.as_ref() };
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
