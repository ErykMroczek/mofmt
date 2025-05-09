/// Lexical analysis for Modelica source code.
///
/// This module provides functionality to tokenize Modelica source code into a sequence of tokens.
/// The main entry point is the `lex` function, which takes the source code as input and returns
/// a collection of tokens. The lexer processes the input character by character, identifying
/// keywords, symbols, literals, and other elements of the Modelica language.
///
/// # Tokenization Process
///
/// The lexer identifies various elements of the Modelica language, including:
///
/// - **Keywords**: Reserved words such as `if`, `else`, `while`, `class`, etc.
/// - **Identifiers**: User-defined names for variables, functions, and other entities.
/// - **Literals**: Numeric values, strings, and booleans.
/// - **Operators**: Arithmetic, logical, and comparison operators.
/// - **Delimiters**: Symbols such as parentheses, braces, and semicolons.
/// - **Comments**: Line comments (`//`) and block comments (`/* ... */`).
///
/// # Error Handling
///
/// The lexer detects and reports errors such as:
///
/// - Illegal characters.
/// - Unclosed strings or comments.
///
/// Errors are stored as special kind of tokens, and the lexer continues processing the input.
///
/// # Notes
///
/// The lexer assumes that the input is valid UTF-8. It handles escaped characters in strings
/// and quoted identifiers, and it ensures that tokens are aligned with character boundaries.
use super::tokens::{TokenKind, Tokenized};

/// Tokenizes the given Modelica source code.
///
/// The `lex` function is the main entry point for lexical analysis of Modelica source code.
/// It takes the name of the source and the source code itself as input, and returns a `Tokenized`
/// structure containing the tokens extracted from the source code.
pub fn lex(name: String, source: String) -> Tokenized {
    let mut lexer = Lexer::new(name, source);
    lexer.tokenize();
    lexer.tokens
}

/// Represents a lexer that processes Modelica code and generates tokens.
///
/// The `Lexer` struct keeps track of the current position in the input, the starting
/// position of the current token, the collection of tokens generated so far, and whether
/// the end of the input has been reached.
///
/// Fields:
/// - `start`: The starting position of the current lexeme in the input.
/// - `current`: The current position being analyzed in the input.
/// - `tokens`: A collection of tokens generated during the lexing process.
/// - `at_eof`: A flag indicating whether the lexer has reached the end of the input.
struct Lexer {
    start: usize,
    current: usize,
    tokens: Tokenized,
    at_eof: bool,
}

impl Lexer {
    /// Return new `Lexer` instance
    ///
    /// Lexer is initialized and positioned at the beginning of the
    /// source code
    fn new(source: String, text: String) -> Self {
        Lexer {
            start: 0,
            current: 0,
            tokens: Tokenized::new(source, text),
            at_eof: false,
        }
    }

    /// Collect tokens
    fn tokenize(&mut self) {
        while !self.at_eof {
            self.lex_source();
        }
    }

    /// Return next byte from the input without consuming it
    fn peek(&mut self) -> Option<u8> {
        self.tokens.code().as_bytes().get(self.current).cloned()
    }

    /// Return next byte from the input and consume it
    fn next(&mut self) -> Option<u8> {
        match self.peek() {
            Some(c) => {
                self.current += 1;
                Some(c)
            }
            None => {
                self.at_eof = true;
                None
            }
        }
    }

    /// Add a new token to the collection
    fn push_token(&mut self, kind: TokenKind) {
        self.tokens.push(kind, self.start, self.current);
        self.jump();
    }

