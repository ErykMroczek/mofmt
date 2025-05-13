use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
/// Represents a type of a Modelica token. Defined based on Modelica
/// Specification
/// 3.7
pub enum TokenKind {
    Eof,

    // Custom kinds for error handling

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
            TK::Eof => write!(f, "EOF"),
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
/// # Fields
/// - `offset`: offset in bytes from the start of the input string that
///   corresponds with this position
/// - `line`: line number that corresponds with this position
/// - `col`: column (number of characters from the last newline) that
///   corresponds with this position
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
/// Represents a unique identifier for a token.
/// 
/// It acts as a opaque pointer to a token in the `Tokenized` collection.
pub struct TokenID(usize);

#[derive(Debug, Clone)]
/// Represents a token extracted from the Modelica source code.
///
/// A `Token` is a slice of the source code that has been identified as a meaningful unit, such as a keyword, identifier, or symbol.
/// It contains metadata about its type, position, and the source it was extracted from.
///
/// # Fields
/// - `kind`: The type of the token, represented by the `TokenKind` enum.
/// - `text`: A slice of the source code that represents the token's text.
/// - `source`: A reference to the entire source code from which the token was extracted.
/// - `id`: A unique identifier for the token within the `Tokenized` collection.
/// - `start`: The starting position of the token in the source code, represented by the `Position` struct.
/// - `end`: The ending position of the token in the source code, represented by the `Position` struct.
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub source: &'a str,
    pub id: TokenID,
    pub start: Position,
    pub end: Position,
}

/// Represents a collection of tokenized data extracted from the Modelica source code.
///
/// This structure has opaque API and does not expose the internal structure of the tokenized data.
/// It provides methods to navigate the tokens and access their properties.
pub struct Tokens {
    source: String,
    code: String,
    kinds: Vec<TokenKind>,
    starts: Vec<usize>,
    ends: Vec<usize>,
}

impl Tokens {

    pub(super) fn new(source: String, code: String) -> Self {
        Tokens {
            source,
            code,
            kinds: Vec::new(),
            starts: Vec::new(),
            ends: Vec::new(),
        }
    }

    pub(super) fn push(&mut self, kind: TokenKind, start: usize, end: usize) {
        self.kinds.push(kind);
        self.starts.push(start);
        self.ends.push(end);
    }

    /// Return first token ID
    pub fn first(&self) -> TokenID {
        TokenID(0)
    }

    /// Return last token ID
    pub fn last(&self) -> TokenID {
        TokenID(self.kinds.len() - 1)
    }

    /// Return next valid token ID
    pub fn next(&self, id: TokenID) -> Option<TokenID> {
        let next = id.0 + 1;
        if next < self.kinds.len() {
            Some(TokenID(next))
        } else {
            None
        }
    }

    /// Return previous valid token ID
    pub fn prev(&self, id: TokenID) -> Option<TokenID> {
        if id.0 > 0 {
            Some(TokenID(id.0 - 1))
        } else {
            None
        }
    }

    /// Return the source/file from which the tokens were extracted
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    /// Return the source code from which the tokens were extracted
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Return kind of the token
    pub fn kind(&self, i: TokenID) -> TokenKind {
        self.kinds[i.0]
    }

    /// Return the text contents of the token
    pub fn text(&self, i: TokenID) -> &str {
        let start = self.starts[i.0];
        let end = self.ends[i.0];
        &self.code[start..end]
    }

    /// Return the start position of the token
    pub fn start(&self, i: TokenID) -> Position {
        let start = self.starts[i.0];
        let pre = &self.code[..start];
        let lines: Vec<&str> = pre.split('\n').collect();
        Position {
            offset: start,
            line: lines.len(),
            col: lines.last().unwrap().chars().count() + 1,
        }
    }

    /// Return the end position of the token
    pub fn end(&self, i: TokenID) -> Position {
        let end = self.ends[i.0];
        let lines: Vec<&str> = self.code[..end].split('\n').collect();
        Position {
            offset: end,
            line: lines.len(),
            col: lines.last().unwrap().chars().count() + 1,
        }
    }

    /// Return all valid token IDs
    pub fn all(&self) -> Vec<TokenID> {
        (0..self.kinds.len()).map(TokenID).collect()
    }

    /// Return all valid token IDs that are not comments
    pub fn tokens(&self) -> Vec<TokenID> {
        self.kinds
            .iter()
            .enumerate()
            .filter(|(_, k)| **k >= TokenKind::Comma)
            .map(|(i, _)| TokenID(i))
            .collect()
    }

    /// Return all valid token IDs that are comments
    pub fn comments(&self) -> Vec<TokenID> {
        self.kinds
            .iter()
            .enumerate()
            .filter(|(_, k)| **k == TokenKind::LineComment || **k == TokenKind::BlockComment)
            .map(|(i, _)| TokenID(i))
            .collect()
    }

    /// Return formatted lexical error messages
    pub fn errors(&self) -> Vec<String> {
        self.kinds
            .iter()
            .enumerate()
            .filter(|(_, k)| {
                (TokenKind::ErrorIllegalCharacter..TokenKind::ErrorUnclosedQIdent).contains(*k)
            })
            .map(|(i, k)| {
                let start = self.start(TokenID(i));
                let c = self.code.chars().nth(start.offset).unwrap();
                
                match *k {
                    TokenKind::ErrorIllegalCharacter => format!(
                        "{}:{}:{}: illegal character '{c}'",
                        self.source, start.line, start.col
                    ),
                    TokenKind::ErrorIllegalQident => format!(
                        "{}:{}:{}: illegal character inside quoted identifier '{c}'",
                        self.source, start.line, start.col
                    ),
                    TokenKind::ErrorUnclosedString => format!(
                        "{}:{}:{}: unclosed string literal",
                        self.source, start.line, start.col
                    ),
                    TokenKind::ErrorUnclosedBlockComment => format!(
                        "{}:{}:{}: unclosed block comment",
                        self.source, start.line, start.col
                    ),
                    TokenKind::ErrorUnclosedQIdent => format!(
                        "{}:{}:{}: unclosed quoted identifier",
                        self.source, start.line, start.col
                    ),
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    /// Return the full token instance of a given ID
    pub fn get(&self, i: TokenID) -> Token {
        Token {
            kind: self.kind(i),
            text: self.text(i),
            source: self.source.as_str(),
            id: i,
            start: self.start(i),
            end: self.end(i),
        }
    }
}
