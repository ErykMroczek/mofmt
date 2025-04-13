use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
/// Represents a type of a Modelica token. Defined based on Modelica
/// Specification
/// 3.7
pub enum TokenKind {
    EOF,

    ErrorIllegalCharacter,
    ErrorIllegalQident,
    ErrorUnclosedString,
    ErrorUnclosedBlockComment,
    ErrorUnclosedQIdent,

    LineComment,
    BlockComment,

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
    Identifier,
    String,
    UInt,
    UReal,
    Bool,
}

impl Debug for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use TokenKind as TK;
        match self {
            TK::EOF => write!(f, "EOF"),
            TK::Comma => write!(f, "','"),
            TK::Dot => write!(f, "'.''"),
            TK::Semicolon => write!(f, "';'"),
            TK::Colon => write!(f, "':'"),
            TK::LParen => write!(f, "'('"),
            TK::RParen => write!(f, "')'"),
            TK::LCurly => write!(f, "'{{'"),
            TK::RCurly => write!(f, "'}}'"),
            TK::LBracket => write!(f, "'['"),
            TK::RBracket => write!(f, "']'"),
            TK::Equal => write!(f, "'='"),
            TK::Assign => write!(f, "':='"),
            TK::Plus => write!(f, "'+'"),
            TK::Minus => write!(f, "'-'"),
            TK::Star => write!(f, "'*'"),
            TK::Slash => write!(f, "'/'"),
            TK::Flex => write!(f, "'^'"),
            TK::DotPlus => write!(f, "'.+'"),
            TK::DotMinus => write!(f, "'.-'"),
            TK::DotStar => write!(f, "'.*'"),
            TK::DotSlash => write!(f, "'./'"),
            TK::DotFlex => write!(f, "'.^'"),
            TK::Gre => write!(f, "'>'"),
            TK::Geq => write!(f, "'>='"),
            TK::Les => write!(f, "'<'"),
            TK::Leq => write!(f, "'<='"),
            TK::Neq => write!(f, "'<>'"),
            TK::Eq => write!(f, "'=='"),
            TK::Not => write!(f, "'not'"),
            TK::And => write!(f, "'and'"),
            TK::Or => write!(f, "'or'"),
            TK::In => write!(f, "'in'"),
            TK::For => write!(f, "for'"),
            TK::If => write!(f, "'if'"),
            TK::Else => write!(f, "'else'"),
            TK::ElseIf => write!(f, "'elseif'"),
            TK::Then => write!(f, "'then'"),
            TK::When => write!(f, "'when'"),
            TK::ElseWhen => write!(f, "'elsewhen'"),
            TK::While => write!(f, "'while'"),
            TK::Loop => write!(f, "'loop'"),
            TK::Break => write!(f, "'break'"),
            TK::Return => write!(f, "'return'"),
            TK::Partial => write!(f, "'partial'"),
            TK::Class => write!(f, "'class'"),
            TK::Operator => write!(f, "'operator'"),
            TK::Expandable => write!(f, "'expandable'"),
            TK::Model => write!(f, "'model'"),
            TK::Function => write!(f, "'function'"),
            TK::Record => write!(f, "'record'"),
            TK::Type => write!(f, "'type'"),
            TK::Block => write!(f, "'block'"),
            TK::Connector => write!(f, "'connector'"),
            TK::Package => write!(f, "'package'"),
            TK::Pure => write!(f, "'pure'"),
            TK::Impure => write!(f, "'impure'"),
            TK::Initial => write!(f, "'initial'"),
            TK::Equation => write!(f, "'equation'"),
            TK::Algorithm => write!(f, "'algorithm'"),
            TK::Extends => write!(f, "'extends'"),
            TK::Import => write!(f, "'import'"),
            TK::Public => write!(f, "'public'"),
            TK::Protected => write!(f, "'protected'"),
            TK::Within => write!(f, "'within'"),
            TK::Final => write!(f, "'final'"),
            TK::Encapsulated => write!(f, "'encapsulated'"),
            TK::Enumeration => write!(f, "'enumeration'"),
            TK::Input => write!(f, "'input'"),
            TK::Output => write!(f, "'output'"),
            TK::Redeclare => write!(f, "'redeclare'"),
            TK::Inner => write!(f, "'inner'"),
            TK::Outer => write!(f, "'outer'"),
            TK::Replaceable => write!(f, "'replaceable'"),
            TK::Constrainedby => write!(f, "'constrainedby'"),
            TK::Flow => write!(f, "'flow'"),
            TK::Stream => write!(f, "'stream'"),
            TK::Discrete => write!(f, "'discrete'"),
            TK::Parameter => write!(f, "'parameter'"),
            TK::Constant => write!(f, "'constant'"),
            TK::Each => write!(f, "'each'"),
            TK::Annotation => write!(f, "'annotation'"),
            TK::External => write!(f, "'external'"),
            TK::End => write!(f, "'end'"),
            TK::Der => write!(f, "'der'"),
            TK::Connect => write!(f, "'connect'"),
            TK::LineComment => write!(f, "LINE COMMENT"),
            TK::BlockComment => write!(f, "BLOCK COMMENT"),
            TK::Identifier => write!(f, "IDENTIFIER"),
            TK::String => write!(f, "STRING"),
            TK::UInt => write!(f, "UNSIGNED INTEGER"),
            TK::UReal => write!(f, "UNSIGNED REAL"),
            TK::Bool => write!(f, "'true' or 'false'"),
            _ => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Represents token position in the input string
///
/// * `offset`: offset in bytes from the start of the input string that
///   corresponds with this position
/// * `line`: line number that corresponds with this position
/// * `col`: column (number of characters from the last newline) that
///   corresponds with this position
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TokenID(usize);

#[derive(Debug, Clone)]
/// Represents a single Modelica token.
///
/// Tokens contain information on their type and their coordinates in
/// the source.
///
/// * `kind`: token's kind
/// * `text`: text content of the token
/// * `source`: source of the token (input string or file)
/// * `id`: ID of the toke; the same as token's order in the input
/// * `start`: starting position
/// * `end`: ending position
pub struct Token<'a> {
    /// Token's kind
    pub kind: TokenKind,
    /// Text of the token
    pub text: &'a str,
    /// Source of the token
    pub source: &'a str,
    /// ID of the toke; the same as token's order in the input
    pub id: TokenID,
    /// Starting position
    pub start: Position,
    /// Ending position
    pub end: Position,
}

