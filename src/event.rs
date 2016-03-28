use std::fmt;
use std::borrow::Cow;

use Attribute;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event<'a> {
    StartTag {
        name: Cow<'a, str>,
        attrs: Cow<'a, [Attribute<'a>]>,
    },
    EndTag {
        name: Cow<'a, str>,
    },
    ClosedTag {
        name: Cow<'a, str>,
        attrs: Cow<'a, [Attribute<'a>]>,
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

impl<'a> Event<'a> {
    pub fn utf8_len(&self) -> usize {
        match *self {
            Event::StartTag { ref name, ref attrs } => {
                let mut len = name.len() + 2;
                for attr in attrs.iter() {
                    len += attr.utf8_len() + 1;
                }
                len
            }
            Event::ClosedTag { ref name, ref attrs } => {
                let mut len = name.len() + 4;
                for attr in attrs.iter() {
                    len += attr.utf8_len() + 1;
                }
                len
            }
            Event::EndTag { ref name } => {
                name.len() + 3
            }
            Event::Text(ref text) => {
                text.len()
            }
            Event::RawHtml(ref html) => {
                html.len()
            }
        }
    }
}
