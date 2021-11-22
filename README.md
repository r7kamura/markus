# markus

[![test](https://github.com/r7kamura/markus/actions/workflows/test.yml/badge.svg)](https://github.com/r7kamura/markus/actions/workflows/test.yml)

Makdown-like text parser.

## Usage

```rust
use markus::parser::Parser;
use markus::html::push_html;

fn main() {
    let text = "abc\ndef\nghi";
    let parser = Parser::new(text);
    let mut buffer = String::new();
    push_html(&mut buffer, parser);
    assert_eq!(buffer, "<p>abc\ndef\nghi</p>\n".to_string());
}
```
