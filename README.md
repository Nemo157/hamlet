# hamlet

Provides token definitions for HTML stream processing. The goal of this library
is to provide a simple API over which higher abstraction can be built on.

## Example

```rust
#[macro_use]
extern crate hamlet;

use std::fmt::Write;

fn main() {
    use hamlet::Token;
    let tokens = vec![
        Token::text("Hello, "),
        Token::start_tag("small", attrs!(class="foo")),
        Token::text("world!"),
        Token::end_tag("small"),
    ];

    let mut html = String::from("");
    for token in tokens {
        write!(html, "{}", token);
    }

    assert_eq!(html, "Hello, <small class=\"foo\">world!</small>");
}
```

## Links

 * Homepage: https://nemo157.com/hamlet
 * Official Repository: https://github.com/Nemo157/hamlet

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
