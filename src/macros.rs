macro_rules! attrs {
    ($($name:ident = $value:expr),+) => {
        (::std::borrow::Cow::Owned(vec![
            $($crate::Attribute {
                name: ::std::borrow::Cow::from(stringify!($name)),
                value: ::std::borrow::Cow::from($value),
            }),+
        ]))
    }
}

#[macro_export]
macro_rules! start_tag {
    ($name:expr) => {
        ($crate::Event::StartTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $crate::attribute::empty_set(),
        })
    };
    ($name:expr, $($aname:ident = $aval:expr),+) => {
        ($crate::Event::StartTag {
            name: ::std::borrow::Cow::from($name),
            attrs: attrs!($($aname = $aval),+),
        })
    };
}

#[macro_export]
macro_rules! closed_tag {
    ($name:expr) => {
        ($crate::Event::ClosedTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $crate::attribute::empty_set(),
        })
    };
    ($name:expr, $($aname:ident = $aval:expr),+) => {
        ($crate::Event::ClosedTag {
            name: ::std::borrow::Cow::from($name),
            attrs: attrs!($($aname = $aval),+),
        })
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
