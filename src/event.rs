#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Text(&'a str),

    HeadingBegin(usize),
    HeadingEnd,

    ParagraphBegin,
    ParagraphEnd,
}
