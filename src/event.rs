use crate::block::HeadingLevel;

#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Text(&'a str),

    HeadingBegin(HeadingLevel),
    HeadingEnd(HeadingLevel),

    ParagraphBegin,
    ParagraphEnd,
}
