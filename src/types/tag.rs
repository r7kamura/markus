use crate::types::HeadingLevel;

#[derive(Debug, PartialEq)]
pub enum Tag<'a> {
    FencedCodeBlock(&'a str),
    Heading(HeadingLevel),
    IndentedCodeBlock,
    Paragraph,
}
