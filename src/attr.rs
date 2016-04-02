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
/// let attr = hamlet::Attribute::new("id", "foo");
/// assert_eq!(format!("{}", attr), "id=\"foo\"");
/// ```
///
/// ```rust
/// let attr = hamlet::Attribute::new("id", "bar & baz");
/// assert_eq!(format!("{}", attr), "id=\"bar &amp; baz\"");
/// ```
///
/// ```rust
/// let attr = hamlet::Attribute::new("invalid=id", "foo");
/// assert_eq!(format!("{}", attr), "invalid=id=\"foo\"");
/// ```
///
/// ```rust
/// let attr = hamlet::Attribute::new("checked", "");
/// assert_eq!(format!("{}", attr), "checked");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// [`AttributeSet`](struct.AttributeSet.html#methods) or the
    /// [`attr_set!`](macro.attr_set!.html) macro.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::borrow::Cow;
    /// let foo = "foo".to_owned();
    /// let foo2 = foo.clone();
    /// assert_eq!(
    ///     hamlet::Attribute::new("id", foo),
    ///     hamlet::Attribute {
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

#[derive(Clone, Debug)]
/// A set of [`Attribute`](./struct.Attribute.html)s.
///
/// This is stored as a plain slice instead of a set as in most cases it will
/// be a small collection over which linear search will be more efficient.
///
/// Generally end users shouldn't construct this struct directly, it's expected
/// that there will be builder APIs or macros available to make construction
/// easier, such as the provided [`attr_set!`](./macro.attr_set!.html) macro.
pub struct AttributeSet<'a>(pub Cow<'a, [Attribute<'a>]>);

impl<'a> AttributeSet<'a> {
    /// Try and get the value of an attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let attrs = attr_set!(id = "foo");
    /// assert_eq!(attrs.get_value("id"), Some("foo"));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let attrs = attr_set!(id = "foo");
    /// assert_eq!(attrs.get_value("class"), None);
    /// # }
    /// ```
    pub fn get_value<S>(&self, name: S) -> Option<&str>
        where S: AsRef<str>
    {
        self.0.iter().find(|attr| attr.name == name.as_ref()).map(|a| a.value.as_ref())
    }

    /// Unconditionally set an attribute to a value. If the attribute already
    /// exists in the set will update its value, otherwise will add a new
    /// attribute to the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let mut attrs = attr_set!(id = "foo");
    ///
    /// attrs.set("id", "bar");
    ///
    /// assert_eq!(attrs.get_value("id"), Some("bar"));
    /// # }
    /// ```
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let mut attrs = attr_set!(id = "foo");
    ///
    /// attrs.set("class", "bar");
    ///
    /// assert_eq!(attrs.get_value("class"), Some("bar"));
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

    /// Removes and returns the attribute it if there was one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// let mut attrs = attr_set!(id = "foo");
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
}

impl<'a> fmt::Display for AttributeSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in self.0.iter() {
            try!(write!(f, " {}", attr));
        }
        Ok(())
    }
}
