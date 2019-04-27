#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![warn(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//!  Command line argument paw-rser abstraction for main.
//! ## Example
//!
//! ```rust
//! ```

use std::env::Args as EnvArgs;
use std::iter::Iterator;
use std::fmt;

#[doc(inline)]
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
pub use paw_attributes::main;

/// Args is a wrapper over env::Args which is an iterator over the arguments to a process.
pub struct Args {
    inner: EnvArgs,
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl ParseArgs for Args {
    type Error = std::io::Error;
    fn try_parse() -> Result<Self, Self::Error> {
        Ok(Self { inner: std::env::args() })
    }
}

/// Try parsing.
pub trait ParseArgs: Sized {
    /// Error type.
    type Error;

    /// Try to parse an input to a type.
    fn try_parse() -> Result<Self, Self::Error>;
}

#[derive(Debug)]
/// An error which is returned when an argument isn't passed to the process
pub struct ArgNotFoundError {
    arg: String,
}

impl std::fmt::Display for ArgNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "the argument {} is not passed", self.arg)
    }
}

impl std::error::Error for ArgNotFoundError {}
