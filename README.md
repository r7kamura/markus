# markus

Makdown-like text parser.

## Usage

```rust
use markus::event::Event;
use markus::parser::Parser;

fn main() {
    let text = "abc\ndef\nghi";
    let mut parser = Parser::new(text);
    assert_eq!(parser.next(), Some(Event::ParagraphBegin));
    assert_eq!(parser.next(), Some(Event::Text("abc")));
    assert_eq!(parser.next(), Some(Event::Text("def")));
    assert_eq!(parser.next(), Some(Event::Text("ghi")));
    assert_eq!(parser.next(), Some(Event::ParagraphEnd));
    assert_eq!(parser.next(), None);
}
```
