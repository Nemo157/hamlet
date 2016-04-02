pub mod util;

#[macro_use]
mod macros;

mod attr;
mod escape;
mod event;
mod write;

pub use event::Event;
pub use attr::Attribute;
pub use attr::AttributeSet;
pub use write::HtmlWriter;
