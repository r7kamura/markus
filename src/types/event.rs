use crate::types::Tag;

#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Begin(Tag),
    End(Tag),
    Text(&'a str),
}
