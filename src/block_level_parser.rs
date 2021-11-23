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
            if let Some(length) = self.scan_line_ending(index) {
                index += length;
                continue;
            }

            // TODO: Consider 4 spaces indented code block.
            index = self.parse_spaces_or_tabs(index);

            if let Some(length) = self.scan_thematic_break(index) {
                index = self.parse_thematic_break(index, length);
            } else if let Some(level) = self.scan_atx_heading(index) {
                index = self.parse_atx_heading(index, level);
            } else {
                index = self.parse_paragraph(index);
            }
        }

        self.tree.go_to_first();
        self.tree
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
            if self.scan_paragraph_interrupt(index) {
                if let Some(node_index) = self.tree.current {
                    let item = self.tree.nodes[node_index].item;
                    let text = &self.text.as_bytes()[item.begin..=item.end];
                    let mut tail = text.len();
                    tail = text[..tail]
                        .iter()
                        .rposition(|&byte| byte != b'\n' && byte != b'\r')
                        .map_or(0, |i| i + 1);
                    self.tree.nodes[node_index].item.end = item.begin + tail - 1;
                }
                break;
            }
            index = self.parse_spaces_or_tabs(index);
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
        index = self.parse_non_line_ending_whitespaces(index);
        index = self.parse_line(index);
        if let Some(node_index) = self.tree.current {
            let item = self.tree.nodes[node_index].item;
            let header_text = &self.text.as_bytes()[item.begin..=item.end];
            let mut tail = header_text.len();
            tail = header_text[..tail]
                .iter()
                .rposition(|&byte| byte != b'\n' && byte != b'\r')
                .map_or(0, |i| i + 1);
            tail = header_text[..tail]
                .iter()
                .rposition(|&byte| byte != b' ' && byte != b'\t')
                .map_or(0, |i| i + 1);
            tail = header_text[..tail]
                .iter()
                .rposition(|&byte| byte != b'#')
                .map_or(0, |i| i + 1);
            tail = header_text[..tail]
                .iter()
                .rposition(|&byte| byte != b' ' && byte != b'\t')
                .map_or(0, |i| i + 1);
            self.tree.nodes[node_index].item.end = item.begin + tail - 1;
            if tail == 0 {
                self.tree.nodes[*self.tree.ancestors.last().unwrap()].child = None;
            }
        }

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
    }

    /// Parse thematic break, and return index after parse.
    fn parse_thematic_break(&mut self, index: usize, length: usize) -> usize {
        let end = index + length;
        self.tree.append(Block {
            begin: index,
            end,
            kind: BlockKind::ThematicBreak,
        });
        end
    }

    /// Parse one line from given index, and return index after the line.
    fn parse_line(&mut self, index: usize) -> usize {
        if index >= self.text.len() {
            return index;
        }
        let end = if let Some(i) = self.text[index..].find(is_line_ending) {
            index + i
        } else {
            self.text.len() - 1
        };
        self.tree.append(Block {
            begin: index,
            end,
            kind: BlockKind::Text,
        });
        end + 1
    }

    /// Parse 0 or more spaces or tabs, and return index after parse.
    fn parse_spaces_or_tabs(&self, index: usize) -> usize {
        index
            + self.text[index..]
                .as_bytes()
                .iter()
                .take_while(|&&byte| byte == b' ' || byte == b'\t')
                .count()
    }

    /// Parse 0 or more non line ending whitespaces, and return index after parse.
    fn parse_non_line_ending_whitespaces(&self, index: usize) -> usize {
        index
            + self.text[index..]
                .as_bytes()
                .iter()
                .take_while(|&&byte| byte == b'\t' || byte == 0x0b || byte == 0x0c || byte == b' ')
                .count()
    }

    /// Check if ATX-style heading starts from given index, and return its level if found.
    fn scan_atx_heading(&self, index: usize) -> Option<HeadingLevel> {
        let mut bytes = self.text[index..].as_bytes();
        let position = bytes.iter().position(|&byte| byte != b' ')?;
        if position >= 4 {
            return None;
        }
        bytes = &bytes[position..];

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

    /// Check if thematic break starts from given index, and return its length if found, which includes possible line ending.
    fn scan_thematic_break(&self, index: usize) -> Option<usize> {
        let bytes = &self.text.as_bytes()[index..];
        let mut i = bytes.iter().position(|&byte| byte != b' ')?;
        if i >= 4 {
            return None;
        }
        let mut characters_count = 0;
        let mut first_found_character = None;
        while i < bytes.len() {
            match bytes[i] {
                c if c == b'*' || c == b'-' || c == b'_' => {
                    i += 1;
                    if let Some(first) = first_found_character {
                        if first != c {
                            return None;
                        }
                        characters_count += 1;
                    } else {
                        characters_count = 1;
                        first_found_character = Some(c)
                    }
                }
                b' ' | b'\t' => {
                    i += 1;
                }
                b'\n' => {
                    i += 1;
                    break;
                }
                b'\r' => {
                    i += 1;
                    if bytes[i] == b'\n' {
                        i += 1;
                    }
                    break;
                }
                _ => return None,
            }
        }
        if characters_count >= 3 {
            Some(i)
        } else {
            None
        }
    }

    /// Check if pargraph interrupt starts from given index.
    fn scan_paragraph_interrupt(&self, index: usize) -> bool {
        self.scan_line_ending(index).is_some()
            || self.scan_thematic_break(index).is_some()
            || self.scan_atx_heading(index).is_some()
    }

    /// Check if line ending starts from given index, and return its length if found.
    fn scan_line_ending(&self, index: usize) -> Option<usize> {
        let bytes = &self.text[index..].as_bytes();
        if bytes.is_empty() {
            return Some(0);
        }
        match bytes[0] {
            b'\n' => Some(1),
            b'\r' => Some(if bytes.get(1) == Some(&b'\n') { 2 } else { 1 }),
            _ => None,
        }
    }
}

fn is_line_ending(c: char) -> bool {
    c == '\n' || c == '\r'
}
