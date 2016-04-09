use std::fmt;
use std::borrow::Cow;

use escape::Escaped;

/// An [HTML attribute](https://www.w3.org/TR/html/syntax.html#attributes-0).
///
/// The name for the attribute will not be validated, you must ensure it meets
/// the requirements specified in the spec yourself.
///
/// The value for the attribute will be escaped automatically. If it is an
/// empty string then the attribute will be written with the 'Empty attribute
/// syntax'.
///
/// # Examples
///
/// ```rust
/// let attr = hamlet::attr::Attribute::new("id", "foo");
/// assert_eq!(format!("{}", attr), "id=\"foo\"");
/// ```
///
/// ```rust
/// let attr = hamlet::attr::Attribute::new("id", "bar & baz");
/// assert_eq!(format!("{}", attr), "id=\"bar &amp; baz\"");
/// ```
///
/// ```rust
/// let attr = hamlet::attr::Attribute::new("invalid=id", "foo");
/// assert_eq!(format!("{}", attr), "invalid=id=\"foo\"");
/// ```
///
/// ```rust
/// let attr = hamlet::attr::Attribute::new("checked", "");
/// assert_eq!(format!("{}", attr), "checked");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> Attribute<'a> {
    /// Create an attribute, useful to avoid having to convert strings to
    /// `Cow<str>` yourself.
    ///
    /// Generally this shouldn't be used directly by end users, it's likely
    /// that there are builder APIs or macros available that make attribute
    /// construction easier, for example the modification methods on
    /// [`AttributeList`](struct.AttributeList.html#methods) or the
    /// [`attrs!`](macro.attrs!.html) macro.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::borrow::Cow;
    /// use hamlet::attr::Attribute;
    ///
    /// let foo = "foo".to_owned();
    /// let foo2 = foo.clone();
    /// assert_eq!(
    ///     Attribute::new("id", foo),
    ///     Attribute {
    ///         name: Cow::Borrowed("id"),
    ///         value: Cow::Owned(foo2),
    ///     });
    /// ```
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

#[derive(Clone, Debug, Eq)]
/// A list of [`Attribute`](./struct.Attribute.html)s.
///
/// This is stored as a plain list instead of a set as in most cases it will
/// be a small collection over which a linear search will be more efficient.
pub struct AttributeList<'a>(Cow<'a, [Attribute<'a>]>);

impl<'a> AttributeList<'a> {
    /// Return an empty `AttributeList`
    pub fn empty() -> AttributeList<'a> {
        AttributeList(Cow::Borrowed(&[]))
    }

    /// Note that this does not check for duplicate attribute names. Generally,
    /// end users are not expected to call this method, and instead use
    /// high-level builder APIs or macros available to make construction easier,
    /// such as the provided [`attrs!`](./macro.attrs!.html) macro.
    pub fn from_vec(attrs: Vec<Attribute<'a>>) -> AttributeList<'a> {
        AttributeList(Cow::Owned(attrs))
    }

    /// Pull all attributes out of this collection, useful if you need to
    /// perform some more extensive modification.
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// use hamlet::attr::{ Attribute, AttributeList };
    /// let attrs = attrs!(dataBar = "bar", dataBaz = "baz");
    ///
    /// // Namespace all data attributes for some reason.
    /// let attrs = AttributeList::from_vec(
    ///     attrs.into_vec().into_iter()
    ///         .map(|Attribute { name, value }| {
    ///             Attribute::new(name.replace("data-", "data-foo-"), value)
    ///         })
    ///         .collect());
    ///
    /// assert_eq!(attrs.get("data-foo-bar"), Some("bar"));
    /// # }
    /// ```
    pub fn into_vec(self) -> Vec<Attribute<'a>> {
        self.0.into_owned()
    }

    /// Try and get the value of an attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let attrs = attrs!(id = "foo");
    /// assert_eq!(attrs.get("id"), Some("foo"));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let attrs = attrs!(id = "foo");
    /// assert_eq!(attrs.get("class"), None);
    /// # }
    /// ```
    pub fn get<S>(&self, name: S) -> Option<&str>
        where S: AsRef<str>
    {
        self.0.iter().find(|attr| attr.name == name.as_ref()).map(|a| a.value.as_ref())
    }

    /// Unconditionally set an attribute to a value. If the attribute already
    /// exists in the list, update its value, otherwise add a new attribute to
    /// the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let mut attrs = attrs!(id = "foo");
    ///
    /// attrs.set("id", "bar");
    ///
    /// assert_eq!(attrs.get("id"), Some("bar"));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let mut attrs = attrs!(id = "foo");
    ///
    /// attrs.set("class", "bar");
    ///
    /// assert_eq!(attrs.get("class"), Some("bar"));
    /// # }
    /// ```
    pub fn set<N, V>(&mut self, name: N, value: V)
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

    /// Removes and returns the attribute if there was one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let mut attrs = attrs!(id = "foo");
    ///
    /// assert_eq!(attrs.remove("id").map(|a| a.value).unwrap().as_ref(), "foo");
    /// # }
    /// ```
    pub fn remove<S>(&mut self, name: S) -> Option<Attribute<'a>>
        where S: AsRef<str>
    {
        let attrs = self.0.to_mut();

        if let Some(pos) = attrs.iter().position(|attr| attr.name.as_ref() == name.as_ref()) {
            Some(attrs.remove(pos))
        } else {
            None
        }
    }

    /// Returns an iterator over the list.
    pub fn iter<'b>(&'b self) -> Iter<'b, 'a> {
        Iter {
            inner: self.0.as_ref(),
            index: 0,
        }
    }
}

/// Immutable [`AttributeList`](./struct.AttributeList.html) iterator.
pub struct Iter<'b, 'a: 'b> {
    inner: &'b [Attribute<'a>],
    index: usize,
}

impl<'a, 'b> Iterator for Iter<'b, 'a> {
    type Item = &'b Attribute<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.inner.get(self.index - 1)
    }
}

impl<'a, 'b> PartialEq<AttributeList<'b>> for AttributeList<'a> {
    fn eq(&self, other: &AttributeList<'b>) -> bool {
        let mut left = self.0.iter().collect::<Vec<_>>();
        let mut right = other.0.iter().collect::<Vec<_>>();
        left.sort();
        right.sort();
        left == right
    }
}
