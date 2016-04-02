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
        if self.value == "" {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}=\"{}\"", self.name.as_ref(), Escaped(&self.value))
        }
    }
}

#[derive(Clone, Debug)]
/// Prefer `attr_set!` macro over manual construction
pub struct AttributeSet<'a>(pub Cow<'a, [Attribute<'a>]>);

impl<'a> AttributeSet<'a> {
    pub fn get_attr<S>(&self, name: S) -> Option<&Attribute<'a>>
        where S: AsRef<str>
    {
        self.0.iter().find(|attr| attr.name == name.as_ref())
    }

    pub fn get_attr_mut<S>(&mut self, name: S) -> Option<&mut Attribute<'a>>
        where S: AsRef<str>
    {
        self.0.to_mut().iter_mut().find(|attr| attr.name == name.as_ref())
    }

    pub fn set_attr<N, V>(&mut self, name: N, value: V)
        where N: Into<Cow<'a, str>>,
              V: Into<Cow<'a, str>>
    {
        let (name, value) = (name.into(), value.into());
        let attrs = self.0.to_mut();

        if let Some(pos) = attrs.iter().position(|attr| attr.name == name) {
            attrs[pos].value = value;
        } else {
            attrs.push(Attribute::new(name, value));
        }
    }
}

impl<'a> fmt::Display for AttributeSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in self.0.iter() {
            try!(write!(f, " {}", attr));
        }
        Ok(())
    }
}
