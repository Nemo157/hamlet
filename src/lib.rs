#[macro_use]
pub mod macros;
pub mod attribute;

mod escape;
mod event;
mod stream;

pub use event::Event;
pub use attribute::Attribute;
pub use stream::HtmlStreamer;
