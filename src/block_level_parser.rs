use crate::tree::Tree;
use crate::types::{Block, BlockKind, HeadingLevel};
use std::convert::TryFrom;

/// Convert text into block-level tree.
impl<'a> From<&'a str> for Tree<Block<'a>> {
    fn from(text: &'a str) -> Self {
        Parser::new(text).run()
    }
}

struct Parser<'a> {
    text: &'a str,
    tree: Tree<Block<'a>>,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            tree: Tree::new(),
        }
    }

    fn run(mut self) -> Tree<Block<'a>> {
        let mut index = 0;
        while index < self.text.len() {
            while let Some(marker_length) = self.scan_block_quote_marker(index) {
                index = self.parse_block_quote_marker(index, marker_length);
            }

            if let Some(length) = self.scan_blank_line(index) {
                index += length;
            } else {
                let indent_level = self.scan_indent(index);
                if indent_level == 4 {
                    index = self.parse_indented_code_block(index);
                } else {
                    let index_before_parse_spaces_or_tabs = index;
                    index = self.parse_spaces_or_tabs(index);
                    if let Some(closing) = self.scan_html_block_type_1_to_5(index) {
                        index = self.parse_html_block_type_1_to_5(index, closing);
                    } else if self.scan_html_block_type_6(index)
                        || self.scan_html_block_type_7(index)
                    {
                        index =
                            self.parse_html_block_type_6_to_7(index_before_parse_spaces_or_tabs);
                    } else if let Some(length) = self.scan_thematic_break(index) {
                        index = self.parse_thematic_break(index, length);
                    } else if let Some(level) = self.scan_atx_heading(index) {
                        index = self.parse_atx_heading(index, level);
                    } else if let Some((length, byte)) = self.scan_openning_code_fence(index) {
                        index = self.parse_fenced_code_block(index, length, byte, indent_level);
                    } else {
                        index = self.parse_setext_heading_or_paragraph(index);
                    }
                }
            }

            for _ in 0..self.tree.ancestors.len() {
                if let Some(marker_length) = self.scan_block_quote_marker(index) {
                    index += marker_length;
                } else {
                    self.tree.go_to_parent();
                    self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1;
                }
            }
        }

        self.tree.go_to_first();
        self.tree
    }

    fn parse_block_quote_marker(&mut self, index: usize, marker_length: usize) -> usize {
        self.tree.append(Block {
            begin: index,
            end: 0, // Dummy,
            kind: BlockKind::BlockQuote,
        });
        self.tree.go_to_child();
        index + marker_length
    }

    fn parse_html_block_type_6_to_7(&mut self, mut index: usize) -> usize {
        while index < self.text.len() {
            let previous_index = index;
            index += self.scan_line(index);
            self.tree.append(Block {
                begin: previous_index,
                end: index - 1,
                kind: BlockKind::Html,
            });
            if self.scan_blank_line(index).is_some() {
                break;
            }
        }
        index
    }

    fn parse_html_block_type_1_to_5(&mut self, mut index: usize, closing: &str) -> usize {
        while index < self.text.len() {
            let previous_index = index;
            index += self.scan_line(index);
            self.tree.append(Block {
                begin: previous_index,
                end: index - 1,
                kind: BlockKind::Html,
            });
            if self.text[previous_index..index].contains(closing) {
                break;
            }
        }
        index
    }

    fn parse_fenced_code_block(
        &mut self,
        begin: usize,
        length: usize,
        byte: u8,
        indent_level: usize,
    ) -> usize {
        let mut index = begin;
        index += length;
        index = self.parse_spaces(index);
        let line_length = self.scan_line(index);
        let info_begin = index;
        let mut info_end = info_begin + line_length - 1; // TODO: Replace 1 with true line ending length.
        info_end -= self.text.as_bytes()[info_begin..info_end]
            .iter()
            .rev()
            .take_while(|&&b| is_non_line_ending_whitespaces(b))
            .count();
        let info = &self.text[info_begin..info_end];
        self.tree.append(Block {
            begin,
            end: 0, // Dummy,
            kind: BlockKind::FencedCodeBlock(info),
        });
        self.tree.go_to_child();

        index += line_length;
        loop {
            if self.scan_container_markers(index) != self.tree.ancestors.len() - 1 {
                break;
            }

            let index_to_check_closing = self.parse_indent(index, 3);
            if let Some(length) = self.scan_closing_code_fence(index_to_check_closing, byte, length)
            {
                index = index_to_check_closing + length;
                break;
            }
            index = self.parse_indent(index, indent_level);
            index = self.parse_line(index);
        }

        self.tree.go_to_parent();
        self.tree.nodes[self.tree.current.unwrap()].item.end = index - 1; // Fix dummy value.
        index
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
            index = self.parse_indent(index, 4);
            index = self.parse_line(index);
            if is_non_blank {
                last_non_blank_node = self.tree.current;
            }
            if self.scan_container_markers(index) != self.tree.ancestors.len() - 1
                || self.scan_indent(index) != 4 && self.scan_blank_line(index).is_none()
            {
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

            // Only non-lazy block quoted line can be setext heading marker.
            let mut lazy = false;
            let mut index2 = index;
            for &node_index in &self.tree.ancestors {
                if let BlockKind::BlockQuote = self.tree.nodes[node_index].item.kind {
                    if let Some(marker_length) = self.scan_block_quote_marker(index2) {
                        index2 += marker_length;
                    } else {
                        lazy = true;
                        break;
                    }
                }
            }

            // Skip interrupt if 4 spaces indent is detected.
            let spaces_length = self.parse_spaces(index2) - index2;
            if spaces_length != 4 {
                index2 += spaces_length;

                if !lazy {
                    if let Some((length, level)) = self.scan_setext_heading(index2) {
                        index2 += length;
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
                        index = index2;
                        break;
                    }
                }

                if self.scan_paragraph_interrupt(index2) {
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

            index = index2;
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

    /// Parse indent of given indent level, and return index after parse.
    fn parse_indent(&self, index: usize, indent_level: usize) -> usize {
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
            if level > indent_level {
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

    fn parse_spaces_up_to(&self, index: usize, count: usize) -> usize {
        index + std::cmp::min(self.parse_spaces(index) - index, count)
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

    /// May return block quote marker byte length that starts from given byte index.
    fn scan_block_quote_marker(&self, begin: usize) -> Option<usize> {
        let mut index = self.parse_spaces_up_to(begin, 3);
        if self.text[index..].starts_with('>') {
            index += 1;
            index = self.parse_spaces_up_to(index, 1);
            Some(index - begin)
        } else {
            None
        }
    }

    /// Return closing sequence.
    fn scan_html_block_type_1_to_5(&self, index: usize) -> Option<&'static str> {
        let bytes = self.text[index..].as_bytes();
        if bytes.get(0)? != &b'<' {
            return None;
        }

        let patterns: [(&[u8], &str); 4] = [
            (b"<pre", "</pre>"),
            (b"<style", "</style>"),
            (b"<script", "</script>"),
            (b"<textarea", "</textarea>"),
        ];
        for (openning, closing) in patterns {
            let length = openning.len();
            if bytes.len() < length {
                break;
            }
            if !bytes[..length].eq_ignore_ascii_case(openning) {
                continue;
            }
            if bytes.len() == length {
                return Some(closing);
            }
            let following = bytes[length];
            if is_whitespace(following) || following == b'>' {
                return Some(closing);
            }
        }

        let patterns: [(&[u8], &str); 3] = [(b"<!--", "-->"), (b"<?", "?>"), (b"<![CDATA[", "]]>")];
        for (openning, closing) in patterns {
            if bytes.starts_with(openning) {
                return Some(closing);
            }
        }

        if bytes.len() > 2 && bytes.starts_with(b"<!") && (b'A'..=b'z').contains(&bytes[2]) {
            return Some(">");
        }

        None
    }

    fn scan_html_block_type_6(&self, index: usize) -> bool {
        let bytes = &self.text.as_bytes()[index..];
        if bytes.is_empty() || bytes[0] != b'<' {
            return false;
        }
        let begin = if bytes.len() >= 2 && bytes[1] == b'/' {
            2
        } else {
            1
        };
        let length = bytes[begin..]
            .iter()
            .take_while(|&&byte| matches!(byte, b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z'))
            .count();
        let name = String::from_utf8(bytes[begin..(begin + length)].to_vec())
            .unwrap()
            .to_lowercase();
        let names: [&str; 62] = [
            "address",
            "article",
            "aside",
            "base",
            "basefont",
            "blockquote",
            "body",
            "caption",
            "center",
            "col",
            "colgroup",
            "dd",
            "details",
            "dialog",
            "dir",
            "div",
            "dl",
            "dt",
            "fieldset",
            "figcaption",
            "figure",
            "footer",
            "form",
            "frame",
            "frameset",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "head",
            "header",
            "hr",
            "html",
            "iframe",
            "legend",
            "li",
            "link",
            "main",
            "menu",
            "menuitem",
            "nav",
            "noframes",
            "ol",
            "optgroup",
            "option",
            "p",
            "param",
            "section",
            "source",
            "summary",
            "table",
            "tbody",
            "td",
            "tfoot",
            "th",
            "thead",
            "title",
            "tr",
            "track",
            "ul",
        ];
        names.iter().any(|&element| element == name)
    }

    fn scan_html_block_type_7(&self, index: usize) -> bool {
        let mut i = index;
        if !self.text[i..].starts_with('<') {
            return false;
        }
        i += 1;
        let is_closing_tag = self.text[i..].starts_with('/');
        if is_closing_tag {
            i += 1;
        }
        let payload = &self.text[i..];
        let tag_name_length = payload
            .find(|c| !is_ascii_alphanumeric(c))
            .unwrap_or(payload.len());
        if tag_name_length == 0 {
            return false;
        }
        i += tag_name_length;

        loop {
            i = self.parse_non_line_ending_whitespaces(i);
            if self.text[i..].starts_with('/') || self.text[i..].starts_with('>') {
                break;
            }

            if let Some(attribute_length) = self.scan_attribute(i) {
                i += attribute_length;
            } else {
                return false;
            }
        }

        if !is_closing_tag && self.text[i..].starts_with('/') {
            i += 1;
        }
        self.text[i..].starts_with('>') && self.scan_blank_line(i + 1).is_some()
    }

    /// Check if HTML tag's attribute part starts from given index, and return its length if found.
    fn scan_attribute(&self, begin: usize) -> Option<usize> {
        let mut index = begin;
        index += self.scan_attribute_name(index)?;
        let spaces_length = self.parse_spaces(index) - index;
        if self.text[index + spaces_length..].starts_with('=') {
            index += spaces_length + 1;
            index = self.parse_spaces(index);
            index += self.scan_attribute_value(index)?;
        }
        Some(index - begin)
    }

    /// Check if HTML tag's attribute name starts from given index, and return its length if found.
    fn scan_attribute_name(&self, index: usize) -> Option<usize> {
        if self.text[index..].starts_with(|c| is_ascii_alpha(c) || c == '_' || c == ':') {
            Some(
                1 + self.text[index + 1..]
                    .chars()
                    .take_while(|&c| {
                        is_ascii_alphanumeric(c) || c == '_' || c == ':' || c == '.' || c == '-'
                    })
                    .count(),
            )
        } else {
            None
        }
    }

    /// Check if HTML tag's attribute value starts from given index, and return its length if found.
    fn scan_attribute_value(&self, begin: usize) -> Option<usize> {
        let mut index = begin;
        let bytes = &self.text.as_bytes();
        match *bytes.get(index)? {
            b @ b'"' | b @ b'\'' => {
                index += 1;
                while index < bytes.len() {
                    if bytes[index] == b {
                        return Some(index + 1 - begin);
                    }
                    if self.scan_line_ending(index).is_some() {
                        return None;
                    }
                    index += 1;
                }
                None
            }
            b' ' | b'=' | b'>' | b'<' | b'`' | b'\n' | b'\r' => None,
            _ => Some(
                bytes
                    .iter()
                    .take_while(|&b| {
                        !matches!(
                            b,
                            b'\'' | b'"' | b' ' | b'=' | b'>' | b'<' | b'`' | b'\n' | b'\r'
                        )
                    })
                    .count(),
            ),
        }
    }

    fn scan_repeated_byte(&self, index: usize, byte: u8) -> usize {
        let bytes = self.text[index..].as_bytes();
        bytes
            .iter()
            .position(|&byte_| byte_ != byte)
            .unwrap_or_else(|| self.text.len())
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

    fn scan_openning_code_fence(&self, index: usize) -> Option<(usize, u8)> {
        let bytes = &self.text[index..].as_bytes();
        let byte = *bytes.get(0)?;
        if byte != b'`' && byte != b'~' {
            return None;
        }
        let count = self.scan_repeated_byte(index, byte);
        if count < 3 {
            return None;
        }
        if byte == b'~' {
            return Some((count, byte));
        }
        if bytes[count..(count + self.scan_line(index + count))]
            .iter()
            .any(|&b| b == b'`')
        {
            return None;
        }
        Some((count, byte))
    }

    fn scan_closing_code_fence(&self, index: usize, byte: u8, count: usize) -> Option<usize> {
        let bytes = &self.text[index..].as_bytes();
        if bytes.is_empty() {
            return Some(0);
        }
        let count_found = self.scan_repeated_byte(index, byte);
        if count_found < count {
            return None;
        }
        let mut new_index = index + count_found;
        new_index += self.scan_line_ending(new_index)?;
        Some(new_index - index)
    }

    /// Check if pargraph interrupt starts from given index.
    fn scan_paragraph_interrupt(&self, index: usize) -> bool {
        self.scan_line_ending(index).is_some()
            || self.scan_thematic_break(index).is_some()
            || self.scan_atx_heading(index).is_some()
            || self.scan_openning_code_fence(index).is_some()
            || self.scan_html_block_type_1_to_5(index).is_some()
            || self.scan_html_block_type_6(index)
            || self.scan_block_quote_marker(index).is_some()
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

    /// Return indent level (up to 4).
    fn scan_indent(&self, index: usize) -> usize {
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
        level
    }

    fn scan_line(&self, index: usize) -> usize {
        let bytes = &self.text[index..].as_bytes();
        if let Some(i) = bytes.iter().position(|&byte| is_line_ending(byte)) {
            i + self.scan_line_ending(index + i).unwrap()
        } else {
            bytes.len()
        }
    }

    // Scan how many container markers are located from given index, and return its count.
    fn scan_container_markers(&self, mut index: usize) -> usize {
        let mut count = 0;
        for &node_index in &self.tree.ancestors {
            if let BlockKind::BlockQuote = self.tree.nodes[node_index].item.kind {
                if let Some(marker_length) = self.scan_block_quote_marker(index) {
                    index += marker_length;
                    count += 1;
                } else {
                    break;
                }
            }
        }
        count
    }
}

fn is_ascii_alpha(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z')
}

fn is_ascii_alphanumeric(c: char) -> bool {
    is_ascii_alpha(c) || matches!(c, '0'..='9')
}

fn is_line_ending(byte: u8) -> bool {
    byte == b'\n' || byte == b'\r'
}

fn is_non_line_ending_whitespaces(byte: u8) -> bool {
    byte == b'\t' || byte == 0x0b || byte == 0x0c || byte == b' '
}

fn is_whitespace(byte: u8) -> bool {
    (0x09..=0x0d).contains(&byte) || byte == b' '
}
