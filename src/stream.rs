use std::io;
use std::io::Write;
use std::marker::PhantomData;
use Event;

pub struct HtmlStreamer<'a, I> where I: Iterator<Item=Event<'a>> + 'a {
    events: I,
    buffer: Vec<u8>,
    ev_life: PhantomData<Event<'a>>,
}

impl<'a, I> HtmlStreamer<'a, I> where I: Iterator<Item=Event<'a>> {
    pub fn new<II>(events: II) -> HtmlStreamer<'a, I> where II: IntoIterator<IntoIter=I, Item=Event<'a>> {
        HtmlStreamer {
            events: events.into_iter(),
            buffer: Vec::new(),
            ev_life: PhantomData,
        }
    }
}

impl<'a, I> io::Read for HtmlStreamer<'a, I> where I: Iterator<Item=Event<'a>> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while self.buffer.len() == 0 {
            if let Some(event) = self.events.next() {
                // Possible to do better?
                try!(write!(&mut self.buffer, "{}", event));
            } else {
                return Ok(0)
            }
        }
        let curlen = self.buffer.len();
        let len = ::std::cmp::min(curlen, buf.len());
        buf[..len].clone_from_slice(&self.buffer[..len]);
        for i in len..curlen {
            self.buffer[i - len] = self.buffer[i];
        }
        self.buffer.truncate(curlen - len);
        Ok(len)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let start = buf.len();
        while let Some(event) = self.events.next() {
            try!(write!(buf, "{}", event));
        }
        Ok(buf.len() - start)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
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
            closed_tag!("img", src="foo-link"),
            end_tag!("h1"),
        ];

        let mut result = String::new();
        HtmlStreamer::new(events).read_to_string(&mut result).unwrap();

        assert_eq!(result, "<h1 id=\"hello\" class=\"fun\">Hello, \
                            <small>world</small><img src=\"foo-link\" /></h1>");
    }
}
