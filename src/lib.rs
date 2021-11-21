pub mod block;
pub mod block_level_parser;
pub mod event;
pub mod parser;
pub mod tree;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[cfg(test)]
mod tests {
    use super::event::Event;
    use super::parser::Parser;

    #[test]
    fn parse_paragraphs() {
        let text = "abc\ndef\nghi\n\njkl";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::ParagraphBegin));
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(parser.next(), Some(Event::Text("def")));
        assert_eq!(parser.next(), Some(Event::Text("ghi")));
        assert_eq!(parser.next(), Some(Event::ParagraphEnd));
        assert_eq!(parser.next(), Some(Event::ParagraphBegin));
        assert_eq!(parser.next(), Some(Event::Text("jkl")));
        assert_eq!(parser.next(), Some(Event::ParagraphEnd));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading() {
        let text = "## abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::HeadingBegin(2)));
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(parser.next(), Some(Event::HeadingEnd));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_whitespaces() {
        let text = "##  abc";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::HeadingBegin(2)));
        assert_eq!(parser.next(), Some(Event::Text("abc")));
        assert_eq!(parser.next(), Some(Event::HeadingEnd));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_heading_with_empty_text() {
        let text = "##";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Event::HeadingBegin(2)));
        assert_eq!(parser.next(), Some(Event::Text("")));
        assert_eq!(parser.next(), Some(Event::HeadingEnd));
        assert_eq!(parser.next(), None);
    }
}
