use std::io;
use std::marker::PhantomData;
use Event;

/// Used for serializing an iterator of `Event`s to an `io::Write` stream.
pub struct HtmlStreamer<'a, I> {
    ev_iter: I,
    ev_life: PhantomData<Event<'a>>,
}

impl<'a, I> HtmlStreamer<'a, I>
    where I: Iterator<Item = Event<'a>>
{
    pub fn new<T>(events: T) -> HtmlStreamer<'a, I>
        where T: IntoIterator<IntoIter = I, Item = Event<'a>> + 'a
    {
        HtmlStreamer {
            ev_iter: events.into_iter(),
            ev_life: PhantomData,
        }
    }

    /// Start iterating over `Event`s, writing them to `w`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// use hamlet::{Event, HtmlStreamer};
    /// let events = vec![
    ///     Event::text("Hello, "),
    ///     Event::start_tag("small", attr_set!()),
    ///     Event::text("world!"),
    ///     Event::end_tag("small"),
    /// ];
    ///
    /// let mut result = Vec::new();
    /// HtmlStreamer::new(events).stream(&mut result).unwrap();
    /// let res_str = String::from_utf8(result).unwrap();
    ///
    /// assert_eq!(res_str.as_str(), "Hello, <small>world!</small>");
    /// # }
    /// ```
    pub fn stream(self, w: &mut io::Write) -> io::Result<usize> {
        for ev in self.ev_iter {
            try!(write!(w, "{}", ev));
        }
        return Ok(0);
    }
}
