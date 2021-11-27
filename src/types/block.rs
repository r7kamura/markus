use crate::types::HeadingLevel;

#[derive(Clone, Copy, Debug)]
pub struct Block<'a> {
    pub begin: usize,
    pub end: usize,
    pub kind: BlockKind<'a>,
}

#[derive(Clone, Copy, Debug)]
pub enum BlockKind<'a> {
    FencedCodeBlock(&'a str),
    Heading(HeadingLevel),
    IndentedCodeBlock,
    Paragraph,
    Text,
    ThematicBreak,
}
