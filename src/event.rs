#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Text(&'a str),

    ParagraphBegin,
    ParagraphEnd,
}
