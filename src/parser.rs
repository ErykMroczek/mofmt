mod events;
mod lexing;
mod parsing;
mod syntax;
mod tokens;
mod tree;

// Re-exports

pub use tokens::{ModelicaToken, Token};
pub use syntax::SyntaxKind;
pub use events::SyntaxEvent;
pub use tree::{Child, Tree, build_tree};

/// Output from the parser.
/// Contains everything necesary to build a parse tree.
pub struct ParsedModelica {
    pub tokens: Vec<Token>,
    pub comments: Vec<Token>,
    pub events: Vec<SyntaxEvent>,
    pub errors: Vec<String>,
}

/// Return `Parsed` object generated from the `source` string.
pub fn parse(name: &str, source: &str, entry: SyntaxKind) -> ParsedModelica {
    let (tokens, comments, mut errors) = lexing::lex(name, source);
    let (events, mut p_errors) = parsing::events(name, &tokens, entry);
    errors.append(&mut p_errors);
    ParsedModelica {
        tokens,
        comments,
        events,
        errors,
    }
}
