use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub begin: usize,
    pub end: usize,
    pub kind: BlockKind,
}

#[derive(Clone, Copy, Debug)]
pub enum BlockKind {
    Heading(HeadingLevel),
    Paragraph,
    Text,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
pub struct InvalidHeadingLevel(pub usize);

impl TryFrom<usize> for HeadingLevel {
    type Error = InvalidHeadingLevel;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::H1),
            2 => Ok(Self::H2),
            3 => Ok(Self::H3),
            4 => Ok(Self::H4),
            5 => Ok(Self::H5),
            6 => Ok(Self::H6),
            _ => Err(InvalidHeadingLevel(value)),
        }
    }
}
