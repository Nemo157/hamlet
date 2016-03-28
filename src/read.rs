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
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.current.len() > 0 {
            if self.current.len() < buf.len() {
                let len = self.current.len();
                buf[..len].clone_from_slice(&self.current[..]);
                self.current.clear();
                Ok(len)
            } else {
                let leftover = self.current.split_off(buf.len());
                buf.clone_from_slice(&self.current[..]);
                self.current.clear();
                self.current.extend_from_slice(&leftover[..]);
                Ok(buf.len())
            }
        } else if let Some(next) = self.events.next() {
            try!(write!(&mut self.current, "{}", next));
            self.read(buf)
        } else {
            Ok(0)
        }
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
