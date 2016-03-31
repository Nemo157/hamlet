#[macro_export]
macro_rules! attr_set {
    () => {
        $crate::AttributeSet(::std::borrow::Cow::Borrowed(&[]))
    };
    ($($name:ident = $value:expr),+) => {
        ($crate::AttributeSet(::std::borrow::Cow::Owned(vec![
            $($crate::Attribute::new(stringify!($name), $value)),+
        ])))
    };
}
