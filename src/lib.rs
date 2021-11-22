pub mod block_level_parser;
pub mod html;
pub mod parser;
pub mod tree;
pub mod types;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[cfg(test)]
mod tests {
    use super::parser::Parser;
    use super::types::Event::{Begin, End, Text};
    use super::types::HeadingLevel::H2;
    use super::types::Tag::{Heading, Paragraph};

    #[test]
    fn parse_paragraphs() {
        let text = "abc\ndef\nghi\n\njkl";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("abc\n")));
        assert_eq!(parser.next(), Some(Text("def\n")));
        assert_eq!(parser.next(), Some(Text("ghi\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("jkl")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading() {
        let text = "## abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("abc")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_closing_sequence() {
        let text = "## abc # ";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("abc")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_trailing_line_feed() {
        let text = "## abc\n";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("abc")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_trailing_carriage_return() {
        let text = "## abc\r";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("abc")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_whitespaces_after_marker() {
        let text = "##  abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("abc")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_a_few_spaces_before_marker() {
        let text = "  ## abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("abc")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    // TODO: Leading four spaces should be treated as a indented code block.
    #[test]
    fn parse_heading_with_too_many_spaces_before_marker() {
        let text = "    ## abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("    ## abc")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_empty_text() {
        let text = "##";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_empty_text_with_line_feed() {
        let text = "##\n";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_empty_text_with_closing_sequence() {
        let text = "## ##";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_invalid_heading_level() {
        let text = "####### abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("####### abc")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_without_space() {
        let text = "#abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("#abc")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }
}
