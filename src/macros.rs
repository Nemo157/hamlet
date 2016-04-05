#[macro_export]
/// A convenience macro for `AttributeSet` construction. It does not check for
/// duplicates in attribute names. Attribute names with hyphens should be
/// camel-cased.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate hamlet;
/// # fn main() {
/// let attrs = attr_set!(dataFoo = "bar");
/// assert_eq!(attrs.get("data-foo"), Some("bar"));
/// # }
/// ```
macro_rules! attr_set {
    () => {
        $crate::attr::AttributeSet::empty()
    };
    ($($name:ident = $value:expr),+) => {
        $crate::attr::AttributeSet::from_vec(vec![
            $($crate::attr::Attribute::new($crate::util::identifier_to_tag_name(stringify!($name)), $value)),+
        ])
    };
}
