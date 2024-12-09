use std::fmt::{Debug, Display, Error, Formatter};

#[derive(PartialEq, Copy, Clone)]
/// Represents a type of a Modelica token. Defined based on Modelica
/// Specification
/// 3.7
pub enum ModelicaToken {
    EOF,
    Comma,
    Dot,
    Semicolon,
    Colon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    Equal,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Flex,
    DotPlus,
    DotMinus,
    DotStar,
    DotSlash,
    DotFlex,
    Gre,
    Geq,
    Les,
    Leq,
    Neq,
    Eq,
    Not,
    And,
    Or,
    In,
    For,
    If,
    Else,
    ElseIf,
    Then,
    When,
    ElseWhen,
    While,
    Loop,
    Break,
    Return,
    Partial,
    Class,
    Operator,
    Expandable,
    Model,
    Function,
    Record,
    Type,
    Block,
    Connector,
    Package,
    Pure,
    Impure,
    Initial,
    Equation,
    Algorithm,
    Extends,
    Import,
    Public,
    Protected,
    Within,
    Final,
    Encapsulated,
    Enumeration,
    Input,
    Output,
    Redeclare,
    Inner,
    Outer,
    Replaceable,
    Constrainedby,
    Flow,
    Stream,
    Discrete,
    Parameter,
    Constant,
    Each,
    Annotation,
    External,
    End,
    Der,
    Connect,
    LineComment,
    BlockComment,
    Identifier,
    String,
    UInt,
    UReal,
    Bool,
}

