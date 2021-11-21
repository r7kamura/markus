use crate::block::{Block, BlockKind};
use crate::tree::Tree;

pub struct Parser<'a> {
    text: &'a str,
    tree: Tree<Block>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            tree: Tree::new(),
        }
    }

    pub fn run(mut self) -> Tree<Block> {
        // Only 1 paragraph is supported for now.
        self.parse_paragraph();

        self.tree.go_to_first();
        self.tree
    }

    /// Parse one line and return index after line.
    fn parse_line(&mut self, index: usize) -> usize {
        if let Some(i) = self.text[index..].find('\n') {
            let end = index + i - 1;
            self.tree.append(Block {
                begin: index,
                end,
                kind: BlockKind::Text,
            });
            end + 2
        } else {
            let end = self.text.len() - 1;
            self.tree.append(Block {
                begin: index,
                end,
                kind: BlockKind::Text,
            });
            end + 1
        }
    }

    fn parse_paragraph(&mut self) {
        self.tree.append(Block {
            begin: 0,
            end: 0, // This dummy value will be fixed at the end of this function.
            kind: BlockKind::Paragraph,
        });
        self.tree.go_to_child();

        let mut index = 0;
        while index < self.text.len() {
            index = self.parse_line(index);
        }

        self.tree.go_to_parent();

        // Fix dummy value.
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1;
    }
}

pub fn parse(text: &str) -> Tree<Block> {
    let parser = Parser::new(text);
    parser.run()
}
