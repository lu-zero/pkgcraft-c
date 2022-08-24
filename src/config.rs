use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::{mem, ptr};

use pkgcraft::repo::Repository;
use pkgcraft::{config, repo};

use crate::macros::*;
use crate::repo::RepoFormat;

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Config objects.
pub struct Config;

/// Wrapper for configured repos.
#[repr(C)]
pub struct RepoConfig {
    id: *mut c_char,
    format: RepoFormat,
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

/// Add local repo from filesystem path.
///
/// Returns NULL on error.
///
/// # Safety
/// The path argument should be a valid path on the system.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_config_add_repo_path(
    config: *mut config::Config,
    id: *const c_char,
    priority: c_int,
    path: *const c_char,
) -> *mut RepoConfig {
    let path = null_ptr_check!(path.as_ref());
    let path = unsafe { unwrap_or_return!(CStr::from_ptr(path).to_str(), ptr::null_mut()) };
    let id = match id.is_null() {
        true => path,
        false => unsafe { unwrap_or_return!(CStr::from_ptr(id).to_str(), ptr::null_mut()) },
    };
    let config = null_ptr_check!(config.as_mut());
    let repo = unwrap_or_return!(config.add_repo_path(id, priority, path), ptr::null_mut());
    let repo_conf = RepoConfig {
        id: CString::new(id).unwrap().into_raw(),
        format: (&repo).into(),
        repo: Box::into_raw(Box::new(repo)),
    };
    Box::into_raw(Box::new(repo_conf))
}

/// Load repos from a given path to a portage-compatible repos.conf directory or file.
///
/// Returns NULL on error.
///
/// # Safety
/// The path argument should be a valid path on the system.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_config_load_repos_conf(
    config: *mut config::Config,
    path: *const c_char,
    len: *mut usize,
) -> *mut *mut RepoConfig {
    let path = null_ptr_check!(path.as_ref());
    let path = unsafe { unwrap_or_return!(CStr::from_ptr(path).to_str(), ptr::null_mut()) };
    let config = null_ptr_check!(config.as_mut());
    let repos = unwrap_or_return!(config.load_repos_conf(path), ptr::null_mut());
    let mut ptrs: Vec<_> = repos
        .into_iter()
        .map(|repo| {
            let repo_conf = RepoConfig {
                id: CString::new(repo.id()).unwrap().into_raw(),
                format: (&repo).into(),
                repo: Box::into_raw(Box::new(repo)),
            };
            Box::into_raw(Box::new(repo_conf))
        })
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let p = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    p
}

/// Return the repos for a config.
///
/// # Safety
/// The config argument must be a non-null Config pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_config_repos(
    config: *mut config::Config,
    len: *mut usize,
) -> *mut *mut RepoConfig {
    // TODO: switch from usize to std::os::raw::c_size_t when it's stable.
    let config = null_ptr_check!(config.as_ref());
    let mut ptrs: Vec<_> = config
        .repos
        .into_iter()
        .map(|(id, repo)| {
            let repo_conf = RepoConfig {
                id: CString::new(id).unwrap().into_raw(),
                format: repo.into(),
                repo,
            };
            Box::into_raw(Box::new(repo_conf))
        })
        .collect();
    ptrs.shrink_to_fit();
    unsafe { *len = ptrs.len() };
    let p = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    p
}

/// Free a repo config.
///
/// Note that repo pointers aren't explicitly freed since different calls return borrowed or owned
/// pointers so external users should handle freeing them if necessary via [`pkgcraft_repo_free`].
///
/// # Safety
/// The argument must be a RepoConfig pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_repo_config_free(r: *mut RepoConfig) {
    if !r.is_null() {
        unsafe {
            let repo_conf = Box::from_raw(r);
            drop(CString::from_raw(repo_conf.id));
        }
    }
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
                pkgcraft_repo_config_free(r);
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
