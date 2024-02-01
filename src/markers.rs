#[derive(PartialEq)]
pub enum Marker {
    Token(String),
    Indent,
    Dedent,
    Space,
    Blank,
    Break,
}
