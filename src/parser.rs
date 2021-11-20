use crate::event::Event;
use crate::tree::Tree;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Parser<'a> {
    text: &'a str,
    tree: Tree<Block>,
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    begin: usize,
    end: usize,
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
                self.tree.go_to_next_sibling_of(index);
                Some(Event::Text(&self.text[node.item.begin..node.item.end + 1]))
            }
            None => None,
        }
    }
}

/// Convert text into block-level tree.
impl From<&str> for Tree<Block> {
    fn from(text: &str) -> Self {
        let mut tree = Tree::new();

        let mut index = 0;
        while index < text.len() {
            if let Some(i) = text[index..].find('\n') {
                tree.append(Block {
                    begin: index,
                    end: index + i - 1,
                });
                index += i + 1;
            } else {
                tree.append(Block {
                    begin: index,
                    end: text.len() - 1,
                });
                break;
            }
        }

        tree.rewind();
        tree
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
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(parser.next(), Some(Event::Text("def")));
        assert_eq!(parser.next(), Some(Event::Text("ghi")));
        assert_eq!(parser.next(), None);
    }
}
