use std::ascii::AsciiExt;
use std::borrow::Cow;

pub fn identifier_to_tag_name<'a, T>(s: T) -> Cow<'a, str>
    where T: Into<Cow<'a, str>>
{
    let s = s.into();
    if s.contains(|c: char| c.is_ascii() && c.is_uppercase()) {
        let mut result = String::with_capacity(s.len() + 4);
        for c in s.chars() {
            if c.is_ascii() && c.is_uppercase() {
                result.push('-');
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }
        Cow::Owned(result)
    } else {
        s
    }
}
