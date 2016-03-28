use std::fmt;
use Attribute;
use Event;

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Escaping
        write!(f, " {}=\"{}\"", self.name, self.value)
    }
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::StartTag { ref name, ref attrs } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, "{}", attr));
                }
                write!(f, ">")
            }
            Event::ClosedTag { ref name, ref attrs } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, "{}", attr));
                }
                write!(f, " />")
            }
            Event::EndTag { ref name } => {
                write!(f, "</{}>", name)
            }
            Event::Text(ref text) => {
                // TODO: Escaping
                write!(f, "{}", text)
            }
            Event::RawHtml(ref html) => {
                write!(f, "{}", html)
            }
        }
    }
}
