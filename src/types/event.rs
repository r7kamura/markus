use crate::types::Tag;

#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Begin(Tag<'a>),
    End(Tag<'a>),
    Html(&'a str),
    Text(&'a str),
    ThematicBreak,
}
