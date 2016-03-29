use std::io;
use std::marker::PhantomData;
use Event;

pub struct HtmlStreamer<'a, I, J>
    where I: IntoIterator<IntoIter = J, Item = Event<'a>> + 'a,
          J: Iterator<Item = Event<'a>>
{
    events: I,
    ev_life: PhantomData<Event<'a>>,
}

impl<'a, I, J> HtmlStreamer<'a, I, J>
    where I: IntoIterator<IntoIter = J, Item = Event<'a>> + 'a,
          J: Iterator<Item = Event<'a>>
{
    pub fn new(events: I) -> HtmlStreamer<'a, I, J> {
        HtmlStreamer {
            events: events,
            ev_life: PhantomData,
        }
    }

    pub fn stream(self, w: &mut io::Write) -> io::Result<usize> {
        for ev in self.events.into_iter() {
            try!(write!(w, "{}", ev));
        }
        return Ok(0);
    }
}

#[cfg(test)]
mod tests {
    use HtmlStreamer;

    #[test]
    fn test() {
        let events = vec![
            start_tag!("h1", id="hello", class="fun"),
            text!("Hello, "),
            raw_html!(""), // empty event
            start_tag!("small"),
            text!("world"),
            end_tag!("small"),
            start_tag!("img", src="foo-link").closed(),
            end_tag!("h1"),
        ];

        let mut result = Vec::new();
        HtmlStreamer::new(events).stream(&mut result).unwrap();
        let res_str = String::from_utf8(result).unwrap();

        assert_eq!(res_str.as_str(),
                   "<h1 id=\"hello\" class=\"fun\">Hello, \
                    <small>world</small><img src=\"foo-link\" /></h1>");
    }
}
