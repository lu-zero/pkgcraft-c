use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::{fmt, mem, ptr, slice};

use tracing::{error, warn};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new<S: Into<String>>(msg: S) -> Error {
        Error {
            message: msg.into(),
        }
    }
}

impl From<pkgcraft::Error> for Error {
    fn from(e: pkgcraft::Error) -> Self {
        Error::new(e.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<dyn std::error::Error>>> = RefCell::new(None);
}

/// Update the most recent error, clearing the previous value.
pub(crate) fn update_last_error<E: std::error::Error + 'static>(err: E) {
    error!("Setting LAST_ERROR: {}", err);

    {
        // Print a pseudo-backtrace for this error, following back each error's
        // source until we reach the root error.
        let mut source = err.source();
        while let Some(parent_err) = source {
            warn!("Caused by: {}", parent_err);
            source = parent_err.source();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Get the most recent error message as a UTF-8 string, if none exists a null pointer is returned.
///
/// # Safety
/// The caller is expected to free the error string using error_message_free() after they're
/// finished using it.
#[no_mangle]
pub extern "C" fn pkgcraft_last_error() -> *mut c_char {
    // Retrieve the most recent error, clearing it in the process.
    let last_error: Option<Box<dyn std::error::Error>> =
        LAST_ERROR.with(|prev| prev.borrow_mut().take());
    match last_error {
        Some(e) => CString::new(e.to_string())
            .expect("invalid error message")
            .into_raw(),
        None => ptr::null_mut(),
    }
}

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
pub unsafe extern "C" fn pkgcraft_str_array_free(array: *mut *mut c_char) {
    if !array.is_null() {
        let len = mem::size_of_val(&array) / mem::size_of::<*mut c_char>();
        unsafe {
            for s in slice::from_raw_parts(array, len) {
                drop(CString::from_raw(*s));
            }
        }
    }
}
