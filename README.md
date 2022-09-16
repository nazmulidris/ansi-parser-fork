[![pipeline status](https://img.shields.io/gitlab/pipeline/gitlab-org/gitlab-ce.svg)](https://gitlab.com/davidbittner/ansi-parser/pipelines?ref=master)
[![docs](https://docs.rs/ansi-parser/badge.svg?version=0.6.1)](https://docs.rs/ansi-parser/)
[![deps](https://img.shields.io/librariesio/release/cargo/ansi-parser.svg)](https://gitlab.com/davidbittner/ansi-parser/blob/master/Cargo.toml)
[![license](https://img.shields.io/crates/l/ansi-parser.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
[![downloads](https://img.shields.io/crates/d/ansi-parser.svg)]()

# Ansi Escape Sequence Parser

For a complete list of implemented sequences, see the [documentation](https://docs.rs/ansi-parser).

This is done through a pulldown type parser, where an iterator is exposed. This essentially
turns all of the ANSI sequences into enums and splits the string at every location that there
was an ANSI Sequence.

Example:

```rust
use ansi_parser::{Output, AnsiParser};
use ansi_parser::AnsiSequence;

fn main() {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.
    let parsed: Vec<Output> = "This is \u{1b}[3Asome text!"
        .ansi_parse()
        .take(2)
        .collect();

    assert_eq!(
        vec![
            Output::TextBlock("This is "),
            Output::Escape(AnsiSequence::CursorUp(3))
        ],
        parsed
    );

    for block in parsed.into_iter() {
        match block {
            Output::TextBlock(text) => println!("{}", text),
            Output::Escape(seq)     => println!("{}", seq)
        }
    }
}
```

# `no_std` support

`no_std` is supported via disabling the `std` feature in your `Cargo.toml`.
