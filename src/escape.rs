use std::fmt;

#[inline]
fn escape_char<'a>(c: char) -> Option<&'a str> {
    match c {
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#39;"),
        '&' => Some("&amp;"),
        _ => None,
    }
}

pub struct Escaped<T>(pub T);

impl<T: AsRef<str>> Escaped<T> {
    #[inline]
    pub fn write_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        let s = self.0.as_ref();
        let mut last = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(esc) = escape_char(c) {
                try!(w.write_str(&s[last..i]));
                try!(w.write_str(&esc));
                last = i + 1;
            }
        }
        w.write_str(&s[last..s.len()])
    }
}

impl<T: AsRef<str>> fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_to(f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use std::fmt::Write;
        let s = "escape 'me' & \"you\"";
        let mut es = String::new();
        write!(es, "{}", super::Escaped(&s)).unwrap();
        assert_eq!(es, "escape &#39;me&#39; &amp; &quot;you&quot;");
    }
}
