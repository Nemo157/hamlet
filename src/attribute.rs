use std::borrow::Cow;

static EMPTY_ATTRS: &'static [Attribute<'static>] = &[];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
  pub name: Cow<'a, str>,
  pub value: Cow<'a, str>,
}

impl<'a> Attribute<'a> {
  pub fn none() -> Cow<'a, [Attribute<'a>]> {
    Cow::Borrowed(EMPTY_ATTRS)
  }
}
