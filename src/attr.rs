use std::fmt;
use std::borrow::Cow;

use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

#[derive(Clone, Debug)]
pub struct AttributeSet<'a> {
    pub attrs: Cow<'a, [Attribute<'a>]>,
}

impl<'a> AttributeSet<'a> {
    pub fn empty() -> AttributeSet<'a> {
        AttributeSet::new(Cow::Borrowed(&[]))
    }

    pub fn new(attrs: Cow<'a, [Attribute<'a>]>) -> AttributeSet<'a> {
        AttributeSet { attrs: attrs }
    }
}

impl<'a, T> From<T> for AttributeSet<'a>
    where T: Into<Cow<'a, [Attribute<'a>]>>
{
    fn from(attrs: T) -> AttributeSet<'a> {
        AttributeSet::new(attrs.into())
    }
}

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO handle invalid attribute names
        write!(f, "{}=\"{}\"", self.name, Escaped(&self.value))
    }
}

impl<'a> fmt::Display for AttributeSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in self.attrs.iter() {
            try!(write!(f, " {}", attr));
        }
        Ok(())
    }
}
