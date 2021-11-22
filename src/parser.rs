use crate::tree::Tree;
use crate::types::{Block, BlockKind, Event, Tag};
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
                    BlockKind::Heading(level) => {
                        self.tree.go_to_child();
                        Some(Event::Begin(Tag::Heading(level)))
                    }
                    BlockKind::Paragraph => {
                        self.tree.go_to_child();
                        Some(Event::Begin(Tag::Paragraph))
                    }
                    BlockKind::Text => {
                        self.tree.go_to_next_sibling();
                        Some(Event::Text(&self.text[node.item.begin..node.item.end + 1]))
                    }
                    BlockKind::ThematicBreak => {
                        self.tree.go_to_next_sibling();
                        Some(Event::ThematicBreak)
                    }
                }
            }
            None => {
                self.tree.go_to_parent();
                let index = self.tree.current?;
                let event = match self.tree.nodes[index].item.kind {
                    BlockKind::Heading(level) => Some(Event::End(Tag::Heading(level))),
                    BlockKind::Paragraph => Some(Event::End(Tag::Paragraph)),
                    _ => panic!("Unexpected node is found as a parent."),
                };
                self.tree.go_to_next_sibling();
                event
            }
        }
    }
}
