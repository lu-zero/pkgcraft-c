/// Unwrap the returned value of a given expression or return the given value.
macro_rules! unwrap_or_return {
    ( $e:expr, $v:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                update_last_error(e);
                return $v;
            }
        }
    };
}
pub(crate) use unwrap_or_return;
