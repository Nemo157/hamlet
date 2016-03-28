use std::io;
use std::io::Write;
use std::marker::PhantomData;
use Event;

pub struct ReadHtml<'a, I> where I: Iterator<Item=Event<'a>> + 'a {
    events: I,
    current: Vec<u8>,
    phantom: PhantomData<&'a I>,
}

impl<'a, I> ReadHtml<'a, I> where I: Iterator<Item=Event<'a>> {
    pub fn new<II>(events: II) -> ReadHtml<'a, I> where II: IntoIterator<IntoIter=I, Item=Event<'a>> {
        ReadHtml {
            events: events.into_iter(),
            current: Vec::new(),
            phantom: PhantomData,
        }
    }
}

impl<'a, I> io::Read for ReadHtml<'a, I> where I: Iterator<Item=Event<'a>> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while self.current.len() == 0 {
            if let Some(event) = self.events.next() {
                // Possible to do better?
                try!(write!(&mut self.current, "{}", event));
            } else {
                return Ok(0)
            }
        }
        let curlen = self.current.len();
        let len = ::std::cmp::min(curlen, buf.len());
        buf[..len].clone_from_slice(&self.current[..len]);
        for i in len..curlen {
            self.current[i - len] = self.current[i];
        }
        self.current.truncate(curlen - len);
        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use ReadHtml;

    #[test]
    fn test() {
        let events = vec![
            start_tag!("h1", attrs!["id" => "hello"]),
            text!("Hello, "),
            ::Event::RawHtml("".into()), // empty event
            start_tag!("small", attrs!["id" => "world"]),
            text!("world"),
            end_tag!("small"),
            end_tag!("h1"),
        ];

        let mut result = String::new();
        ReadHtml::new(events).read_to_string(&mut result).unwrap();

        assert_eq!(result, "<h1 id=\"hello\">Hello, <small id=\"world\">world</small></h1>");
    }
}
