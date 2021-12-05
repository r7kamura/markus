use crate::types::HeadingLevel;

#[derive(Debug, PartialEq)]
pub enum Tag<'a> {
    BlockQuote,
    FencedCodeBlock(&'a str),
    Heading(HeadingLevel),
    IndentedCodeBlock,
    Paragraph,
}
