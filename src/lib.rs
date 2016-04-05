//! Provides event definitions for HTML stream processing. The goal of this
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
//!     use hamlet::Event;
//!     let events = vec![
//!         Event::text("Hello, "),
//!         Event::start_tag("small", attr_set!(class="foo")),
//!         Event::text("world!"),
//!         Event::end_tag("small"),
//!     ];
//!
//!     let mut html = String::from("");
//!     for event in events {
//!         write!(html, "{}", event);
//!     }
//!
//!     assert_eq!(html, "Hello, <small class=\"foo\">world!</small>");
//! }
//! ```

pub mod util;

#[macro_use]
mod macros;

pub mod attr;
mod escape;
mod event;

pub use event::Event;
