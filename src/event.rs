use std::fmt;
use std::borrow::Cow;

use attribute::AttributeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event<'a> {
    StartTag {
        name: Cow<'a, str>,
        attrs: AttributeSet<'a>,
    },
    EndTag {
        name: Cow<'a, str>,
    },
    ClosedTag {
        name: Cow<'a, str>,
        attrs: AttributeSet<'a>,
    },
    Text(Cow<'a, str>),
    RawHtml(Cow<'a, str>),
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::StartTag { ref name, ref attrs } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, " {}", attr));
                }
                write!(f, ">")
            }
            Event::ClosedTag { ref name, ref attrs } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, " {}", attr));
                }
                write!(f, " />")
            }
            Event::EndTag { ref name } => write!(f, "</{}>", name),
            Event::Text(ref text) => {
                // TODO: Escaping
                write!(f, "{}", text)
            }
            Event::RawHtml(ref html) => write!(f, "{}", html),
        }
    }
}
