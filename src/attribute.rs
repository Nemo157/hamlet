use std::fmt;
use std::borrow::Cow;

use escape::Escaped;

pub type AttributeSet<'a> = Cow<'a, [Attribute<'a>]>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

pub fn empty_set<'a>() -> AttributeSet<'a> {
    Cow::Borrowed(&[])
}

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO handle invalid attribute names
        write!(f, "{}=\"{}\"", self.name, Escaped(&self.value))
    }
}
