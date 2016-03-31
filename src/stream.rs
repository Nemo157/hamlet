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
    use Event;

    #[test]
    fn test() {
        let events = vec![
            Event::start_tag("h1", attr_set!(id="hello", class="fun")),
            Event::text("Hello, "),
            Event::raw_html(""), // empty event
            Event::start_tag("small", attr_set!()),
            Event::text("world"),
            Event::end_tag("small"),
            Event::start_tag("img", attr_set!(src="foo-link")).closed(),
            Event::start_tag("br", attr_set!(dataAttr="'1'")).closed(),
            Event::end_tag("h1"),
        ];

        let mut result = Vec::new();
        HtmlStreamer::new(events).stream(&mut result).unwrap();
        let res_str = String::from_utf8(result).unwrap();

        assert_eq!(res_str.as_str(),
                   "<h1 id=\"hello\" class=\"fun\">Hello, \
                    <small>world</small><img src=\"foo-link\" />\
                    <br data-attr=\"&apos;1&apos;\" /></h1>");
    }
}
