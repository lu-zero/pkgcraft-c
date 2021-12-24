use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{mem, ptr};

use pkgcraft::{atom, eapi};

use crate::error::update_last_error;
use crate::{Error, Result};

#[repr(C)]
pub struct Atom {
    string: *const c_char,
    eapi: *const c_char,
    category: *const c_char,
    package: *const c_char,
    version: *const c_char,
    slot: *const c_char,
    subslot: *const c_char,
    use_deps: *const *const c_char,
    // TODO: switch to c_size_t once it's non-experimental
    // https://doc.rust-lang.org/std/os/raw/type.c_size_t.html
    use_deps_len: usize,
    repo: *const c_char,
}

/// Parse a string into an atom using a specific EAPI. Pass a null pointer for the eapi argument in
/// order to parse using the latest EAPI with extensions (e.g. support for repo deps).
#[no_mangle]
pub extern "C" fn str_to_atom(atom: *const c_char, eapi: *const c_char) -> *mut Atom {
    if atom.is_null() {
        let err = Error::new("no atom string provided");
        update_last_error(err);
        return ptr::null_mut();
    }

    let atom_str = match unsafe { CStr::from_ptr(atom).to_str() } {
        Ok(s) => s,
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    let eapi = match eapi.is_null() {
        true => &eapi::EAPI_PKGCRAFT,
        false => match unsafe { CStr::from_ptr(eapi).to_str() } {
            Ok(s) => match eapi::get_eapi(s) {
                Ok(eapi) => eapi,
                Err(e) => {
                    update_last_error(e);
                    return ptr::null_mut();
                }
            },
            Err(e) => {
                update_last_error(e);
                return ptr::null_mut();
            }
        },
    };

    let atom = match atom::parse::dep(atom_str, eapi) {
        Ok(a) => a,
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    // parsing should catch errors so no need to check here
    let string = CString::new(atom_str).unwrap().into_raw();
    let eapi = CString::new(eapi.to_string()).unwrap().into_raw();
    let category = CString::new(atom.category).unwrap().into_raw();
    let package = CString::new(atom.package).unwrap().into_raw();

    let version = match atom.version {
        Some(s) => CString::new(format!("{}", s)).unwrap().into_raw(),
        None => ptr::null(),
    };

    let slot = match atom.slot {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => ptr::null(),
    };

    let subslot = match atom.subslot {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => ptr::null(),
    };

    let mut use_strs = vec![];
    if let Some(use_deps) = atom.use_deps {
        for u in use_deps.iter() {
            use_strs.push(CString::new(u.as_str()).unwrap().into_raw())
        }
    }
    let use_deps_len = use_strs.len();
    // TODO: switch to into_raw_parts() once it's non-experimental
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts
    let use_deps = Box::into_raw(use_strs.into_boxed_slice()).cast();

    let repo = match atom.repo {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => ptr::null(),
    };

    // create C-compatible struct
    let c_atom = Atom {
        string,
        eapi,
        category,
        package,
        version,
        slot,
        subslot,
        use_deps,
        use_deps_len,
        repo,
    };

    Box::into_raw(Box::new(c_atom))
}

/// Convert a C-compatible Atom struct to a rust Atom struct.
pub fn atom_to_rust(atom: *mut Atom) -> Result<atom::Atom> {
    if atom.is_null() {
        return Err(Error::new("no atom provided"));
    }

    let atom = unsafe { Box::from_raw(atom) };
    let atom_str = unsafe { CStr::from_ptr(atom.string) }
        .to_str()
        .map_err(|e| Error {
            message: format!("invalid atom string: {:?}", e),
        })?;

    let eapi = match atom.eapi.is_null() {
        true => &eapi::EAPI_PKGCRAFT,
        false => {
            let eapi_str = unsafe { CStr::from_ptr(atom.eapi) }
                .to_str()
                .map_err(|e| Error {
                    message: format!("invalid eapi string: {:?}", e),
                })?;
            eapi::get_eapi(eapi_str)?
        }
    };

    // don't deallocate memory when `atom` is dropped
    mem::forget(atom);

    atom::parse::dep(atom_str, eapi).map_err(|e| Error {
        message: e.to_string(),
    })
}

/// Return a given atom's key, e.g. the atom "=cat/pkg-1-r2" has a key of "cat/pkg".
/// Returns a null pointer on error.
#[no_mangle]
pub extern "C" fn atom_key(atom: *mut Atom) -> *const c_char {
    let key = match atom_to_rust(atom) {
        Ok(a) => a.key(),
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    CString::new(key).unwrap().into_raw()
}

/// Return a given atom's cpv, e.g. the atom "=cat/pkg-1-r2" has a cpv of "cat/pkg-1-r2".
/// Returns a null pointer on error.
#[no_mangle]
pub extern "C" fn atom_cpv(atom: *mut Atom) -> *const c_char {
    let cpv = match atom_to_rust(atom) {
        Ok(a) => a.cpv(),
        Err(e) => {
            update_last_error(e);
            return ptr::null_mut();
        }
    };

    CString::new(cpv).unwrap().into_raw()
}

/// Free an atom.
#[no_mangle]
pub unsafe extern "C" fn atom_free(atom: *mut Atom) {
    if atom.is_null() {
        return;
    }

    let a = Box::from_raw(atom);
    drop(CString::from_raw(a.string as *mut _));
    drop(CString::from_raw(a.eapi as *mut _));
    drop(CString::from_raw(a.category as *mut _));
    drop(CString::from_raw(a.package as *mut _));
    if !a.version.is_null() {
        drop(CString::from_raw(a.version as *mut _));
    }
    if !a.slot.is_null() {
        drop(CString::from_raw(a.slot as *mut _));
    }
    if !a.subslot.is_null() {
        drop(CString::from_raw(a.subslot as *mut _));
    }
    if !a.use_deps.is_null() {
        let use_deps = Vec::from_raw_parts(a.use_deps as *mut _, a.use_deps_len, a.use_deps_len);
        for &u in use_deps.iter() {
            drop(CString::from_raw(u));
        }
    }
    if !a.repo.is_null() {
        drop(CString::from_raw(a.repo as *mut _));
    }
}
