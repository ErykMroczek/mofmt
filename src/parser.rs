mod lexing;
mod parsing;
mod tokens;
mod cst;

// Re-exports

pub use tokens::{TokenKind, TokenID, Position, Tokens};
pub use parsing::SyntaxKind;
pub use cst::{Child, ModelicaCST, TreeID};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{TokenKind, SyntaxKind};

    #[test]
    fn test_parse_correct_modelica_code() {
        let code = "model Complex \n parameter Real x = 1.0;  \n end Complex;".to_string();
        let cst = ModelicaCST::from(String::from("test"), code, SyntaxKind::StoredDefinition);

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
        let cst = ModelicaCST::from(String::from("test"), code, SyntaxKind::StoredDefinition);

        // Check syntax errors
        let errors = cst.errors();
        assert_eq!(errors.len(), 1);
    }
}
