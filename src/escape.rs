use std::fmt;

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

impl<T: AsRef<str>> fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut last = 0;
        let s = self.0.as_ref();
        for (i, c) in s.chars().enumerate() {
            if let Some(esc) = escape_char(c) {
                try!(f.write_str(&s[last..i]));
                try!(f.write_str(&esc));
                last = i + 1;
            }
        }
        f.write_str(&s[last..s.len()])
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
