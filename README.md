# hamlet

A low-level html event stream definition. The purpose of this library is to
provide a simple API over which higher abstraction can be built on.

## Example

```rust
#[macro_use]
extern crate hamlet;

use std::io;

fn main() {
    use hamlet::{Event, HtmlStreamer};
    let events = vec![
        Event::text("Hello, "),
        Event::start_tag("small", attr_set!(class="foo")),
        Event::text("world!"),
        Event::end_tag("small"),
    ];

    let ev_iter = events.into_iter(); // .map( ... ) ...

    HtmlStreamer::new(ev_iter).stream(&mut io::stdout()).unwrap();
    // will output: "Hello, <small class=\"foo\">world!</small>"
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
