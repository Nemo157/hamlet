use std::borrow::Cow;

use Attribute;

#[derive(Clone)]
pub enum Event<'a> {
  StartTag {
    name: Cow<'a, str>,
    attrs: Cow<'a, [Attribute<'a>]>,
  },
  EndTag {
    name: Cow<'a, str>,
  },
  ClosedTag {
    name: Cow<'a, str>,
    attrs: Cow<'a, [Attribute<'a>]>,
  },
  Text(Cow<'a, str>),
  RawHtml(Cow<'a, str>),
}
