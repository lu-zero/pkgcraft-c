#![warn(unreachable_pub)]

pub mod atom;
pub mod bash;
pub mod error;
mod macros;

pub use self::error::{Error, Result};
