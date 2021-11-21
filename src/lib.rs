pub mod block_level_parser;
pub mod parser;
pub mod tree;
pub mod types;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[cfg(test)]
mod tests {
    use super::parser::Parser;
    use super::types::{Event, HeadingLevel, Tag};

    #[test]
    fn parse_paragraphs() {
        let text = "abc\ndef\nghi\n\njkl";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::Begin(Tag::Paragraph)));
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(parser.next(), Some(Event::Text("def")));
        assert_eq!(parser.next(), Some(Event::Text("ghi")));
        assert_eq!(parser.next(), Some(Event::End(Tag::Paragraph)));
        assert_eq!(parser.next(), Some(Event::Begin(Tag::Paragraph)));
        assert_eq!(parser.next(), Some(Event::Text("jkl")));
        assert_eq!(parser.next(), Some(Event::End(Tag::Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading() {
        let text = "## abc";
        let mut parser = Parser::new(text);
        assert_eq!(
            parser.next(),
            Some(Event::Begin(Tag::Heading(HeadingLevel::H2)))
        );
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(
            parser.next(),
            Some(Event::End(Tag::Heading(HeadingLevel::H2)))
        );
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_whitespaces() {
        let text = "##  abc";
        let mut parser = Parser::new(text);
        assert_eq!(
            parser.next(),
            Some(Event::Begin(Tag::Heading(HeadingLevel::H2)))
        );
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(
            parser.next(),
            Some(Event::End(Tag::Heading(HeadingLevel::H2)))
        );
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_empty_text() {
        let text = "##";
        let mut parser = Parser::new(text);
        assert_eq!(
            parser.next(),
            Some(Event::Begin(Tag::Heading(HeadingLevel::H2)))
        );
        assert_eq!(parser.next(), Some(Event::Text("")));
        assert_eq!(
            parser.next(),
            Some(Event::End(Tag::Heading(HeadingLevel::H2)))
        );
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_invalid_heading_level() {
        let text = "####### abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::Begin(Tag::Paragraph)));
        assert_eq!(parser.next(), Some(Event::Text("####### abc")));
        assert_eq!(parser.next(), Some(Event::End(Tag::Paragraph)));
        assert_eq!(parser.next(), None);
    }
}
