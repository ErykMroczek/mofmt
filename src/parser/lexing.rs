use std::fmt::{Debug, Error, Formatter};

/// Return collections of Modelica tokens, comments and errors generated from the input.
pub fn tokenize(source: &str) -> Tokens {
    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    lexer.tokens
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
/// Represents a type of a Modelica token. Defined based on Modelica
/// Specification
/// 3.7
pub enum TokenKind {
    EOF,

    ErrorChar,
    ErrorUnclosed,

    Space,
    Tab,
    EOL,

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
        match self {
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::ErrorChar | TokenKind::ErrorUnclosed => write!(f, "ERROR"),
            TokenKind::EOL => write!(f, "EOL"),
            TokenKind::Space => write!(f, "SPACE"),
            TokenKind::Tab => write!(f, "TAB"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Dot => write!(f, "'.''"),
            TokenKind::Semicolon => write!(f, "';'"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::LParen => write!(f, "'('"),
            TokenKind::RParen => write!(f, "')'"),
            TokenKind::LCurly => write!(f, "'{{'"),
            TokenKind::RCurly => write!(f, "'}}'"),
            TokenKind::LBracket => write!(f, "'['"),
            TokenKind::RBracket => write!(f, "']'"),
            TokenKind::Equal => write!(f, "'='"),
            TokenKind::Assign => write!(f, "':='"),
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Star => write!(f, "'*'"),
            TokenKind::Slash => write!(f, "'/'"),
            TokenKind::Flex => write!(f, "'^'"),
            TokenKind::DotPlus => write!(f, "'.+'"),
            TokenKind::DotMinus => write!(f, "'.-'"),
            TokenKind::DotStar => write!(f, "'.*'"),
            TokenKind::DotSlash => write!(f, "'./'"),
            TokenKind::DotFlex => write!(f, "'.^'"),
            TokenKind::Gre => write!(f, "'>'"),
            TokenKind::Geq => write!(f, "'>='"),
            TokenKind::Les => write!(f, "'<'"),
            TokenKind::Leq => write!(f, "'<='"),
            TokenKind::Neq => write!(f, "'<>'"),
            TokenKind::Eq => write!(f, "'=='"),
            TokenKind::Not => write!(f, "'not'"),
            TokenKind::And => write!(f, "'and'"),
            TokenKind::Or => write!(f, "'or'"),
            TokenKind::In => write!(f, "'in'"),
            TokenKind::For => write!(f, "for'"),
            TokenKind::If => write!(f, "'if'"),
            TokenKind::Else => write!(f, "'else'"),
            TokenKind::ElseIf => write!(f, "'elseif'"),
            TokenKind::Then => write!(f, "'then'"),
            TokenKind::When => write!(f, "'when'"),
            TokenKind::ElseWhen => write!(f, "'elsewhen'"),
            TokenKind::While => write!(f, "'while'"),
            TokenKind::Loop => write!(f, "'loop'"),
            TokenKind::Break => write!(f, "'break'"),
            TokenKind::Return => write!(f, "'return'"),
            TokenKind::Partial => write!(f, "'partial'"),
            TokenKind::Class => write!(f, "'class'"),
            TokenKind::Operator => write!(f, "'operator'"),
            TokenKind::Expandable => write!(f, "'expandable'"),
            TokenKind::Model => write!(f, "'model'"),
            TokenKind::Function => write!(f, "'function'"),
            TokenKind::Record => write!(f, "'record'"),
            TokenKind::Type => write!(f, "'type'"),
            TokenKind::Block => write!(f, "'block'"),
            TokenKind::Connector => write!(f, "'connector'"),
            TokenKind::Package => write!(f, "'package'"),
            TokenKind::Pure => write!(f, "'pure'"),
            TokenKind::Impure => write!(f, "'impure'"),
            TokenKind::Initial => write!(f, "'initial'"),
            TokenKind::Equation => write!(f, "'equation'"),
            TokenKind::Algorithm => write!(f, "'algorithm'"),
            TokenKind::Extends => write!(f, "'extends'"),
            TokenKind::Import => write!(f, "'import'"),
            TokenKind::Public => write!(f, "'public'"),
            TokenKind::Protected => write!(f, "'protected'"),
            TokenKind::Within => write!(f, "'within'"),
            TokenKind::Final => write!(f, "'final'"),
            TokenKind::Encapsulated => write!(f, "'encapsulated'"),
            TokenKind::Enumeration => write!(f, "'enumeration'"),
            TokenKind::Input => write!(f, "'input'"),
            TokenKind::Output => write!(f, "'output'"),
            TokenKind::Redeclare => write!(f, "'redeclare'"),
            TokenKind::Inner => write!(f, "'inner'"),
            TokenKind::Outer => write!(f, "'outer'"),
            TokenKind::Replaceable => write!(f, "'replaceable'"),
            TokenKind::Constrainedby => write!(f, "'constrainedby'"),
            TokenKind::Flow => write!(f, "'flow'"),
            TokenKind::Stream => write!(f, "'stream'"),
            TokenKind::Discrete => write!(f, "'discrete'"),
            TokenKind::Parameter => write!(f, "'parameter'"),
            TokenKind::Constant => write!(f, "'constant'"),
            TokenKind::Each => write!(f, "'each'"),
            TokenKind::Annotation => write!(f, "'annotation'"),
            TokenKind::External => write!(f, "'external'"),
            TokenKind::End => write!(f, "'end'"),
            TokenKind::Der => write!(f, "'der'"),
            TokenKind::Connect => write!(f, "'connect'"),
            TokenKind::LineComment => write!(f, "LINE COMMENT"),
            TokenKind::BlockComment => write!(f, "BLOCK COMMENT"),
            TokenKind::Identifier => write!(f, "IDENTIFIER"),
            TokenKind::String => write!(f, "STRING"),
            TokenKind::UInt => write!(f, "UNSIGNED INTEGER"),
            TokenKind::UReal => write!(f, "UNSIGNED REAL"),
            TokenKind::Bool => write!(f, "'true' or 'false'"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Represents a single Modelica token.
struct Tokens {
    kinds: Vec<TokenKind>,
    starts: Vec<usize>,
    ends: Vec<usize>,
    toks: Vec<usize>,
}

impl Tokens {
    fn new() -> Self {
        Tokens { kinds: Vec::new(), starts: Vec::new(), ends: Vec::new(), toks: Vec::new() }
    }
    fn push(&mut self, kind: TokenKind, start: usize, end: usize) {
        if kind > TokenKind::BlockComment {
            self.toks.push(self.kinds.len());
        }
        self.kinds.push(kind);
        self.starts.push(start);
        self.ends.push(end);
    }
}

/// Represents Modelica lexer/scanner.
struct Lexer<'a> {
    source: &'a str,
    byts: &'a [u8],
    start: usize,
    current: usize,
    tokens: Tokens,
}

impl<'a> Lexer<'a> {
    /// Return new `Lexer` instance
    ///
    /// Lexer is initialized and positioned at the beginning of the
    /// source code
    ///
    /// * source - reference to the source string
    fn new(source: &'a str) -> Self {
        return Lexer {
            source: source,
            byts: source.as_bytes(),
            start: 0,
            current: 0,
            tokens: Tokens::new(),
        };
    }

    /// Collect tokens
    fn tokenize(&mut self) {
        while self.current < self.source.len() {
            self.lex_source();
        }
    }

    #[inline]
    /// Return next character from the input without consuming it
    fn peek(&mut self) -> Option<u8> {
        self.byts.get(self.current).map(|b| *b)
    }

    #[inline]
    /// Return next character from the input and consume it
    fn next(&mut self) -> Option<u8> {
        let b = self.peek();
        self.current += 1;
        b
    }

    /// Add a new token to the collection
    fn generate_token(&mut self, kind: TokenKind) {
        self.tokens.push(kind, self.start, self.current);
        self.start = self.current;
    }

    /// Return `true` if character is valid and consume it
    fn accept(&mut self, s: &[u8]) -> bool {
        if let Some(c) = self.peek() {
            if s.contains(&c) {
                self.next();
                return true;
            }
        }
        false
    }

    /// Consume a sequence of valid characters from the input
    #[inline]
    fn accept_digits(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.next();
        }
    }

    /// Top-level lexing procedure
    fn lex_source(&mut self) {
        if let Some(c) = self.next() {
            match c {
                b' ' => self.lex_space(),
                b';' => self.generate_token(TokenKind::Semicolon),
                b',' => self.generate_token(TokenKind::Comma),
                b'+' => self.generate_token(TokenKind::Plus),
                b'-' => self.generate_token(TokenKind::Minus),
                b'*' => self.generate_token(TokenKind::Star),
                b'^' => self.generate_token(TokenKind::Flex),
                b'(' => self.generate_token(TokenKind::LParen),
                b'{' => self.generate_token(TokenKind::LCurly),
                b'[' => self.generate_token(TokenKind::LBracket),
                b')' => self.generate_token(TokenKind::RParen),
                b'}' => self.generate_token(TokenKind::RCurly),
                b']' => self.generate_token(TokenKind::RBracket),
                b'\n' => self.generate_token(TokenKind::EOL),
                b'\r' => self.lex_eol(),
                b'\t' => self.lex_tab(),
                b':' => self.lex_colon(),
                b'=' => self.lex_equal(),
                b'<' => self.lex_lesser(),
                b'>' => self.lex_greater(),
                b'.' => self.lex_dot(),
                b'"' => self.lex_string(),
                b'\'' => self.lex_qident(),
                b'/' => self.lex_slash(),
                _ => {
                    if c.is_ascii_digit() {
                        return self.lex_numeral();
                    } else if c.is_ascii_alphabetic() || c == b'_' {
                        return self.lex_nondigit();
                    }
                    self.generate_token(TokenKind::ErrorChar)
                }
            }
        }
    }

    fn lex_eol(&mut self) {
        self.accept(&[b'\n']);
        self.generate_token(TokenKind::EOL)
    }

    /// Scan the slice that starts with `:`
    fn lex_colon(&mut self) {
        if self.accept(&[b'=']) {
            return self.generate_token(TokenKind::Assign);
        }
        self.generate_token(TokenKind::Colon)
    }

    /// Scan the slice that starts with `=`
    fn lex_equal(&mut self) {
        if self.accept(&[b'=']) {
            return self.generate_token(TokenKind::Eq);
        }
        self.generate_token(TokenKind::Equal)
    }

    /// Scan the slice that starts with `<`
    fn lex_lesser(&mut self) {
        if self.accept(&[b'>']) {
            self.generate_token(TokenKind::Neq)
        } else if self.accept(&[b'=']) {
            self.generate_token(TokenKind::Leq)
        } else {
            self.generate_token(TokenKind::Les)
        }
    }

    /// Scan the slice that starts with `>`
    fn lex_greater(&mut self) {
        if self.accept(&[b'=']) {
            return self.generate_token(TokenKind::Geq);
        }
        self.generate_token(TokenKind::Gre)
    }

    /// Scan the slice that starts with `.`
    fn lex_dot(&mut self) {
        if self.accept(&[b'+']) {
            self.generate_token(TokenKind::DotPlus)
        } else if self.accept(&[b'-']) {
            self.generate_token(TokenKind::DotMinus)
        } else if self.accept(&[b'*']) {
            self.generate_token(TokenKind::DotStar)
        } else if self.accept(&[b'/']) {
            self.generate_token(TokenKind::DotSlash)
        } else if self.accept(&[b'^']) {
            self.generate_token(TokenKind::DotFlex)
        } else {
            self.generate_token(TokenKind::Dot)
        }
    }

    /// Scan the slice that is supposed to be a string
    fn lex_string(&mut self) {
        while let Some(c) = self.next() {
            match c {
                // Skip the escaped character
                b'\\' => _ = self.next(),
                b'"' => return self.generate_token(TokenKind::String),
                _ => (),
            }
        }
        self.generate_token(TokenKind::ErrorUnclosed);
    }

    /// Scan the slice that is supposed to be a quoted identifier
    fn lex_qident(&mut self) {
        const ALLOWED: [u8; 28] = [
            b'!', b'#', b'$', b'%', b'&', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':',
            b';', b'<', b'>', b'=', b'?', b'@', b'[', b']', b'^', b'{', b'}', b'|', b'~', b'\"',
        ];
        while let Some(c) = self.next() {
            match c {
                b'\\' => _ = self.next(),
                b'\'' => return self.generate_token(TokenKind::Identifier),
                _ => {
                    if !(c.is_ascii_alphanumeric() || c == b'_' || ALLOWED.contains(&c)) {
                        return self.generate_token(TokenKind::ErrorChar);
                    }
                }
            }
        }
        self.generate_token(TokenKind::ErrorUnclosed);
    }

    /// Scan the slice that begins with `/`
    fn lex_slash(&mut self) {
        if let Some(c) = self.peek() {
            match c {
                b'/' => self.lex_linecomment(),
                b'*' => self.lex_blockcomment(),
                _ => self.generate_token(TokenKind::Slash),
            }
        } else {
            self.generate_token(TokenKind::Slash);
        }
    }

    /// Scan the slice that is supposed to be a line comment
    fn lex_linecomment(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b'\r' | b'\n' => return self.generate_token(TokenKind::LineComment),
                _ => _ = self.next(),
            }
        }
        self.generate_token(TokenKind::LineComment);
    }

    /// Scan the slice that is supposed to be a block comment
    fn lex_blockcomment(&mut self) {
        while let Some(c) = self.next() {
            if c == b'*' {
                if let Some(c) = self.peek() {
                    if c == b'/' {
                        self.next();
                        return self.generate_token(TokenKind::BlockComment);
                    }
                } else {
                    return self.generate_token(TokenKind::ErrorUnclosed);
                }
            }
        }
        self.generate_token(TokenKind::ErrorUnclosed);
    }

    /// Scan the slice of whitespace
    fn lex_space(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' {
                self.next();
            } else {
                break;
            }
        }
        self.generate_token(TokenKind::Space);
    }

    fn lex_tab(&mut self) {
        while let Some(c) = self.peek() {
            if c == b'\t' {
                self.next();
            } else {
                break;
            }
        }
        self.generate_token(TokenKind::Tab);
    }

    /// Scan the slice that is supposed to be a numeral
    fn lex_numeral(&mut self) {
        self.accept_digits();
        if !self.accept(&[b'.']) {
            if !self.accept(&[b'e', b'E']) {
                return self.generate_token(TokenKind::UInt);
            }
            self.accept(&[b'-', b'+']);
            self.accept_digits();
            return self.generate_token(TokenKind::UReal);
        }
        self.accept_digits();
        if self.accept(&[b'e', b'E']) {
            self.accept(&[b'-', b'+']);
            self.accept_digits();
        }
        self.generate_token(TokenKind::UReal)
    }

    /// Scan the slice that is supposed to be an indentifier or a keyword
    fn lex_nondigit(&mut self) {
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == b'_') {
                break;
            }
            self.next();
        }
        let word: &str = &self.source[self.start..self.current];
        match word {
            "not" => self.generate_token(TokenKind::Not),
            "and" => self.generate_token(TokenKind::And),
            "or" => self.generate_token(TokenKind::Or),
            "in" => self.generate_token(TokenKind::In),
            "for" => self.generate_token(TokenKind::For),
            "if" => self.generate_token(TokenKind::If),
            "else" => self.generate_token(TokenKind::Else),
            "elseif" => self.generate_token(TokenKind::ElseIf),
            "then" => self.generate_token(TokenKind::Then),
            "when" => self.generate_token(TokenKind::When),
            "elsewhen" => self.generate_token(TokenKind::ElseWhen),
            "while" => self.generate_token(TokenKind::While),
            "loop" => self.generate_token(TokenKind::Loop),
            "break" => self.generate_token(TokenKind::Break),
            "return" => self.generate_token(TokenKind::Return),
            "partial" => self.generate_token(TokenKind::Partial),
            "class" => self.generate_token(TokenKind::Class),
            "operator" => self.generate_token(TokenKind::Operator),
            "expandable" => self.generate_token(TokenKind::Expandable),
            "model" => self.generate_token(TokenKind::Model),
            "function" => self.generate_token(TokenKind::Function),
            "record" => self.generate_token(TokenKind::Record),
            "type" => self.generate_token(TokenKind::Type),
            "block" => self.generate_token(TokenKind::Block),
            "connector" => self.generate_token(TokenKind::Connector),
            "package" => self.generate_token(TokenKind::Package),
            "pure" => self.generate_token(TokenKind::Pure),
            "impure" => self.generate_token(TokenKind::Impure),
            "end" => self.generate_token(TokenKind::End),
            "der" => self.generate_token(TokenKind::Der),
            "connect" => self.generate_token(TokenKind::Connect),
            "initial" => self.generate_token(TokenKind::Initial),
            "equation" => self.generate_token(TokenKind::Equation),
            "algorithm" => self.generate_token(TokenKind::Algorithm),
            "extends" => self.generate_token(TokenKind::Extends),
            "import" => self.generate_token(TokenKind::Import),
            "public" => self.generate_token(TokenKind::Public),
            "protected" => self.generate_token(TokenKind::Protected),
            "within" => self.generate_token(TokenKind::Within),
            "final" => self.generate_token(TokenKind::Final),
            "encapsulated" => self.generate_token(TokenKind::Encapsulated),
            "enumeration" => self.generate_token(TokenKind::Enumeration),
            "input" => self.generate_token(TokenKind::Input),
            "output" => self.generate_token(TokenKind::Output),
            "redeclare" => self.generate_token(TokenKind::Redeclare),
            "inner" => self.generate_token(TokenKind::Inner),
            "outer" => self.generate_token(TokenKind::Outer),
            "replaceable" => self.generate_token(TokenKind::Replaceable),
            "constrainedby" => self.generate_token(TokenKind::Constrainedby),
            "flow" => self.generate_token(TokenKind::Flow),
            "stream" => self.generate_token(TokenKind::Stream),
            "discrete" => self.generate_token(TokenKind::Discrete),
            "parameter" => self.generate_token(TokenKind::Parameter),
            "constant" => self.generate_token(TokenKind::Constant),
            "each" => self.generate_token(TokenKind::Each),
            "annotation" => self.generate_token(TokenKind::Annotation),
            "external" => self.generate_token(TokenKind::External),
            "true" | "false" => self.generate_token(TokenKind::Bool),
            _ => self.generate_token(TokenKind::Identifier),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexing_correct_input() {
        let source = r#"within Some.Library;
        // Here goes a line comment!
        parameter Real x(start = 0) = if true then 1e-3 else -2
          "Some parameter";
        /* End there goes
        a block comment! */
        final constant Some.Type 'quoted'(min = 0, max = 1) = func.call(x);"#;
        let tokens = tokenize(source);
        assert_eq!(tokens.len(), 48);
        assert_eq!(tokens[0].kind, TokenKind::Within);
        assert_eq!(tokens[0].end - tokens[0].start, 6);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Semicolon);
    }

    #[test]
    fn lexing_erroneus_input() {
        let source = "Some.Name x1y_ = ! \"string\";";
        let tokens = tokenize(source);
        assert_eq!(tokens.len(), 8);
    }

    #[test]
    fn lexing_unicode_string() {
        let source = "String s := \"y̆stringąé\" foo ;";
        let tokens = tokenize(source);
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[3].kind, TokenKind::String);
    }

    #[test]
    fn lexing_block_comment() {
        let source = "/** comment **/";
        let tokens = tokenize(source);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.last().unwrap().kind, TokenKind::BlockComment);
    }
}


