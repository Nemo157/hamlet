use std::fmt;
use std::borrow::Cow;

use attr::AttributeSet;
use escape::Escaped;

#[derive(Clone, Debug)]
pub enum Token<'a> {
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
    /// The text contained will be `Display`ed as-is.
    RawText(Cow<'a, str>),
}

impl<'a> Token<'a> {
    pub fn start_tag<S>(name: S, attrs: AttributeSet<'a>) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::StartTag {
            name: name.into(),
            attrs: attrs,
            self_closing: false,
        }
    }

    pub fn end_tag<S>(name: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::EndTag { name: name.into() }
    }

    pub fn text<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::Text(s.into())
    }

    pub fn raw_text<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::RawText(s.into())
    }

    /// If `self` is a `StartTag`, returns the `Token` after setting
    /// `self_closing` to `true`; otherwise, it is a no-op.
    pub fn closed(self) -> Token<'a> {
        if let Token::StartTag { name, attrs, .. } = self {
            Token::StartTag {
                name: name,
                attrs: attrs,
                self_closing: true,
            }
        } else {
            self
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::StartTag { ref name, ref attrs, self_closing } => {
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
            Token::EndTag { ref name } => write!(f, "</{}>", name),
            Token::Text(ref text) => write!(f, "{}", Escaped(text)),
            Token::RawText(ref text) => write!(f, "{}", text),
        }
    }
}
