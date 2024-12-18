// Re-exports

pub use parser::{parse, SyntaxKind, build_tree};

use self::formatter::{format, print};
use self::parser::{SyntaxEvent, Token};

/// Return string containing formatted Modelica code
pub fn pretty_print(tokens: Vec<Token>, comments: Vec<Token>, events: Vec<SyntaxEvent>) -> String {
    let tree = build_tree(tokens, events);
    let markers = format(tree, comments);
    print(markers)
}

mod parser;
mod formatter;

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
