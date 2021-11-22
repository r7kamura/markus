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

    #[test]
    fn parse_thematic_break_example_43() {
        let text = r#"***
---
___"#;
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_44() {
        let text = "+++";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_45() {
        let text = "===";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_46() {
        let text = r#"--
**
__"#;
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("--\n")));
        assert_eq!(parser.next(), Some(Text("**\n")));
        assert_eq!(parser.next(), Some(Text("__")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_47() {
        let text = r#" ***
  ***
   ***"#;
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_48() {
        let text = "    ***";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(text)));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_49() {
        let text = r#"Foo
    ***"#;
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("Foo\n")));
        assert_eq!(parser.next(), Some(Text("    ***")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_50() {
        let text = "_____________________________________";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_51() {
        let text = " - - -";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_52() {
        let text = " **  * ** * ** * **";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_53() {
        let text = "-     -      -      -";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_54() {
        let text = "- - - -    ";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_55() {
        let text = r#"_ _ _ _ a

a------

---a---"#;
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("_ _ _ _ a\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("a------\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("---a---")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_56() {
        let text = " *-*";
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text(" *-*")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parse_thematic_break_example_59() {
        let text = r#"Foo
***
bar"#;
        let mut parser = Parser::new(text);
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("Foo\n")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), Some(ThematicBreak));
        assert_eq!(parser.next(), Some(Begin(Paragraph)));
        assert_eq!(parser.next(), Some(Text("bar")));
        assert_eq!(parser.next(), Some(End(Paragraph)));
        assert_eq!(parser.next(), None);
    }
}
