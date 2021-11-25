use crate::types::HeadingLevel;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub begin: usize,
    pub end: usize,
    pub kind: BlockKind,
}

#[derive(Clone, Copy, Debug)]
pub enum BlockKind {
    Heading(HeadingLevel),
    IndentedCodeBlock,
    Paragraph,
    Text,
    ThematicBreak,
}
