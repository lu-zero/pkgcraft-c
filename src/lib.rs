#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod atom;
pub mod bash;
pub mod error;
mod macros;

pub use self::error::{Error, Result};
