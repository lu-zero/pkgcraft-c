use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr::{self, NonNull};

use pkgcraft::{config, repo};

use crate::macros::unwrap_or_return;

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Config objects.
pub struct Config;

/// Wrapper for configured repos.
#[repr(C)]
pub struct RepoConfig {
    id: *mut c_char,
    repo: *const repo::Repo,
}

/// Return the pkgcraft config for the system.
///
/// Returns NULL on error.
#[no_mangle]
pub extern "C" fn pkgcraft_config() -> *mut config::Config {
    let config = unwrap_or_return!(config::Config::new("pkgcraft", "", false), ptr::null_mut());
    Box::into_raw(Box::new(config))
}

/// Add an external repo to a config.
///
/// Returns NULL on error.
///
/// # Safety
/// The path argument should be a valid path on the system.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_config_add_repo(
    mut config: NonNull<config::Config>,
    id: *const c_char,
    priority: c_int,
    path: NonNull<c_char>,
) -> *const repo::Repo {
    let path =
        unsafe { unwrap_or_return!(CStr::from_ptr(path.as_ref()).to_str(), ptr::null_mut()) };
    let id = match id.is_null() {
        true => path,
        false => unsafe { unwrap_or_return!(CStr::from_ptr(id).to_str(), ptr::null_mut()) },
    };
    let config = unsafe { config.as_mut() };
    let repo = unwrap_or_return!(config.add_repo(id, priority, path), ptr::null_mut());
    Box::into_raw(Box::new(repo))
}

/// Return the repos for a config.
///
/// # Safety
/// The config argument must be a non-null Config pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_config_repos(
    config: NonNull<config::Config>,
    len: *mut usize,
) -> *mut *mut RepoConfig {
    // TODO: switch from usize to std::os::raw::c_size_t when it's stable.
    let config = unsafe { config.as_ref() };
    let repos: Vec<_> = config.repos.iter().collect();
    unsafe { *len = repos.len() };
    let mut ptrs: Vec<_> = repos
        .iter()
        .copied()
        .map(|(id, r)| {
            let r = RepoConfig {
                id: CString::new(id).unwrap().into_raw(),
                repo: r,
            };
            Box::into_raw(Box::new(r))
        })
        .collect();
    ptrs.shrink_to_fit();
    let p = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    p
}

/// Free an array of configured repos.
///
/// # Safety
/// The argument must be the value received from pkgcraft_config_repos() or NULL along with the
/// length of the array.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repos_free(repos: *mut *mut RepoConfig, len: usize) {
    if !repos.is_null() {
        unsafe {
            for r in Vec::from_raw_parts(repos, len, len).into_iter() {
                let repo = Box::from_raw(r);
                drop(CString::from_raw(repo.id));
            }
        }
    }
}

/// Free a config.
///
/// # Safety
/// The argument must be a Config pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_config_free(config: *mut config::Config) {
    if !config.is_null() {
        unsafe { drop(Box::from_raw(config)) };
    }
}
