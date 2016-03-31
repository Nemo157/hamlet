use std::fmt;
use std::borrow::Cow;

use AttributeSet;
use escape::Escaped;

#[derive(Clone, Debug)]
pub enum Event<'a> {
    StartTag {
        name: Cow<'a, str>,
        attrs: AttributeSet<'a>,
        self_closing: bool,
    },
    EndTag {
        name: Cow<'a, str>,
    },
    Text(Cow<'a, str>),
    RawHtml(Cow<'a, str>),
}

impl<'a> Event<'a> {
    pub fn closed(self) -> Event<'a> {
        if let Event::StartTag { name, attrs, .. } = self {
            Event::StartTag {
                name: name,
                attrs: attrs,
                self_closing: true,
            }
        } else {
            self
        }
    }

    pub fn with_attrs(self, attrs: AttributeSet<'a>) -> Event<'a> {
        if let Event::StartTag { name, self_closing, .. } = self {
            Event::StartTag {
                name: name,
                attrs: attrs.into(),
                self_closing: self_closing,
            }
        } else {
            self
        }
    }
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::StartTag { ref name, ref attrs, self_closing } => {
                if self_closing {
                    write!(f, "<{}{} />", name, attrs)
                } else {
                    write!(f, "<{}{}>", name, attrs)
                }
            }
            Event::EndTag { ref name } => write!(f, "</{}>", name),
            Event::Text(ref text) => write!(f, "{}", Escaped(text)),
            Event::RawHtml(ref html) => write!(f, "{}", html),
        }
    }
}
