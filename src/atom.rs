use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr::{self, NonNull};

use pkgcraft::{atom, eapi, utils::hash};

use crate::macros::unwrap_or_return;

// force opaque types to be defined in pkgcraft.h
pub struct Atom;
pub type Blocker = atom::Blocker;

/// Parse a string into an atom using a specific EAPI. Pass NULL for the eapi argument in
/// order to parse using the latest EAPI with extensions (e.g. support for repo deps).
///
/// Returns NULL on error.
///
/// # Safety
/// The atom argument should be a valid string while eapi can be a string or may be
/// NULL to use the default EAPI.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom(
    atom: NonNull<c_char>,
    eapi: *const c_char,
) -> *mut atom::Atom {
    let atom =
        unsafe { unwrap_or_return!(CStr::from_ptr(atom.as_ref()).to_str(), ptr::null_mut()) };
    let eapi = unwrap_or_return!(eapi::IntoEapi::into_eapi(eapi), ptr::null_mut());
    let atom = unwrap_or_return!(atom::Atom::new(atom, eapi), ptr::null_mut());
    Box::into_raw(Box::new(atom))
}

/// Parse a CPV string into an atom.
///
/// Returns NULL on error.
///
/// # Safety
/// The atom argument should be a valid UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_cpv(atom: NonNull<c_char>) -> *mut atom::Atom {
    let atom =
        unsafe { unwrap_or_return!(CStr::from_ptr(atom.as_ref()).to_str(), ptr::null_mut()) };
    let atom = unwrap_or_return!(atom::cpv(atom), ptr::null_mut());
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
pub unsafe extern "C" fn pkgcraft_atom_category(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.category()).unwrap().into_raw()
}

/// Return a given atom's package, e.g. the atom "=cat/pkg-1-r2" has a package of "pkg".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_package(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.package()).unwrap().into_raw()
}

/// Return a given atom's blocker status, e.g. the atom "!cat/pkg" has a weak blocker.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_blocker(atom: NonNull<atom::Atom>) -> u8 {
    let atom = unsafe { atom.as_ref() };
    atom.blocker() as u8
}

/// Return a given atom's version, e.g. the atom "=cat/pkg-1-r2" has a version of "1-r2".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_version(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.version() {
        None => ptr::null_mut(),
        Some(v) => CString::new(v.as_str()).unwrap().into_raw(),
    }
}

/// Return a given atom's revision, e.g. the atom "=cat/pkg-1-r2" has a revision of "2".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_revision(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.revision() {
        None => ptr::null_mut(),
        Some(r) => CString::new(r.as_str()).unwrap().into_raw(),
    }
}

/// Return a given atom's slot, e.g. the atom "=cat/pkg-1-r2:3" has a slot of "3".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_slot(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.slot() {
        None => ptr::null_mut(),
        Some(s) => CString::new(s).unwrap().into_raw(),
    }
}

/// Return a given atom's subslot, e.g. the atom "=cat/pkg-1-r2:3/4" has a subslot of "4".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_subslot(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.subslot() {
        None => ptr::null_mut(),
        Some(s) => CString::new(s).unwrap().into_raw(),
    }
}

/// Return a given atom's slot operator, e.g. the atom "=cat/pkg-1-r2:0=" has a slot operator of
/// "=".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_slot_op(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.slot_op() {
        None => ptr::null_mut(),
        Some(s) => CString::new(s).unwrap().into_raw(),
    }
}

/// Return a given atom's USE dependencies, e.g. the atom "=cat/pkg-1-r2[a,b,c]" has USE
/// dependencies of "a, b, c".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_use_deps(atom: NonNull<atom::Atom>) -> *mut *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.use_deps() {
        None => ptr::null_mut(),
        Some(use_deps) => {
            let use_strs: Vec<_> = use_deps.iter().map(|&s| CString::new(s).unwrap()).collect();
            let mut use_ptrs: Vec<_> = use_strs.iter().map(|s| s.as_ptr() as *mut _).collect();
            use_ptrs.push(ptr::null_mut());
            let p = use_ptrs.as_mut_ptr();
            mem::forget(use_strs);
            mem::forget(use_ptrs);
            p
        }
    }
}

/// Return a given atom's repo, e.g. the atom "=cat/pkg-1-r2:3/4::repo" has a repo of "repo".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_repo(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    match atom.repo() {
        None => ptr::null_mut(),
        Some(s) => CString::new(s).unwrap().into_raw(),
    }
}

/// Return a given atom's key, e.g. the atom "=cat/pkg-1-r2" has a key of "cat/pkg".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_key(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.key()).unwrap().into_raw()
}

/// Return a given atom's cpv, e.g. the atom "=cat/pkg-1-r2" has a cpv of "cat/pkg-1-r2".
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_cpv(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(atom.cpv()).unwrap().into_raw()
}

/// Return the string for a given atom.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_str(atom: NonNull<atom::Atom>) -> *mut c_char {
    let atom = unsafe { atom.as_ref() };
    CString::new(format!("{atom}")).unwrap().into_raw()
}

/// Free an atom.
///
/// # Safety
/// The atom argument should be a non-null Atom pointer received from pkgcraft_atom().
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_free(atom: *mut atom::Atom) {
    if !atom.is_null() {
        unsafe { drop(Box::from_raw(atom)) };
    }
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
