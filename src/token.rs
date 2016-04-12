use std::fmt;
use std::borrow::Cow;

use attr::AttributeList;
use escape::Escaped;

#[derive(Clone, Debug, PartialEq, Eq)]
/// An HTML token, these are representations of everything needed to generate
/// an [HTML document](https://www.w3.org/TR/html/syntax.html#writing).
///
/// By convention, [`Token::Text`](#variant.Text) should be preferred over
/// [`Token::RawText`](#variant.RawText) when a piece of text can be
/// represented by both. For instance, use `Text` when tokenizing whitespaces
/// or line-breaks, but use `RawText` for representing all text inside
/// a `<style>` tag.
///
/// When `Display`ing a `Token`, the output stream is assumed to be Unicode, and
/// therefore only five characters are escaped: `&`, `<`, `>`, `"`, and `'`
/// ([ref](http://stackoverflow.com/a/7382028)).
pub enum Token<'a> {
    /// A [start tag](https://www.w3.org/TR/html/syntax.html#syntax-start-tag)
    /// token.
    StartTag {
        /// The element's [tag
        /// name](https://www.w3.org/TR/html/syntax.html#syntax-tag-name).
        name: Cow<'a, str>,

        /// Any attributes for the start tag.
        attrs: AttributeList<'a>,

        /// Marker indicating the tag should be self-closing, such as `<br />`
        /// (although `br` is a [void
        /// element](https://www.w3.org/TR/html/syntax.html#void-elements) so
        /// this has no effect on it).
        self_closing: bool,
    },

    /// An [end tag](https://www.w3.org/TR/html/syntax.html#syntax-end-tag)
    /// token.
    EndTag {
        /// The element's [tag
        /// name](https://www.w3.org/TR/html/syntax.html#syntax-tag-name).
        name: Cow<'a, str>,
    },

    /// The text contained will be escaped on `Display`.
    Text(Cow<'a, str>),

    /// The text contained will be `Display`ed as-is.
    RawText(Cow<'a, str>),

    /// Comments contained within `<!--` and `-->`. No validation is done to
    /// ensure that the text conforms to the [html comment
    /// syntax](https://www.w3.org/TR/html/syntax.html#syntax-comments).
    Comment(Cow<'a, str>),

    /// The [HTML5 DOCTYPE
    /// declaration](https://www.w3.org/TR/html/syntax.html#syntax-doctype)
    /// (`<!DOCTYPE html>`)
    DOCTYPE,
}

impl<'a> Token<'a> {
    /// Create a [`StartTag`](#variant.StartTag) token with specified element
    /// name and attributes, use [`closed()`](#method.closed) to set the
    /// [`self_closing`](#variant.StartTag.field.self_closing) flag.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// assert_eq!(
    ///     hamlet::Token::start_tag("script", attrs!()),
    ///     hamlet::Token::StartTag {
    ///         name: std::borrow::Cow::Borrowed("script"),
    ///         attrs: attrs!(),
    ///         self_closing: false,
    ///     });
    /// # }
    /// ```
    pub fn start_tag<S>(name: S, attrs: AttributeList<'a>) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::StartTag {
            name: name.into(),
            attrs: attrs,
            self_closing: false,
        }
    }

    /// Create an [`EndTag`](#variant.EndTag) token with specified element
    /// name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     hamlet::Token::end_tag("script"),
    ///     hamlet::Token::EndTag {
    ///         name: std::borrow::Cow::Borrowed("script"),
    ///     });
    /// ```
    pub fn end_tag<S>(name: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::EndTag { name: name.into() }
    }

    /// Create a [`Text`](#variant.Text) token with specified text content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     hamlet::Token::text("hello world"),
    ///     hamlet::Token::Text(std::borrow::Cow::Borrowed("hello world")));
    /// ```
    pub fn text<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::Text(s.into())
    }


    /// Create a [`RawText`](#variant.RawText) token with specified raw text
    /// content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     hamlet::Token::raw_text("hello world"),
    ///     hamlet::Token::RawText(std::borrow::Cow::Borrowed("hello world")));
    /// ```
    pub fn raw_text<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::RawText(s.into())
    }

    /// Create a [`Comment`](#variant.Comment) token with specified comment
    /// content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     hamlet::Token::comment("hello world"),
    ///     hamlet::Token::Comment(std::borrow::Cow::Borrowed("hello world")));
    /// ```
    pub fn comment<S>(s: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token::Comment(s.into())
    }

    /// If `self` is a [`StartTag`](#variant.StartTag), returns a copy with
    /// [`self_closing`](#variant.StartTag.field.self_closing) set to `true`;
    /// otherwise, returns `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate hamlet;
    /// # fn main() {
    /// assert_eq!(
    ///     hamlet::Token::start_tag("br", attrs!()).closed(),
    ///     hamlet::Token::StartTag {
    ///         name: std::borrow::Cow::Borrowed("br"),
    ///         attrs: attrs!(),
    ///         self_closing: true,
    ///     });
    /// # }
    /// ```
    ///
    /// ```rust
    /// assert_eq!(
    ///     hamlet::Token::text("hello world").closed(),
    ///     hamlet::Token::Text(std::borrow::Cow::Borrowed("hello world")));
    /// ```
    pub fn closed(self) -> Token<'a> {
        if let Token::StartTag { name, attrs, .. } = self {
            Token::StartTag {
                name: name,
                attrs: attrs,
                self_closing: true,
            }
        } else {
            self
        }
    }

    pub fn write_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match *self {
            Token::StartTag { ref name, ref attrs, self_closing } => {
                try!(w.write_str("<"));
                try!(w.write_str(name));
                for attr in attrs.iter() {
                    try!(w.write_str(" "));
                    try!(attr.write_to(w));
                }
                if self_closing {
                    w.write_str(" />")
                } else {
                    w.write_str(">")
                }
            }
            Token::EndTag { ref name } => {
                try!(w.write_str("</"));
                try!(w.write_str(name));
                w.write_str(">")
            }
            Token::Text(ref text) => {
                Escaped(text).write_to(w)
            }
            Token::RawText(ref text) => {
                w.write_str(text)
            }
            Token::Comment(ref text) => {
                try!(w.write_str("<!--"));
                try!(w.write_str(text));
                w.write_str("-->")
            }
            Token::DOCTYPE => {
                w.write_str("<!DOCTYPE html>")
            }
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_to(f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_variants() {
        use Token;
        use attr::AttributeList;

        let start_tag = Token::start_tag("tag", AttributeList::empty());
        assert_eq!(format!("{}", start_tag), "<tag>");

        let end_tag = Token::end_tag("tag");
        assert_eq!(format!("{}", end_tag), "</tag>");

        let text = Token::text("<bomb>");
        assert_eq!(format!("{}", text), "&lt;bomb&gt;");

        let raw_text = Token::raw_text("<bomb>");
        assert_eq!(format!("{}", raw_text), "<bomb>");

        let comment = Token::comment("Multi\nline\ncomment");
        assert_eq!(format!("{}", comment), "<!--Multi\nline\ncomment-->");

        assert_eq!(format!("{}", Token::DOCTYPE), "<!DOCTYPE html>");
    }
}
