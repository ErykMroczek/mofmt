mod lexing;
mod parsing;
mod tokens;
mod cst;

// Re-exports

pub use tokens::{TokenKind, Token, TokenID, Tokenized};
pub use parsing::SyntaxKind;
pub use cst::{Child, ModelicaCST, TreeID};

/// Return Modelica Concrete Syntax Tree object
pub fn parse(source: String, code: String, entry: SyntaxKind) -> ModelicaCST {
    let tokens = lexing::lex(source, code);
    let events = parsing::events(&tokens, entry);
    cst::ModelicaCST::new(tokens, events)
}
