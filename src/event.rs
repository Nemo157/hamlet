use std::fmt;
use std::borrow::Cow;

use attr::AttributeSet;
use escape::Escaped;

#[derive(Clone, Debug)]
pub enum Event<'a> {
    StartTag {
        name: Cow<'a, str>,
        attrs: AttributeSet<'a>,
        /// Marker indicating self-closing tags such as `<br />`
        self_closing: bool,
    },
    EndTag {
        name: Cow<'a, str>,
    },
    /// The text contained will be escaped on `Display`.
    Text(Cow<'a, str>),
    RawHtml(Cow<'a, str>),
}

impl<'a> Event<'a> {
    pub fn start_tag<S>(name: S, attrs: AttributeSet<'a>) -> Event<'a>
        where S: Into<Cow<'a, str>>
    {
        Event::StartTag {
            name: name.into(),
            attrs: attrs,
            self_closing: false,
        }
    }

    pub fn end_tag<S>(name: S) -> Event<'a>
        where S: Into<Cow<'a, str>>
    {
        Event::EndTag { name: name.into() }
    }

    pub fn text<S>(s: S) -> Event<'a>
        where S: Into<Cow<'a, str>>
    {
        Event::Text(s.into())
    }

    pub fn raw_html<S>(s: S) -> Event<'a>
        where S: Into<Cow<'a, str>>
    {
        Event::RawHtml(s.into())
    }

    /// If `self` is a `StartTag`, returns the `Event` after setting
    /// `self_closing` to `true`; otherwise, it is a no-op.
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
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::StartTag { ref name, ref attrs, self_closing } => {
                try!(write!(f, "<{}", name));
                for attr in attrs.iter() {
                    try!(write!(f, " {}", attr));
                }
                if self_closing {
                    write!(f, " />")
                } else {
                    write!(f, ">")
                }
            }
            Event::EndTag { ref name } => write!(f, "</{}>", name),
            Event::Text(ref text) => write!(f, "{}", Escaped(text)),
            Event::RawHtml(ref html) => write!(f, "{}", html),
        }
    }
}
