pub mod util;

#[macro_use]
mod macros;

pub mod attr;
mod escape;
mod event;
mod write;

pub use event::Event;
pub use write::HtmlWriter;
