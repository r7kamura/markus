use crate::tree::Tree;
use crate::types::{Block, BlockKind, HeadingLevel};
use std::convert::TryFrom;

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
        if index >= self.text.len() {
            return index;
        }
        if let Some(i) = self.text[index..].find(is_line_break) {
            let end = index + i;
            self.tree.append(Block {
                begin: index,
                end,
                kind: BlockKind::Text,
            });
            end + 1
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
            if self.text[index..].starts_with(is_line_break) {
                index += 1;
                break;
            }
        }

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
    }

    /// Parse ATX heading (e.g. `## Usage`) from given index, and return index after the heading.
    fn parse_atx_heading(&mut self, mut index: usize, level: HeadingLevel) -> usize {
        self.tree.append(Block {
            begin: index,
            end: 0, // This dummy value will be fixed at the end of this function.
            kind: BlockKind::Heading(level),
        });
        self.tree.go_to_child();

        index += level as usize;
        index = self.parse_non_line_break_whitespaces(index);
        index = self.parse_line(index);
        if let Some(node_index) = self.tree.current {
            let item = self.tree.nodes[node_index].item;
            let header_text = &self.text.as_bytes()[item.begin..=item.end];
            let mut tail = header_text.len() - 1;
            tail = header_text[..=tail]
                .iter()
                .rposition(|&byte| byte != b'\n' && byte != b'\r')
                .unwrap_or(0);
            tail = header_text[..=tail]
                .iter()
                .rposition(|&byte| byte != b' ' && byte != b'\t')
                .unwrap_or(0);
            tail = header_text[..=tail]
                .iter()
                .rposition(|&byte| byte != b'#')
                .unwrap_or(0);
            tail = header_text[..=tail]
                .iter()
                .rposition(|&byte| byte != b' ' && byte != b'\t')
                .unwrap_or(0);
            self.tree.nodes[node_index].item.end = item.begin + tail;
        }

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
    }

    /// Check if ATX-style heading starts from given index, and return its level if found.
    fn scan_atx_heading(&self, index: usize) -> Option<HeadingLevel> {
        let bytes = self.text[index..].as_bytes();
        let level = bytes.iter().take_while(|&&byte| byte == b'#').count();
        if bytes
            .get(level)
            .map_or(true, |&byte| (0x09..=0x0d).contains(&byte) || byte == b' ')
        {
            HeadingLevel::try_from(level).ok()
        } else {
            None
        }
    }

    /// Parse possible non-break whitespaces, and return index after parse.
    fn parse_non_line_break_whitespaces(&self, index: usize) -> usize {
        index
            + self.text[index..]
                .as_bytes()
                .iter()
                .take_while(|&&byte| byte == b'\t' || byte == 0x0b || byte == 0x0c || byte == b' ')
                .count()
    }
}

fn is_line_break(c: char) -> bool {
    c == '\n' || c == '\r'
}
