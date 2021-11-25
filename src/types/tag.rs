use crate::types::HeadingLevel;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Heading(HeadingLevel),
    IndentedCodeBlock,
    Paragraph,
}
