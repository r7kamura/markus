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

    fn assert_parse(a: &str, b: &str) {
        let parser = Parser::new(a);
        let mut buffer = String::new();
        crate::html::push_html(&mut buffer, parser);
        assert_eq!(&buffer, b);
    }

    #[test]
    fn parse_example_043() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/043.md"),
            include_str!("../tests/fixtures/htmls/043.html"),
        );
    }

    #[test]
    fn parse_example_044() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/044.md"),
            include_str!("../tests/fixtures/htmls/044.html"),
        );
    }

    #[test]
    fn parse_example_045() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/045.md"),
            include_str!("../tests/fixtures/htmls/045.html"),
        );
    }

    #[test]
    fn parse_example_046() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/046.md"),
            include_str!("../tests/fixtures/htmls/046.html"),
        );
    }

    #[test]
    fn parse_example_047() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/047.md"),
            include_str!("../tests/fixtures/htmls/047.html"),
        );
    }

    #[test]
    fn parse_example_049() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/049.md"),
            include_str!("../tests/fixtures/htmls/049.html"),
        );
    }

    #[test]
    fn parse_example_050() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/050.md"),
            include_str!("../tests/fixtures/htmls/050.html"),
        );
    }

    #[test]
    fn parse_example_051() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/051.md"),
            include_str!("../tests/fixtures/htmls/051.html"),
        );
    }

    #[test]
    fn parse_example_052() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/052.md"),
            include_str!("../tests/fixtures/htmls/052.html"),
        );
    }

    #[test]
    fn parse_example_053() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/053.md"),
            include_str!("../tests/fixtures/htmls/053.html"),
        );
    }

    #[test]
    fn parse_example_054() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/054.md"),
            include_str!("../tests/fixtures/htmls/054.html"),
        );
    }

    #[test]
    fn parse_example_055() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/055.md"),
            include_str!("../tests/fixtures/htmls/055.html"),
        );
    }

    #[test]
    fn parse_example_058() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/058.md"),
            include_str!("../tests/fixtures/htmls/058.html"),
        );
    }

    #[test]
    fn parse_example_062() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/062.md"),
            include_str!("../tests/fixtures/htmls/062.html"),
        );
    }

    #[test]
    fn parse_example_063() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/063.md"),
            include_str!("../tests/fixtures/htmls/063.html"),
        );
    }

    #[test]
    fn parse_example_064() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/064.md"),
            include_str!("../tests/fixtures/htmls/064.html"),
        );
    }

    #[test]
    fn parse_example_067() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/067.md"),
            include_str!("../tests/fixtures/htmls/067.html"),
        );
    }

    #[test]
    fn parse_example_068() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/068.md"),
            include_str!("../tests/fixtures/htmls/068.html"),
        );
    }

    #[test]
    fn parse_example_070() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/070.md"),
            include_str!("../tests/fixtures/htmls/070.html"),
        );
    }

    #[test]
    fn parse_example_071() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/071.md"),
            include_str!("../tests/fixtures/htmls/071.html"),
        );
    }

    #[test]
    fn parse_example_072() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/072.md"),
            include_str!("../tests/fixtures/htmls/072.html"),
        );
    }

    #[test]
    fn parse_example_073() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/073.md"),
            include_str!("../tests/fixtures/htmls/073.html"),
        );
    }

    #[test]
    fn parse_example_074() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/074.md"),
            include_str!("../tests/fixtures/htmls/074.html"),
        );
    }

    #[test]
    fn parse_example_077() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/077.md"),
            include_str!("../tests/fixtures/htmls/077.html"),
        );
    }

    #[test]
    fn parse_example_078() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/078.md"),
            include_str!("../tests/fixtures/htmls/078.html"),
        );
    }

    #[test]
    fn parse_example_079() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/079.md"),
            include_str!("../tests/fixtures/htmls/079.html"),
        );
    }

    #[test]
    fn parse_example_219() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/219.md"),
            include_str!("../tests/fixtures/htmls/219.html"),
        );
    }

    #[test]
    fn parse_example_220() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/220.md"),
            include_str!("../tests/fixtures/htmls/220.html"),
        );
    }

    #[test]
    fn parse_example_221() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/221.md"),
            include_str!("../tests/fixtures/htmls/221.html"),
        );
    }

    #[test]
    fn parse_example_222() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/222.md"),
            include_str!("../tests/fixtures/htmls/222.html"),
        );
    }

    #[test]
    fn parse_example_223() {
        assert_parse(
            include_str!("../tests/fixtures/markdowns/223.md"),
            include_str!("../tests/fixtures/htmls/223.html"),
        );
    }
}
