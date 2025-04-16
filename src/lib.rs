
mod parser;
mod formatter;

// Re-exports

pub use parser::parse;
pub use formatter::pretty_print;


mod tests {

    use super::*;
    use crate::parser::{parse, SyntaxKind};

    fn format_code(input: &str, entry: SyntaxKind) -> String {
        let parsed = parse("", input, entry);
        assert_eq!(parsed.errors.len(), 0);
        pretty_print(parsed.tokens, parsed.comments, parsed.events)
    }

    #[test]
    fn test_expression_formatting() {
        let source = "x+2   * (  - y)";
        let expected = "x + 2 * (-y)";
        let actual = format_code(source, SyntaxKind::SimpleExpression);
        assert_eq!(
            expected, actual,
            "expected that operators are surrouned with spaces"
        );
    }
}
