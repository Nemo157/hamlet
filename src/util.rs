use std::borrow::Cow;

pub fn identifier_to_tag_name<'a, T>(s: T) -> Cow<'a, str>
    where T: Into<Cow<'a, str>>
{
    let mut owned: Option<String> = None;
    let s = s.into();
    {
        let bs: &str = s.as_ref();
        for (i, c) in bs.chars().enumerate() {
            match c {
                c @ 'A' ... 'Z' => {
                    let mut os = owned.unwrap_or({
                        let mut os = String::with_capacity(bs.len() + 4);
                        os.extend((&bs[..i]).chars());
                        os
                    });
                    os.push('-');
                    os.push(c.to_lowercase().next().unwrap());
                    owned = Some(os);
                }
                _ => if let Some(ref mut os) = owned {
                    os.push(c);
                }
            }
        }
    }
    owned.map(|os| os.into()).unwrap_or(s)
}
