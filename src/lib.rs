//! Rust bindings for the [Oniguruma](https://github.com/kkos/oniguruma)
//! regular expressions library.
//!
//! Example of usage:
//!
//! ```rust
//! use oniguruma::Regex;
//!
//! let regex = Regex::new("e(l+)").unwrap();
//! for (i, pos) in regex.captures("hello").unwrap().iter_pos().enumerate() {
//!     match pos {
//!          Some((beg, end)) =>
//!              println!("Group {} captured in position {}:{}", i, beg, end),
//!          None =>
//!              println!("Group {} is not captured", i)
//!     }
//! }
//! ```
extern crate libc;

#[macro_use]
extern crate bitflags;

mod flags;
mod captures;
mod encoding;
mod regex;
mod region;
mod syntax;

#[cfg(test)]
mod test;

// re-export
pub use flags::*;
pub use captures::*;
pub use encoding::*;
pub use regex::*;
pub use region::*;
pub use syntax::*;
