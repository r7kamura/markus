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
    use super::types::Event::*;
    use super::types::HeadingLevel::*;
    use super::types::Tag::{Heading, Paragraph};

    #[test]
    fn parse_example_219_paragraphs() {
        let text = include_str!("../tests/fixtures/markdowns/219.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("aaa\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("bbb\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_220_paragraphs() {
        let text = include_str!("../tests/fixtures/markdowns/220.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("aaa\n")));
        assert_eq!(parser.next(), Some(Text("bbb\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("ccc\n")));
        assert_eq!(parser.next(), Some(Text("ddd\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_221_paragraphs() {
        let text = include_str!("../tests/fixtures/markdowns/221.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("aaa\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("bbb\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_062_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/062.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H1))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H1))));
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), Some(Begin(Heading(H3))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H3))));
        assert_eq!(parser.next(), Some(Begin(Heading(H4))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H4))));
        assert_eq!(parser.next(), Some(Begin(Heading(H5))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H5))));
        assert_eq!(parser.next(), Some(Begin(Heading(H6))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H6))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_063_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/063.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_064_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/064.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("#5 bolt\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("#hashtag\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_065_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/065.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_067_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/067.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H1))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H1))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_068_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/068.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Heading(H3))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H3))));
        assert_eq!(parser.next(), Some(Begin(Heading(H2))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H2))));
        assert_eq!(parser.next(), Some(Begin(Heading(H1))));
        assert_eq!(parser.next(), Some(Text("foo")));
        assert_eq!(parser.next(), Some(End(Heading(H1))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_070_atx_headings() {
        let text = include_str!("../tests/fixtures/markdowns/070.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("foo\n")));
        assert_eq!(parser.next(), Some(Text("    # bar\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_043_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/043.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_044_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/044.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_045_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/045.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_046_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/046.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("--\n")));
        assert_eq!(parser.next(), Some(Text("**\n")));
        assert_eq!(parser.next(), Some(Text("__\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_047_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/047.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_048_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/048.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_049_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/049.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("Foo\n")));
        assert_eq!(parser.next(), Some(Text("    ***\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_050_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/050.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_051_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/051.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_052_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/052.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_053_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/053.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_054_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/054.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_055_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/055.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("_ _ _ _ a\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("a------\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("---a---\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_056_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/056.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_example_059_thematic_breaks() {
        let text = include_str!("../tests/fixtures/markdowns/059.md");
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("Foo\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("bar\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }
}
