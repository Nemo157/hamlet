#[macro_export]
macro_rules! start_tag {
    ($name:expr) => {
        ($crate::Event::StartTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $crate::Attribute::none(),
        })
    };
    ($name:expr, $attrs:expr) => {
        ($crate::Event::StartTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $attrs,
        })
    };
}

#[macro_export]
macro_rules! closed_tag {
    ($name:expr) => {
        ($crate::Event::ClosedTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $crate::Attribute::none(),
        })
    };
    ($name:expr, $attrs:expr) => {
        ($crate::Event::ClosedTag {
            name: ::std::borrow::Cow::from($name),
            attrs: $attrs,
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

#[macro_export]
macro_rules! attr {
    ($name:expr => $value:expr) => {
        ($crate::Attribute {
            name: ::std::borrow::Cow::from($name),
            value: ::std::borrow::Cow::from($value),
        })
    }
}

#[macro_export]
macro_rules! attrs {
    ($($name:expr => $value:expr),+) => {
        (::std::borrow::Cow::Owned(vec![
            $(attr!($name => $value)),+
        ]))
    }
}
