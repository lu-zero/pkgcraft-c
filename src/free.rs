use std::ffi::CString;
use std::os::raw::c_char;

/// Free a string previously allocated by rust.
///
/// # Safety
/// This allows calling against NULL since some string-related functions return NULL when no value
/// exists.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_str_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

/// Free an array of strings previously allocated by rust.
///
/// # Safety
/// This allows calling against NULL since some string array related functions return NULL when no
/// value exists.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_str_array_free(array: *mut *mut c_char, len: usize) {
    if !array.is_null() {
        unsafe {
            for s in Vec::from_raw_parts(array, len, len).into_iter() {
                drop(CString::from_raw(s));
            }
        }
    }
}
