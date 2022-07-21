use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::{mem, ptr};

use pkgcraft::{atom, eapi, restrict, utils::hash};

use crate::macros::*;

pub mod version;

// explicitly force symbols to be exported
// TODO: https://github.com/rust-lang/rfcs/issues/2771
/// Opaque wrapper for Atom objects.
pub struct Atom;
pub type Blocker = atom::Blocker;

/// Parse a string into an atom using a specific EAPI. Pass NULL for the eapi argument in
/// order to parse using the latest EAPI with extensions (e.g. support for repo deps).
///
/// Returns NULL on error.
///
/// # Safety
/// The atom argument should be a UTF-8 string while eapi can be a string or may be
/// NULL to use the default EAPI.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom(
    atom: *const c_char,
    eapi: *const c_char,
) -> *mut atom::Atom {
    let atom = null_ptr_check!(atom.as_ref());
    let atom = unsafe { unwrap_or_return!(CStr::from_ptr(atom).to_str(), ptr::null_mut()) };
    let eapi = unwrap_or_return!(eapi::IntoEapi::into_eapi(eapi), ptr::null_mut());
    let atom = unwrap_or_return!(atom::Atom::new(atom, eapi), ptr::null_mut());
    Box::into_raw(Box::new(atom))
}

/// Parse a CPV string into an atom.
///
/// Returns NULL on error.
///
/// # Safety
/// The argument should be a UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_cpv(s: *const c_char) -> *mut atom::Atom {
    let s = null_ptr_check!(s.as_ref());
    let atom = unsafe { unwrap_or_return!(CStr::from_ptr(s).to_str(), ptr::null_mut()) };
    let atom = unwrap_or_return!(atom::cpv(atom), ptr::null_mut());
    Box::into_raw(Box::new(atom))
}

/// Compare two atoms returning -1, 0, or 1 if the first atom is less than, equal to, or greater
/// than the second atom, respectively.
///
/// # Safety
/// The arguments must be non-null Atom pointers.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_cmp(a1: *mut atom::Atom, a2: *mut atom::Atom) -> c_int {
    let a1 = null_ptr_check!(a1.as_ref());
    let a2 = null_ptr_check!(a2.as_ref());

    match a1.cmp(a2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Return a given atom's category, e.g. the atom "=cat/pkg-1-r2" has a category of "cat".
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_category(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    CString::new(atom.category()).unwrap().into_raw()
}

/// Return a given atom's package, e.g. the atom "=cat/pkg-1-r2" has a package of "pkg".
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_package(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    CString::new(atom.package()).unwrap().into_raw()
}

/// Return a given atom's blocker status, e.g. the atom "!cat/pkg" has a weak blocker.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_blocker(atom: *mut atom::Atom) -> atom::Blocker {
    let atom = null_ptr_check!(atom.as_ref());
    atom.blocker()
}

/// Return a given atom's version, e.g. the atom "=cat/pkg-1-r2" has a version of "1-r2".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The argument must be a non-null Atom pointer. Also, note that the returned pointer
/// is borrowed from its related Atom object and should never be freed manually.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_version(atom: *mut atom::Atom) -> *const atom::Version {
    let atom = null_ptr_check!(atom.as_ref());
    match atom.version() {
        None => ptr::null_mut(),
        Some(v) => v,
    }
}

/// Return a given atom's revision, e.g. the atom "=cat/pkg-1-r2" has a revision of "2".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_revision(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
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
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_slot(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
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
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_subslot(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    match atom.subslot() {
        None => ptr::null_mut(),
        Some(s) => CString::new(s).unwrap().into_raw(),
    }
}

/// Return a given atom's slot operator, e.g. the atom "=cat/pkg-1-r2:0=" has an equal slot
/// operator.
///
/// Returns -1 on nonexistence.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_slot_op(atom: *mut atom::Atom) -> c_int {
    let atom = null_ptr_check!(atom.as_ref());
    match atom.slot_op() {
        None => -1,
        Some(op) => op as c_int,
    }
}

/// Return a given atom's USE dependencies, e.g. the atom "=cat/pkg-1-r2[a,b,c]" has USE
/// dependencies of "a, b, c".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_use_deps(
    atom: *mut atom::Atom,
    len: *mut usize,
) -> *mut *mut c_char {
    // TODO: switch from usize to std::os::raw::c_size_t when it's stable.
    let atom = null_ptr_check!(atom.as_ref());
    match atom.use_deps() {
        None => ptr::null_mut(),
        Some(use_deps) => {
            let mut ptrs: Vec<_> = use_deps
                .iter()
                .map(|s| CString::new(s.as_str()).unwrap().into_raw())
                .collect();
            ptrs.shrink_to_fit();
            unsafe { *len = ptrs.len() };
            let ptr = ptrs.as_mut_ptr();
            mem::forget(ptrs);
            ptr
        }
    }
}

/// Return a given atom's repo, e.g. the atom "=cat/pkg-1-r2:3/4::repo" has a repo of "repo".
///
/// Returns NULL on nonexistence.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_repo(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    match atom.repo() {
        None => ptr::null_mut(),
        Some(s) => CString::new(s).unwrap().into_raw(),
    }
}

/// Return a given atom's key, e.g. the atom "=cat/pkg-1-r2" has a key of "cat/pkg".
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_key(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    CString::new(atom.key()).unwrap().into_raw()
}

/// Return a given atom's cpv, e.g. the atom "=cat/pkg-1-r2" has a cpv of "cat/pkg-1-r2".
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_cpv(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    CString::new(atom.cpv()).unwrap().into_raw()
}

/// Return the string for a given atom.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_str(atom: *mut atom::Atom) -> *mut c_char {
    let atom = null_ptr_check!(atom.as_ref());
    CString::new(format!("{atom}")).unwrap().into_raw()
}

/// Return the hash value for a given atom.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_hash(atom: *mut atom::Atom) -> u64 {
    let atom = null_ptr_check!(atom.as_ref());
    hash(atom)
}

/// Return the restriction for a given atom.
///
/// # Safety
/// The argument must be a non-null Atom pointer.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_restrict(atom: *mut atom::Atom) -> *mut restrict::Restrict {
    let atom = null_ptr_check!(atom.as_ref());
    Box::into_raw(Box::new(atom.into()))
}

/// Free an atom.
///
/// # Safety
/// The argument must be a Atom pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_atom_free(atom: *mut atom::Atom) {
    if !atom.is_null() {
        unsafe { drop(Box::from_raw(atom)) };
    }
}
