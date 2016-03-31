#[macro_export]
macro_rules! attrs {
    ($($name:ident = $value:expr),+) => {
        ($crate::AttributeSet(::std::borrow::Cow::Owned(vec![
            $($crate::Attribute::new(stringify!($name), $value)),+
        ])))
    }
}

#[macro_export]
macro_rules! start_tag {
    ($name:expr) => {
        ($crate::Event::StartTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $crate::AttributeSet::empty(),
            self_closing: false,
        })
    };
    ($name:expr, $($aname:ident = $aval:expr),+) => {
        start_tag!($name).with_attrs(attrs!($($aname = $aval),+))
    };
}

#[macro_export]
macro_rules! end_tag {
    ($name:expr) => {
        ($crate::Event::EndTag {
            name: ::std::borrow::Cow::from($name),
        })
    };
}

#[macro_export]
macro_rules! text {
    ($text:expr) => {
        ($crate::Event::Text(::std::borrow::Cow::from($text)))
    };
}

#[macro_export]
macro_rules! raw_html {
    ($html:expr) => {
        ($crate::Event::RawHtml(::std::borrow::Cow::from($html)))
    };
}
