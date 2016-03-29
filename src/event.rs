use std::fmt;
use std::borrow::Cow;

use attribute::AttributeSet;
use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event<'a> {
    StartTag {
        name: Cow<'a, str>,
        attrs: AttributeSet<'a>,
        is_self_closing: bool,
    },
    EndTag {
        name: Cow<'a, str>,
    },
    Text(Cow<'a, str>),
    RawHtml(Cow<'a, str>),
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::StartTag { ref name, ref attrs, is_self_closing } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, " {}", attr));
                }
                if is_self_closing {
                  try!(write!(f, " /"));
                }
                write!(f, ">")
            }
            Event::EndTag { ref name } => write!(f, "</{}>", name),
            Event::Text(ref text) => write!(f, "{}", Escaped(text)),
            Event::RawHtml(ref html) => write!(f, "{}", html),
        }
    }
}
