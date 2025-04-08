use super::syntax::SyntaxKind;

#[derive(Debug)]
/// Represents a single Modelica syntax event.
///
/// Syntax event may mark starts and ends of productions or terminals.
/// The list of such syntax events should be consumed to build a parse
/// tree or an AST.
pub enum SyntaxEvent {
    /// Event indicating beginning of the Modelica production.
    Enter(SyntaxKind),
    /// Event indicating an end of some Modelica production.
    Exit,
    /// Event indicating a token.
    Advance(usize),
    Error(String),
}
