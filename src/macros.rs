#[macro_export]
/// doc which explains: `dataAttrName` will become `data-attr-name`
macro_rules! attr_set {
    () => {
        $crate::AttributeSet(::std::borrow::Cow::Borrowed(&[]))
    };
    ($($name:ident = $value:expr),+) => {
        ($crate::AttributeSet(::std::borrow::Cow::Owned(vec![
            $($crate::Attribute::new($crate::util::identifier_to_tag_name(stringify!($name)), $value)),+
        ])))
    };
}
