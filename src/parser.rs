mod lexing;
mod parsing;
mod tokens;
mod cst;

// Re-exports

pub use tokens::{TokenKind, TokenID, Position, Tokenized};
pub use parsing::SyntaxKind;
pub use cst::{Child, ModelicaCST, TreeID};

/// Parse Modelica code into a concrete syntax tree (CST).
/// 
/// # Arguments
/// * `source` - The source file name.
/// * `code` - The Modelica code to parse.
/// * `entry` - The entry point for parsing.
/// 
/// # Returns
/// A `ModelicaCST` object representing the parsed code.
pub fn parse(source: String, code: String, entry: SyntaxKind) -> ModelicaCST {
    let tokens = lexing::lex(source, code);
    let events = parsing::events(&tokens, entry);
    cst::ModelicaCST::new(tokens, events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{TokenKind, SyntaxKind};

    #[test]
    fn test_parse_correct_modelica_code() {
        let code = "model Complex \n parameter Real x = 1.0;  \n end Complex;".to_string();
        let cst = parse(String::from("test"), code, SyntaxKind::StoredDefinition);

        // Check token kinds
        let tokens = cst.tokens();
        assert_eq!(tokens.kind(tokens.first()), TokenKind::Model);
        assert_eq!(tokens.kind(tokens.last()), TokenKind::Semicolon);

        // Check token positions
        let first_token = tokens.get(tokens.first());
        assert_eq!(first_token.start.line, 1);
        assert_eq!(first_token.start.col, 1);

        let last_token = tokens.get(tokens.last());
        assert_eq!(last_token.start.line, 3);
        assert_eq!(last_token.start.col, 13);

        // Check CST navigation
        let root = cst.root().unwrap();
        assert_eq!(cst.kind(root), SyntaxKind::StoredDefinition);
        assert!(!cst.is_empty(root));
        assert!(cst.is_multiline(root));
        assert_eq!(cst.start(root), tokens.first());
        assert_eq!(cst.end(root), tokens.last());
        assert_eq!(cst.children(root).len(), 2);
    }

    #[test]
    fn test_parse_incorrect_modelica_code() {
        let code = "model Example end Example".to_string();
        let cst = parse(String::from("test"), code, SyntaxKind::StoredDefinition);

        // Check syntax errors
        let errors = cst.errors();
        assert_eq!(errors.len(), 1);
    }
}
