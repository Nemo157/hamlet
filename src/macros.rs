#[macro_export]
/// A convenience macro for `AttributeSet` construction. Attributes with hyphens
/// should be camel-cased.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate hamlet;
/// # fn main() {
/// let attrs = attr_set!(dataFoo = "bar");
/// assert_eq!(attrs.get_attr("data-foo"),
///            Some(&hamlet::Attribute::new("data-foo", "bar")));
/// # }
/// ```
macro_rules! attr_set {
    () => {
        $crate::AttributeSet(::std::borrow::Cow::Borrowed(&[]))
    };
    ($($name:ident = $value:expr),+) => {
        $crate::AttributeSet(::std::borrow::Cow::Owned(vec![
            $($crate::Attribute::new($crate::util::identifier_to_tag_name(stringify!($name)), $value)),+
        ]))
    };
}
