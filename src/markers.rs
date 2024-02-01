use moparse::Token;

#[derive(PartialEq)]
pub enum Marker {
    Token(Token),
    Indent,
    Dedent,
    Space,
    Blank,
    Break,
}
