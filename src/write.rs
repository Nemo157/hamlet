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
    fn write_events<'a, I>(&mut self, events: I) -> Result<(), Error>
        where I: IntoIterator<Item = Event<'a>>
    {
        for event in events {
            try!(self.write_event(event));
        }
        Ok(())
    }
}

impl<T: fmt::Write> Write for T {
    fn write_event(&mut self, event: Event) -> Result<(), Error> {
        try!(write!(self, "{}", event));
        Ok(())
    }
}