impl Debug for ModelicaToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ModelicaToken::EOF => write!(f, "EOF"),
            ModelicaToken::Comma => write!(f, "','"),
            ModelicaToken::Dot => write!(f, "'.''"),
            ModelicaToken::Semicolon => write!(f, "';'"),
            ModelicaToken::Colon => write!(f, "':'"),
            ModelicaToken::LParen => write!(f, "'('"),
            ModelicaToken::RParen => write!(f, "')'"),
            ModelicaToken::LCurly => write!(f, "'{{'"),
            ModelicaToken::RCurly => write!(f, "'}}'"),
            ModelicaToken::LBracket => write!(f, "'['"),
            ModelicaToken::RBracket => write!(f, "']'"),
            ModelicaToken::Equal => write!(f, "'='"),
            ModelicaToken::Assign => write!(f, "':='"),
            ModelicaToken::Plus => write!(f, "'+'"),
            ModelicaToken::Minus => write!(f, "'-'"),
            ModelicaToken::Star => write!(f, "'*'"),
            ModelicaToken::Slash => write!(f, "'/'"),
            ModelicaToken::Flex => write!(f, "'^'"),
            ModelicaToken::DotPlus => write!(f, "'.+'"),
            ModelicaToken::DotMinus => write!(f, "'.-'"),
            ModelicaToken::DotStar => write!(f, "'.*'"),
            ModelicaToken::DotSlash => write!(f, "'./'"),
            ModelicaToken::DotFlex => write!(f, "'.^'"),
            ModelicaToken::Gre => write!(f, "'>'"),
            ModelicaToken::Geq => write!(f, "'>='"),
            ModelicaToken::Les => write!(f, "'<'"),
            ModelicaToken::Leq => write!(f, "'<='"),
            ModelicaToken::Neq => write!(f, "'<>'"),
            ModelicaToken::Eq => write!(f, "'=='"),
            ModelicaToken::Not => write!(f, "'not'"),
            ModelicaToken::And => write!(f, "'and'"),
            ModelicaToken::Or => write!(f, "'or'"),
            ModelicaToken::In => write!(f, "'in'"),
            ModelicaToken::For => write!(f, "for'"),
            ModelicaToken::If => write!(f, "'if'"),
            ModelicaToken::Else => write!(f, "'else'"),
            ModelicaToken::ElseIf => write!(f, "'elseif'"),
            ModelicaToken::Then => write!(f, "'then'"),
            ModelicaToken::When => write!(f, "'when'"),
            ModelicaToken::ElseWhen => write!(f, "'elsewhen'"),
            ModelicaToken::While => write!(f, "'while'"),
            ModelicaToken::Loop => write!(f, "'loop'"),
            ModelicaToken::Break => write!(f, "'break'"),
            ModelicaToken::Return => write!(f, "'return'"),
            ModelicaToken::Partial => write!(f, "'partial'"),
            ModelicaToken::Class => write!(f, "'class'"),
            ModelicaToken::Operator => write!(f, "'operator'"),
            ModelicaToken::Expandable => write!(f, "'expandable'"),
            ModelicaToken::Model => write!(f, "'model'"),
            ModelicaToken::Function => write!(f, "'function'"),
            ModelicaToken::Record => write!(f, "'record'"),
            ModelicaToken::Type => write!(f, "'type'"),
            ModelicaToken::Block => write!(f, "'block'"),
            ModelicaToken::Connector => write!(f, "'connector'"),
            ModelicaToken::Package => write!(f, "'package'"),
            ModelicaToken::Pure => write!(f, "'pure'"),
            ModelicaToken::Impure => write!(f, "'impure'"),
            ModelicaToken::Initial => write!(f, "'initial'"),
            ModelicaToken::Equation => write!(f, "'equation'"),
            ModelicaToken::Algorithm => write!(f, "'algorithm'"),
            ModelicaToken::Extends => write!(f, "'extends'"),
            ModelicaToken::Import => write!(f, "'import'"),
            ModelicaToken::Public => write!(f, "'public'"),
            ModelicaToken::Protected => write!(f, "'protected'"),
            ModelicaToken::Within => write!(f, "'within'"),
            ModelicaToken::Final => write!(f, "'final'"),
            ModelicaToken::Encapsulated => write!(f, "'encapsulated'"),
            ModelicaToken::Enumeration => write!(f, "'enumeration'"),
            ModelicaToken::Input => write!(f, "'input'"),
            ModelicaToken::Output => write!(f, "'output'"),
            ModelicaToken::Redeclare => write!(f, "'redeclare'"),
            ModelicaToken::Inner => write!(f, "'inner'"),
            ModelicaToken::Outer => write!(f, "'outer'"),
            ModelicaToken::Replaceable => write!(f, "'replaceable'"),
            ModelicaToken::Constrainedby => write!(f, "'constrainedby'"),
            ModelicaToken::Flow => write!(f, "'flow'"),
            ModelicaToken::Stream => write!(f, "'stream'"),
            ModelicaToken::Discrete => write!(f, "'discrete'"),
            ModelicaToken::Parameter => write!(f, "'parameter'"),
            ModelicaToken::Constant => write!(f, "'constant'"),
            ModelicaToken::Each => write!(f, "'each'"),
            ModelicaToken::Annotation => write!(f, "'annotation'"),
            ModelicaToken::External => write!(f, "'external'"),
            ModelicaToken::End => write!(f, "'end'"),
            ModelicaToken::Der => write!(f, "'der'"),
            ModelicaToken::Connect => write!(f, "'connect'"),
            ModelicaToken::LineComment => write!(f, "LINE COMMENT"),
            ModelicaToken::BlockComment => write!(f, "BLOCK COMMENT"),
            ModelicaToken::Identifier => write!(f, "IDENTIFIER"),
            ModelicaToken::String => write!(f, "STRING"),
            ModelicaToken::UInt => write!(f, "UNSIGNED INTEGER"),
            ModelicaToken::UReal => write!(f, "UNSIGNED REAL"),
            ModelicaToken::Bool => write!(f, "'true' or 'false'"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Used to represent token's position in the input string
///
/// * `pos`: index of the character that corresponds with this position
/// * `line`: line number of the character that corresponds with this
///   position
/// * `col`: column number of the character that corresponds with this
///   position
pub struct Position {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
/// Represents a single Modelica token.
///
/// Tokens contain information on their type and their coordinates in
/// the source.
///
/// * `idx`: position in the token collection
/// * `text`: text content of the token
/// * `kind`: token's kind
/// * `start`: position of the first character
/// * `end`: position of the last character
pub struct Token {
    /// Index of the token in the input
    pub idx: usize,
    /// Text of the token
    pub text: String,
    /// Token's kind
    pub kind: ModelicaToken,
    /// Position of staring character in the input
    pub start: Position,
    /// Positon of ending character in the input
    pub end: Position,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "'{}'", self.text)
    }
}
