use crate::block::{Block, BlockKind};
use crate::event::Event;
use crate::tree::Tree;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Parser<'a> {
    text: &'a str,
    tree: Tree<Block>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            tree: text.into(),
        }
    }
}

/// Parser emits events as Iterator.
impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tree.current {
            Some(index) => {
                let node = self.tree.nodes[index];
                match node.item.kind {
                    BlockKind::Paragraph => {
                        self.tree.go_to_child();
                        Some(Event::ParagraphBegin)
                    }
                    BlockKind::Text => {
                        self.tree.go_to_next_sibling();
                        Some(Event::Text(&self.text[node.item.begin..node.item.end + 1]))
                    }
                }
            }
            None => {
                self.tree.go_to_parent();
                self.tree.current?;
                self.tree.go_to_next_sibling();
                Some(Event::ParagraphEnd)
            }
        }
    }
}

/// Convert text into block-level tree.
impl From<&str> for Tree<Block> {
    fn from(text: &str) -> Self {
        let parser = crate::block_level_parser::Parser::new(text);
        parser.run()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::event::Event;

    #[test]
    fn parse_works() {
        let text = "abc\ndef\nghi";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::ParagraphBegin));
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(parser.next(), Some(Event::Text("def")));
        assert_eq!(parser.next(), Some(Event::Text("ghi")));
        assert_eq!(parser.next(), Some(Event::ParagraphEnd));
        assert_eq!(parser.next(), None);
    }
}