pub struct Tokenized {
    /// Source name
    source: String,
    /// Source code
    text: String,
    /// Tokens' kinds
    kinds: Vec<TokenKind>,
    /// Token's starting offsets
    starts: Vec<usize>,
    /// Token's ending offsets
    ends: Vec<usize>,
}

impl Tokenized {
    pub fn new(source: String, text: String) -> Self {
        return Tokenized {
            source,
            text,
            kinds: Vec::new(),
            starts: Vec::new(),
            ends: Vec::new(),
        };
    }

    pub fn push(&mut self, kind: TokenKind, start: usize, end: usize) {
        self.kinds.push(kind);
        self.starts.push(start);
        self.ends.push(end);
    }

    pub fn first(&self) -> TokenID {
        TokenID(0)
    }

    pub fn last(&self) -> TokenID {
        TokenID(self.kinds.len() - 1)
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn kind(&self, i: TokenID) -> TokenKind {
        self.kinds[i.0]
    }

    pub fn all<'a>(&'a self) -> impl Iterator<Item = TokenID> + 'a {
        self.kinds
            .iter()
            .enumerate()
            .map(|(i, _)| TokenID(i))
    }

    pub fn tokens<'a>(&'a self) -> impl Iterator<Item = TokenID> + 'a {
        self.kinds
            .iter()
            .enumerate()
            .filter(|(_, k)| **k >= TokenKind::Comma)
            .map(|(i, _)| TokenID(i))
    }

    pub fn get(&self, i: TokenID) -> Token {
        let kind = self.kinds[i.0];
        let start = self.starts[i.0];
        let end = self.ends[i.0];
        let text = &self.text[start..end];
        let lines: Vec<&str> = text.split('\n').collect();
        let pre = &self.text[..start];
        let pre_lines: Vec<&str> = pre.split('\n').collect();
        let start_pos = Position {
            offset: start,
            line: pre_lines.len(),
            col: pre_lines.last().unwrap().chars().count(),
        };
        let end_pos = Position {
            offset: end,
            line: lines.len(),
            col: lines.last().unwrap().chars().count(),
        };
        Token {
            kind: kind.clone(),
            text,
            source: self.source.as_str(),
            id: i,
            start: start_pos,
            end: end_pos,
        }
    }
}
