//! Provides token definitions for HTML stream processing. The goal of this
//! library is to provide a simple API over which higher abstraction can be
//! built on.
//!
//! ## Example
//!
//! ```rust
//! #[macro_use]
//! extern crate hamlet;
//!
//! use std::fmt::Write;
//!
//! fn main() {
//!     use hamlet::Token;
//!     let tokens = vec![
//!         Token::text("Hello, "),
//!         Token::start_tag("small", attrs!(class="foo")),
//!         Token::text("world!"),
//!         Token::end_tag("small"),
//!     ];
//!
//!     let mut html = String::from("");
//!     for token in tokens {
//!         write!(html, "{}", token);
//!     }
//!
//!     assert_eq!(html, "Hello, <small class=\"foo\">world!</small>");
//! }
//! ```

#![warn(missing_docs)]

/// Currently contains just a semi-private utility function to support the
/// [`attrs!`](./macro.attrs!.html) macro.
pub mod util;

#[macro_use]
mod macros;

/// Contains structs for defining attributes on elements.
pub mod attr;
mod escape;
mod token;

pub use token::Token;
