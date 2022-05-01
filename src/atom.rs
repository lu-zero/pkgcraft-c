use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use pkgcraft::{atom, eapi};

use crate::error::update_last_error;
use crate::macros::unwrap_or_return;
use crate::Error;

// force an opaque type to be defined in pkgcraft.h
pub struct Atom;

/// Parse a string into an atom using a specific EAPI. Pass a null pointer for the eapi argument in
/// order to parse using the latest EAPI with extensions (e.g. support for repo deps).
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

macro_rules! ptr_to_atom {
    ($x:expr) => {
        match $x.is_null() {
            true => {
                let err = Error::new("no atom provided");
                update_last_error(err);
                return ptr::null_mut();
            }
            false => unsafe { &*$x },
        }
    };
}

/// Return a given atom's category, e.g. the atom "=cat/pkg-1-r2" has a category of "cat".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_category(atom: *mut atom::Atom) -> *const c_char {
    let atom = ptr_to_atom!(atom);
    CString::new(atom.category()).unwrap().into_raw()
}

/// Return a given atom's package, e.g. the atom "=cat/pkg-1-r2" has a package of "pkg".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_package(atom: *mut atom::Atom) -> *const c_char {
    let atom = ptr_to_atom!(atom);
    CString::new(atom.package()).unwrap().into_raw()
}

/// Return a given atom's version, e.g. the atom "=cat/pkg-1-r2" has a package of "1-r2".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_version(atom: *mut atom::Atom) -> *const c_char {
    let atom = ptr_to_atom!(atom);
    let s = atom.fullver().unwrap_or_else(|| "".into());
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's slot, e.g. the atom "=cat/pkg-1-r2:3" has a slot of "3".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_slot(atom: *mut atom::Atom) -> *const c_char {
    let atom = ptr_to_atom!(atom);
    let s = atom.slot().unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's subslot, e.g. the atom "=cat/pkg-1-r2:3/4" has a subslot of "4".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_subslot(atom: *mut atom::Atom) -> *const c_char {
    let atom = ptr_to_atom!(atom);
    let s = atom.subslot().unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's repo, e.g. the atom "=cat/pkg-1-r2:3/4::repo" has a repo of "repo".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_repo(atom: *mut atom::Atom) -> *const c_char {
    let atom = ptr_to_atom!(atom);
    let s = atom.repo().unwrap_or("");
    CString::new(s).unwrap().into_raw()
}

/// Return a given atom's key, e.g. the atom "=cat/pkg-1-r2" has a key of "cat/pkg".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom received from str_to_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_key(atom: *mut atom::Atom) -> *const c_char {
    let key = unsafe { (*atom).key() };
    CString::new(key).unwrap().into_raw()
}

/// Return a given atom's cpv, e.g. the atom "=cat/pkg-1-r2" has a cpv of "cat/pkg-1-r2".
/// Returns a null pointer on error.
///
/// # Safety
/// The atom argument should only correspond to an Atom received from str_to_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_cpv(atom: *mut atom::Atom) -> *const c_char {
    let cpv = unsafe { (*atom).cpv() };
    CString::new(cpv).unwrap().into_raw()
}

/// Free an atom.
///
/// # Safety
/// The atom argument should only correspond to an Atom received from str_to_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_free(atom: *mut atom::Atom) {
    if atom.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(atom) };
}
