use std::fmt;
use Event;

#[derive(Debug)]
pub struct Error;

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Error {
        Error
    }
}

pub trait Write {
    fn write_event(&mut self, event: Event) -> Result<(), Error>;
    fn write_events<'a, I: IntoIterator<Item = Event<'a>>>(&mut self,
                                                           events: I)
                                                           -> Result<(), Error> {
        for event in events {
            try!(self.write_event(event));
        }
        Ok(())
    }
}

impl<T: fmt::Write> Write for T {
    fn write_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::StartTag { name, attrs } => {
                try!(write!(self, "<{}", name));
                for attr in attrs.into_iter() {
                    // TODO: Escaping
                    try!(write!(self, " {}=\"{}\"", attr.name, attr.value));
                }
                try!(write!(self, ">"));
            }
            Event::ClosedTag { name, attrs } => {
                try!(write!(self, "<{}", name));
                for attr in attrs.into_iter() {
                    // TODO: Escaping
                    try!(write!(self, " {}=\"{}\"", attr.name, attr.value));
                }
                try!(write!(self, " />"));
            }
            Event::EndTag { name } => {
                try!(write!(self, "</{}>", name));
            }
            Event::Text(text) => {
                // TODO: Escaping
                try!(write!(self, "{}", text));
            }
            Event::RawHtml(html) => {
                try!(write!(self, "{}", html));
            }
        }
        Ok(())
    }
}
