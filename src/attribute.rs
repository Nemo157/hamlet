use std::fmt;
use std::borrow::Cow;

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
        // TODO: Escaping
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}
