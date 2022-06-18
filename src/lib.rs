#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod atom;
pub mod error;
pub mod free;
mod macros;
pub mod parse;

pub use self::error::{Error, Result};
