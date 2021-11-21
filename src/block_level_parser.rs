use crate::block::{Block, BlockKind};
use crate::tree::Tree;

/// Convert text into block-level tree.
impl From<&str> for Tree<Block> {
    fn from(text: &str) -> Self {
        Parser::new(text).run()
    }
}

struct Parser<'a> {
    text: &'a str,
    tree: Tree<Block>,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            tree: Tree::new(),
        }
    }

    fn run(mut self) -> Tree<Block> {
        let mut index = 0;
        while index < self.text.len() {
            if let Some(level) = self.scan_atx_heading(index) {
                index = self.parse_atx_heading(index, level);
            } else {
                index = self.parse_paragraph(index);
            }
        }

        self.tree.go_to_first();
        self.tree
    }

    /// Parse one line from given index, and return index after the line.
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

    /// Parse one paragraph from given index, and return index after the paragraph.
    fn parse_paragraph(&mut self, mut index: usize) -> usize {
        self.tree.append(Block {
            begin: index,
            end: 0, // This dummy value will be fixed at the end of this function.
            kind: BlockKind::Paragraph,
        });
        self.tree.go_to_child();

        loop {
            index = self.parse_line(index);
            if index == self.text.len() {
                break;
            }
            if self.text[index..].starts_with('\n') {
                index += 1;
                break;
            }
        }

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
    }

    /// Parse ATX-style heading (e.g. `## Usage`) from given index, and return index after the heading.
    fn parse_atx_heading(&mut self, mut index: usize, level: usize) -> usize {
        self.tree.append(Block {
            begin: index,
            end: 0, // This dummy value will be fixed at the end of this function.
            kind: BlockKind::Heading(level),
        });
        self.tree.go_to_child();

        index += level;
        index = self.parse_non_break_whitespaces(index);
        index = self.parse_line(index);

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
    }

    /// Check if ATX-style heading starts from given index, and return its level if found.
    fn scan_atx_heading(&self, index: usize) -> Option<usize> {
        let bytes = self.text[index..].as_bytes();
        let level = bytes.iter().take_while(|&&byte| byte == b'#').count();
        if bytes
            .get(level)
            .map_or(true, |&byte| (0x09..=0x0d).contains(&byte) || byte == b' ')
        {
            Some(level)
        } else {
            None
        }
    }

    /// Parse possible non-break whitespaces, and return index after parse.
    fn parse_non_break_whitespaces(&self, index: usize) -> usize {
        index
            + self.text[index..]
                .as_bytes()
                .iter()
                .take_while(|&&byte| byte == b'\t' || byte == 0x0b || byte == 0x0c || byte == b' ')
                .count()
    }
}