    /// Update the starting position for building the next token
    #[inline(always)]
    fn jump(&mut self) {
        // If the current position is not a char boundary, we need to
        // find the next char boundary
        while !self.tokens.code().is_char_boundary(self.current) {
            self.current += 1;
        }
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

    /// Consume a sequence of digits
    #[inline(always)]
    fn accept_digits(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                return;
            }
            self.next();
        }
    }

    /// Top-level lexing procedure
    fn lex_source(&mut self) {
        if let Some(c) = self.next() {
            match c {
                b';' => self.push_token(TokenKind::Semicolon),
                b',' => self.push_token(TokenKind::Comma),
                b'+' => self.push_token(TokenKind::Plus),
                b'-' => self.push_token(TokenKind::Minus),
                b'*' => self.push_token(TokenKind::Star),
                b'^' => self.push_token(TokenKind::Flex),
                b'(' => self.push_token(TokenKind::LParen),
                b'{' => self.push_token(TokenKind::LCurly),
                b'[' => self.push_token(TokenKind::LBracket),
                b')' => self.push_token(TokenKind::RParen),
                b'}' => self.push_token(TokenKind::RCurly),
                b']' => self.push_token(TokenKind::RBracket),
                b':' => self.lex_colon(),
                b'=' => self.lex_equal(),
                b'<' => self.lex_lesser(),
                b'>' => self.lex_greater(),
                b'.' => self.lex_dot(),
                b'"' => self.lex_string(),
                b'\'' => self.lex_qident(),
                b'/' => self.lex_slash(),
                _ => {
                    if c.is_ascii_whitespace() {
                        return self.lex_space();
                    } else if c.is_ascii_digit() {
                        return self.lex_numeral();
                    } else if c.is_ascii_alphabetic() || c == b'_' {
                        return self.lex_nondigit();
                    }
                    self.push_token(TokenKind::ErrorIllegalCharacter);
                }
            }
        }
    }

    /// Scan the slice that starts with `:`
    fn lex_colon(&mut self) {
        if self.accept(&[b'=']) {
            return self.push_token(TokenKind::Assign);
        }
        self.push_token(TokenKind::Colon)
    }

    /// Scan the slice that starts with `=`
    fn lex_equal(&mut self) {
        if self.accept(&[b'=']) {
            return self.push_token(TokenKind::Eq);
        }
        self.push_token(TokenKind::Equal)
    }

    /// Scan the slice that starts with `<`
    fn lex_lesser(&mut self) {
        if self.accept(&[b'>']) {
            self.push_token(TokenKind::Neq)
        } else if self.accept(&[b'=']) {
            self.push_token(TokenKind::Leq)
        } else {
            self.push_token(TokenKind::Les)
        }
    }

    /// Scan the slice that starts with `>`
    fn lex_greater(&mut self) {
        if self.accept(&[b'=']) {
            return self.push_token(TokenKind::Geq);
        }
        self.push_token(TokenKind::Gre)
    }

    /// Scan the slice that starts with `.`
    fn lex_dot(&mut self) {
        if self.accept(&[b'+']) {
            self.push_token(TokenKind::DotPlus)
        } else if self.accept(&[b'-']) {
            self.push_token(TokenKind::DotMinus)
        } else if self.accept(&[b'*']) {
            self.push_token(TokenKind::DotStar)
        } else if self.accept(&[b'/']) {
            self.push_token(TokenKind::DotSlash)
        } else if self.accept(&[b'^']) {
            self.push_token(TokenKind::DotFlex)
        } else {
            self.push_token(TokenKind::Dot)
        }
    }

    /// Scan the slice that is supposed to be a string
    fn lex_string(&mut self) {
        while let Some(c) = self.next() {
            match c {
                // Skip the escaped character
                b'\\' => _ = self.next(),
                b'"' => return self.push_token(TokenKind::String),
                _ => (),
            }
        }
        self.push_token(TokenKind::ErrorUnclosedString);
    }

    /// Scan the slice that is supposed to be a quoted identifier
    fn lex_qident(&mut self) {
        const ALLOWED: &[u8] = "!#$%&()*+,-./:;<>=?@[]^{}|~ \"".as_bytes();
        while let Some(c) = self.next() {
            match c {
                b'\\' => _ = self.next(),
                b'\'' => return self.push_token(TokenKind::Identifier),
                _ => {
                    if !(c.is_ascii_alphanumeric() || c == b'_' || ALLOWED.contains(&c)) {
                        return self.push_token(TokenKind::ErrorIllegalQident);
                    }
                }
            }
        }
        self.push_token(TokenKind::ErrorUnclosedQIdent);
    }

    /// Scan the slice that begins with `/`
    fn lex_slash(&mut self) {
        if let Some(c) = self.peek() {
            match c {
                b'/' => self.lex_linecomment(),
                b'*' => self.lex_blockcomment(),
                _ => self.push_token(TokenKind::Slash),
            }
        } else {
            self.push_token(TokenKind::Slash);
        }
    }

    /// Scan the slice that is supposed to be a line comment
    fn lex_linecomment(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b'\r' | b'\n' => return self.push_token(TokenKind::LineComment),
                _ => _ = self.next(),
            }
        }
        self.push_token(TokenKind::LineComment);
    }

    /// Scan the slice that is supposed to be a block comment
    fn lex_blockcomment(&mut self) {
        while let Some(c) = self.next() {
            if c == b'*' {
                if let Some(c) = self.peek() {
                    if c == b'/' {
                        self.next();
                        return self.push_token(TokenKind::BlockComment);
                    }
                } else {
                    return self.push_token(TokenKind::ErrorUnclosedBlockComment);
                }
            }
        }
        self.push_token(TokenKind::ErrorUnclosedBlockComment);
    }

    /// Scan the slice of whitespace chars and discard them
    fn lex_space(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.next();
            } else {
                break;
            }
        }
        self.jump();
        self.lex_source();
    }

    /// Scan the slice that is supposed to be a numeral
    fn lex_numeral(&mut self) {
        self.accept_digits();
        if !self.accept(&[b'.']) {
            if !self.accept(&[b'e', b'E']) {
                return self.push_token(TokenKind::UInt);
            }
            self.accept(&[b'+', b'-']);
            self.accept_digits();
            return self.push_token(TokenKind::UReal);
        }
        self.accept_digits();
        if self.accept(&[b'e', b'E']) {
            self.accept(&[b'+', b'-']);
            self.accept_digits();
        }
        self.push_token(TokenKind::UReal)
    }

    /// Scan the slice that is supposed to be an indentifier or a keyword
    fn lex_nondigit(&mut self) {
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == b'_') {
                break;
            }
            self.next();
        }
        let word: &str = &self.tokens.code()[self.start..self.current];
        match word {
            "not" => self.push_token(TokenKind::Not),
            "and" => self.push_token(TokenKind::And),
            "or" => self.push_token(TokenKind::Or),
            "in" => self.push_token(TokenKind::In),
            "for" => self.push_token(TokenKind::For),
            "if" => self.push_token(TokenKind::If),
            "else" => self.push_token(TokenKind::Else),
            "elseif" => self.push_token(TokenKind::ElseIf),
            "then" => self.push_token(TokenKind::Then),
            "when" => self.push_token(TokenKind::When),
            "elsewhen" => self.push_token(TokenKind::ElseWhen),
            "while" => self.push_token(TokenKind::While),
            "loop" => self.push_token(TokenKind::Loop),
            "break" => self.push_token(TokenKind::Break),
            "return" => self.push_token(TokenKind::Return),
            "partial" => self.push_token(TokenKind::Partial),
            "class" => self.push_token(TokenKind::Class),
            "operator" => self.push_token(TokenKind::Operator),
            "expandable" => self.push_token(TokenKind::Expandable),
            "model" => self.push_token(TokenKind::Model),
            "function" => self.push_token(TokenKind::Function),
            "record" => self.push_token(TokenKind::Record),
            "type" => self.push_token(TokenKind::Type),
            "block" => self.push_token(TokenKind::Block),
            "connector" => self.push_token(TokenKind::Connector),
            "package" => self.push_token(TokenKind::Package),
            "pure" => self.push_token(TokenKind::Pure),
            "impure" => self.push_token(TokenKind::Impure),
            "end" => self.push_token(TokenKind::End),
            "der" => self.push_token(TokenKind::Der),
            "connect" => self.push_token(TokenKind::Connect),
            "initial" => self.push_token(TokenKind::Initial),
            "equation" => self.push_token(TokenKind::Equation),
            "algorithm" => self.push_token(TokenKind::Algorithm),
            "extends" => self.push_token(TokenKind::Extends),
            "import" => self.push_token(TokenKind::Import),
            "public" => self.push_token(TokenKind::Public),
            "protected" => self.push_token(TokenKind::Protected),
            "within" => self.push_token(TokenKind::Within),
            "final" => self.push_token(TokenKind::Final),
            "encapsulated" => self.push_token(TokenKind::Encapsulated),
            "enumeration" => self.push_token(TokenKind::Enumeration),
            "input" => self.push_token(TokenKind::Input),
            "output" => self.push_token(TokenKind::Output),
            "redeclare" => self.push_token(TokenKind::Redeclare),
            "inner" => self.push_token(TokenKind::Inner),
            "outer" => self.push_token(TokenKind::Outer),
            "replaceable" => self.push_token(TokenKind::Replaceable),
            "constrainedby" => self.push_token(TokenKind::Constrainedby),
            "flow" => self.push_token(TokenKind::Flow),
            "stream" => self.push_token(TokenKind::Stream),
            "discrete" => self.push_token(TokenKind::Discrete),
            "parameter" => self.push_token(TokenKind::Parameter),
            "constant" => self.push_token(TokenKind::Constant),
            "each" => self.push_token(TokenKind::Each),
            "annotation" => self.push_token(TokenKind::Annotation),
            "external" => self.push_token(TokenKind::External),
            "true" | "false" => self.push_token(TokenKind::Bool),
            _ => self.push_token(TokenKind::Identifier),
        }
    }
}
