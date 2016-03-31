use std::fmt;
use std::borrow::Cow;

use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> Attribute<'a> {
    pub fn new<N, V>(name: N, value: V) -> Attribute<'a>
        where N: Into<Cow<'a, str>>,
              V: Into<Cow<'a, str>>
    {
        Attribute {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}=\"{}\"", self.name.as_ref(), Escaped(&self.value))
    }
}

#[derive(Clone, Debug)]
/// Use `attr_set!` macro instead of manual construction
pub struct AttributeSet<'a>(pub Cow<'a, [Attribute<'a>]>);

impl<'a> fmt::Display for AttributeSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in self.0.iter() {
            try!(write!(f, " {}", attr));
        }
        Ok(())
    }
}
