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
            if let Some(length) = self.scan_blank_line(index) {
                index += length;
                continue;
            }

            if self.scan_indent_4(index) {
                index = self.parse_indented_code_block(index);
                continue;
            }

            index = self.parse_spaces_or_tabs(index);
            if let Some(length) = self.scan_thematic_break(index) {
                index = self.parse_thematic_break(index, length);
            } else if let Some(level) = self.scan_atx_heading(index) {
                index = self.parse_atx_heading(index, level);
            } else {
                index = self.parse_setext_heading_or_paragraph(index);
            }
        }

        self.tree.go_to_first();
        self.tree
    }

    /// Parse indented code block from given index, and return index after parse.
    fn parse_indented_code_block(&mut self, mut index: usize) -> usize {
        self.tree.append(Block {
            begin: index,
            end: 0, // Dummy
            kind: BlockKind::IndentedCodeBlock,
        });
        self.tree.go_to_child();

        let mut last_non_blank_node = None;
        let mut is_non_blank;
        while index < self.text.len() {
            is_non_blank = self.scan_blank_line(index).is_none();
            index = self.parse_indent_0_to_4(index);
            index = self.parse_line(index);
            if is_non_blank {
                last_non_blank_node = self.tree.current;
            }
            if !self.scan_indent_4(index) && self.scan_blank_line(index).is_none() {
                break;
            }
        }
        if let Some(node_index) = last_non_blank_node {
            self.tree.nodes[node_index].next = None;
            self.tree.current = last_non_blank_node;
        }

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
    }

    /// Parse setext heading or paragraph from given index, and return index after parse.
    fn parse_setext_heading_or_paragraph(&mut self, mut index: usize) -> usize {
        self.tree.append(Block {
            begin: index,
            end: 0, // This dummy value will be fixed at the end of this function.
            kind: BlockKind::Paragraph, // Maybe paragraph, but maybe setext heading.
        });
        self.tree.go_to_child();

        loop {
            index = self.parse_spaces_or_tabs(index);
            index = self.parse_line(index);

            // Skip interrupt if 4 spaces indent is detected.
            let new_index = self.parse_spaces(index);
            if new_index - index == 4 {
                continue;
            }
            index = new_index;

            if let Some((length, level)) = self.scan_setext_heading(index) {
                index += length;
                self.tree.nodes[*self.tree.ancestors.last().unwrap()]
                    .item
                    .kind = BlockKind::Heading(level);
                if let Some(node_index) = self.tree.current {
                    let item = self.tree.nodes[node_index].item;
                    let text = &self.text.as_bytes()[..item.end];
                    let tail = text
                        .iter()
                        .rposition(|&byte| !is_non_line_ending_whitespaces(byte))
                        .unwrap_or(0);
                    self.tree.nodes[node_index].item.end = tail;
                }
                break;
            }

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
            let tail2 = header_text[..tail]
                .iter()
                .rposition(|&byte| byte != b'#')
                .map_or(0, |i| i + 1);
            if tail2 == 0 {
                tail = tail2;
            } else {
                let tail3 = header_text[..tail2]
                    .iter()
                    .rposition(|&byte| byte != b' ' && byte != b'\t')
                    .map_or(0, |i| i + 1);
                if tail2 != tail3 {
                    tail = tail3
                }
            }
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
        let end = if let Some(i) = self.text[index..]
            .as_bytes()
            .iter()
            .position(|&byte| is_line_ending(byte))
        {
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

    /// Parse 0 to 4 level indent, and return index after parse.
    fn parse_indent_0_to_4(&self, index: usize) -> usize {
        let bytes = &self.text[index..].as_bytes();
        let mut i = 0;
        let mut level = 0;
        while i < bytes.len() {
            match bytes[i] {
                b' ' => {
                    level += 1;
                }
                b'\t' => {
                    level += 4 - i % 4;
                }
                _ => break,
            }
            if level > 4 {
                break;
            }
            i += 1;
        }
        index + i
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

    /// Parse spaces, and return index after parse.
    fn parse_spaces(&self, index: usize) -> usize {
        index
            + self.text[index..]
                .as_bytes()
                .iter()
                .take_while(|&&byte| byte == b' ')
                .count()
    }

    /// Parse 0 or more non line ending whitespaces, and return index after parse.
    fn parse_non_line_ending_whitespaces(&self, index: usize) -> usize {
        index
            + self.text[index..]
                .as_bytes()
                .iter()
                .take_while(|&&byte| is_non_line_ending_whitespaces(byte))
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

    /// Check if setext heading underline starts from given index, and returns its heading level and length if found (including line ending).
    fn scan_setext_heading(&self, index: usize) -> Option<(usize, HeadingLevel)> {
        let bytes = &self.text.as_bytes()[index..];
        let byte = *bytes.get(0)?;
        if byte != b'=' && byte != b'-' {
            return None;
        }
        let mut length = 1 + bytes[1..].iter().take_while(|&&b| b == byte).count();
        length += self.scan_blank_line(index + length)?;
        let level = if byte == b'=' {
            HeadingLevel::H1
        } else {
            HeadingLevel::H2
        };
        Some((length, level))
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

    /// Check if a series of whitespaces and a line ending starts from given index, and return its length.
    fn scan_blank_line(&self, index: usize) -> Option<usize> {
        let new_index = self.parse_non_line_ending_whitespaces(index);
        self.scan_line_ending(new_index)
            .map(|length| new_index - index + length)
    }

    /// Check if 4 level indent starts from given index.
    fn scan_indent_4(&self, index: usize) -> bool {
        let bytes = &self.text[index..].as_bytes();
        let mut i = 0;
        let mut level = 0;
        while level < 4 && i < bytes.len() {
            match bytes[i] {
                b' ' => {
                    i += 1;
                    level += 1;
                }
                b'\t' => {
                    i += 1;
                    level += 4 - i % 4;
                }
                _ => break,
            }
        }
        level == 4
    }
}

fn is_line_ending(byte: u8) -> bool {
    byte == b'\n' || byte == b'\r'
}

fn is_non_line_ending_whitespaces(byte: u8) -> bool {
    byte == b'\t' || byte == 0x0b || byte == 0x0c || byte == b' '
}
