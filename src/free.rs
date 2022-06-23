use std::ffi::CString;
use std::os::raw::c_char;

/// Free a string.
///
/// # Safety
/// The argument must be a string pointer or NULL.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_str_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

/// Free an array of strings.
///
/// # Safety
/// The argument must be a pointer to a string array or NULL along with the length of the array.
#[no_mangle]
pub unsafe extern "C" fn pkgcraft_str_array_free(strs: *mut *mut c_char, len: usize) {
    if !strs.is_null() {
        unsafe {
            for s in Vec::from_raw_parts(strs, len, len).into_iter() {
                drop(CString::from_raw(s));
            }
        }
    }
}
