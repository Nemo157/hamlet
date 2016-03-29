use std::fmt;
use std::borrow::Cow;

use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO handle invalid attribute names
        write!(f, "{}=\"{}\"", self.name, Escaped(&self.value))
    }
}
