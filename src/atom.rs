use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr::{self, NonNull};

use pkgcraft::{atom, eapi, utils::hash};

use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

// force opaque types to be defined in pkgcraft.h
pub struct Atom;
pub struct Version;

/// Parse a string into an atom using a specific EAPI. Pass NULL for the eapi argument in
/// order to parse using the latest EAPI with extensions (e.g. support for repo deps).
///
/// Returns NULL on error.
///
/// # Safety
/// The atom and eapi arguments should point to valid strings.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom(
    atom: *const c_char,
    eapi: *const c_char,
) -> *mut atom::Atom {
    if atom.is_null() {
        let err = Error::new("no atom string provided");
        update_last_error(err);
        return ptr::null_mut();
    }

    let atom_str = unsafe { unwrap_or_return!(CStr::from_ptr(atom).to_str(), ptr::null_mut()) };

    let eapi = match eapi.is_null() {
        true => &eapi::EAPI_PKGCRAFT,
        false => match unsafe { CStr::from_ptr(eapi).to_str() } {
            Ok(s) => unwrap_or_return!(eapi::get_eapi(s), ptr::null_mut()),
            Err(e) => {
                update_last_error(e);
                return ptr::null_mut();
            }
        },
    };

    let atom = unwrap_or_return!(atom::parse::dep(atom_str, eapi), ptr::null_mut());
    Box::into_raw(Box::new(atom))
}

/// Compare two atoms returning -1, 0, or 1 if the first atom is less than, equal to, or greater
/// than the second atom, respectively.
///
/// # Safety
/// The atom arguments should be non-null Atom pointers received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_cmp(
    a1: NonNull<atom::Atom>,
    a2: NonNull<atom::Atom>,
) -> c_int {
    let (a1, a2) = unsafe { (a1.as_ref(), a2.as_ref()) };

    match a1.cmp(a2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return a given atom's category, e.g. the atom "=cat/pkg-1-r2" has a category of "cat".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_category(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.category()).unwrap().into_raw()
}

/// Return a given atom's package, e.g. the atom "=cat/pkg-1-r2" has a package of "pkg".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_package(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.package()).unwrap().into_raw()
}

/// Return a given atom's version, e.g. the atom "=cat/pkg-1-r2" has a package of "1-r2".
/// Returns an empty string on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_version(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    let s = atom.version().map(|v| v.as_str()).unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's slot, e.g. the atom "=cat/pkg-1-r2:3" has a slot of "3".
/// Returns an empty string on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_slot(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    let s = atom.slot().unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's subslot, e.g. the atom "=cat/pkg-1-r2:3/4" has a subslot of "4".
/// Returns an empty string on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_subslot(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    let s = atom.subslot().unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's repo, e.g. the atom "=cat/pkg-1-r2:3/4::repo" has a repo of "repo".
/// Returns an empty string on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_repo(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    let s = atom.repo().unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's key, e.g. the atom "=cat/pkg-1-r2" has a key of "cat/pkg".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_key(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.key()).unwrap().into_raw()
}

/// Return a given atom's cpv, e.g. the atom "=cat/pkg-1-r2" has a cpv of "cat/pkg-1-r2".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_cpv(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.cpv()).unwrap().into_raw()
}

/// Return the string for a given atom.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_str(atom: NonNull<atom::Atom>) -> *const c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(format!("{atom}")).unwrap().into_raw()
}

/// Free an atom.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_free(atom: NonNull<atom::Atom>) {
    let _ = unsafe { Box::from_raw(atom.as_ptr()) };
}

/// Return the hash value for a given atom.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_hash(atom: NonNull<atom::Atom>) -> u64 {
    let atom = unsafe { atom.as_ref() };
    hash(atom)
}

/// Parse a string into a version.
///
/// Returns NULL on error.
///
/// # Safety
/// The version argument should point to a valid string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version(version: *const c_char) -> *mut atom::Version {
    let ver_str = unsafe { unwrap_or_return!(CStr::from_ptr(version).to_str(), ptr::null_mut()) };
    let ver = unwrap_or_return!(atom::parse::version(ver_str), ptr::null_mut());
    Box::into_raw(Box::new(ver))
}

/// Compare two versions returning -1, 0, or 1 if the first version is less than, equal to, or greater
/// than the second version, respectively.
///
/// # Safety
/// The version arguments should be non-null Version pointers received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_cmp(
    v1: NonNull<atom::Version>,
    v2: NonNull<atom::Version>,
) -> c_int {
    let (v1, v2) = unsafe { (v1.as_ref(), v2.as_ref()) };

    match v1.cmp(v2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return a given version's revision, e.g. the version "1-r2" has a revision of "2".
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_revision(
    version: NonNull<atom::Version>,
) -> *const c_char {
    let version = unsafe { version.as_ref() };
    let s = version.revision().as_str();
    CString::new(s).unwrap().into_raw()
}

/// Return the string for a given version.
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_str(version: NonNull<atom::Version>) -> *const c_char {
    let version = unsafe { version.as_ref() };
    CString::new(version.as_str()).unwrap().into_raw()
}

/// Free a version.
///
/// # Safety
/// The version argument should be a non-null Version pointer received from pkgcraft_version().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_version_free(version: NonNull<atom::Version>) {
    let _ = unsafe { Box::from_raw(version.as_ptr()) };
}
