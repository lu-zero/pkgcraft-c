/// Panic on null pointer.
macro_rules! null_ptr_check {
    ( $ptr:expr ) => {
        match unsafe { $ptr } {
            Some(p) => p,
            None => panic!("unexpected null pointer argument"),
        }
    };
}
pub(crate) use null_ptr_check;

/// Unwrap the returned value of a given expression or return the given value.
macro_rules! unwrap_or_return {
    ( $e:expr, $v:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $crate::error::update_last_error(e);
                return $v;
            }
        }
    };
}
pub(crate) use unwrap_or_return;
