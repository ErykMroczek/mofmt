use super::tokens::{TokenKind, Position, Token};
use std::{iter::Peekable, str::CharIndices};

/// Return collections of Modelica tokens, comments and errors generated from the input.
pub fn lex(name: &str, source: &str) -> (Vec<Token>, Vec<Token>, Vec<String>) {
    let mut lexer = Lexer::new(name, source);
    lexer.tokenize();
    (lexer.tokens, lexer.comments, lexer.errors)
}

/// Represents Modelica lexer/scanner.
struct Lexer<'a> {
    /// Source name
    name: &'a str,
    /// Source code
    source: &'a str,
    /// Iterator through source code characters
    chars: Peekable<CharIndices<'a>>,
    /// Starting position of currently constructed token
    start: Position,
    /// Current position of the lexer
    current: Position,
    /// Tokens collected so far
    tokens: Vec<Token>,
    /// Comments collected so far
    comments: Vec<Token>,
    /// Errors collected so far
    errors: Vec<String>,
    /// Tokens count
    count: usize,
    /// `true` if lexer reached the end of file
    at_eof: bool,
}

impl<'a> Lexer<'a> {
    /// Return new `Lexer` instance
    ///
    /// Lexer is initialized and positioned at the beginning of the
    /// source code
    ///
    /// * source - reference to the source string
    fn new(name: &'a str, source: &'a str) -> Self {
        return Lexer {
            name,
            source,
            chars: source.char_indices().peekable(),
            start: Position {
                offset: 0,
                line: 1,
                col: 1,
            },
            current: Position {
                offset: 0,
                line: 1,
                col: 1,
            },
            tokens: Vec::new(),
            comments: Vec::new(),
            errors: Vec::new(),
            at_eof: false,
            count: 0,
        };
    }

    /// Collect tokens
    fn tokenize(&mut self) {
        while !self.at_eof {
            self.lex_source();
        }
    }

    /// Return next character from the input without consuming it
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    /// Return next character from the input and consume it
    fn next(&mut self) -> Option<char> {
        match self.chars.next() {
            Some((i, c)) => {
                self.current.offset = i + 1;
                self.current.col += 1;
                if c == '\n' {
                    self.current.line += 1;
                    self.current.col = 1
                }
                Some(c)
            }
            None => {
                self.at_eof = true;
                None
            }
        }
    }

    /// Add a new token to the collection
    fn generate_token(&mut self, kind: TokenKind) {
        let start = self.start;
        let end = self.current;
        let token = Token {
            idx: self.count,
            text: String::from(&self.source[start.offset..end.offset]),
            kind,
            start,
            end,
        };
        if kind == TokenKind::LineComment || kind == TokenKind::BlockComment {
            self.comments.push(token);
        } else {
            self.tokens.push(token);
        }
        self.count += 1;
        self.jump();
    }

    /// Add a new error to the collection
    fn generate_error(&mut self, msg: String) {
        let start = self.start;
        let error = format!("{}:{}:{}: {}", self.name, start.line, start.col, msg);
        self.errors.push(error);
        self.jump();
    }

    /// Update the starting position for building the next token
    fn jump(&mut self) {
        self.start = self.current;
    }

    /// Return `true` if character is valid and consume it
    fn accept(&mut self, s: &str) -> bool {
        if let Some(c) = self.peek() {
            if s.contains(c) {
                self.next();
                return true;
            }
        }
        false
    }

    /// Consume a sequence of valid characters from the input
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
                ';' => self.generate_token(TokenKind::Semicolon),
                ',' => self.generate_token(TokenKind::Comma),
                '+' => self.generate_token(TokenKind::Plus),
                '-' => self.generate_token(TokenKind::Minus),
                '*' => self.generate_token(TokenKind::Star),
                '^' => self.generate_token(TokenKind::Flex),
                '(' => self.generate_token(TokenKind::LParen),
                '{' => self.generate_token(TokenKind::LCurly),
                '[' => self.generate_token(TokenKind::LBracket),
                ')' => self.generate_token(TokenKind::RParen),
                '}' => self.generate_token(TokenKind::RCurly),
                ']' => self.generate_token(TokenKind::RBracket),
                ':' => self.lex_colon(),
                '=' => self.lex_equal(),
                '<' => self.lex_lesser(),
                '>' => self.lex_greater(),
                '.' => self.lex_dot(),
                '"' => self.lex_string(),
                '\'' => self.lex_qident(),
                '/' => self.lex_slash(),
                _ => {
                    if c.is_ascii_whitespace() {
                        return self.lex_space();
                    } else if c.is_ascii_digit() {
                        return self.lex_numeral();
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        return self.lex_nondigit();
                    }
                    self.generate_error(format!("unexpected character: '{}'", c))
                }
            }
        }
    }

    /// Scan the slice that starts with `:`
    fn lex_colon(&mut self) {
        if self.accept("=") {
            return self.generate_token(TokenKind::Assign);
        }
        self.generate_token(TokenKind::Colon)
    }

    /// Scan the slice that starts with `=`
    fn lex_equal(&mut self) {
        if self.accept("=") {
            return self.generate_token(TokenKind::Eq);
        }
        self.generate_token(TokenKind::Equal)
    }

    /// Scan the slice that starts with `<`
    fn lex_lesser(&mut self) {
        if self.accept(">") {
            self.generate_token(TokenKind::Neq)
        } else if self.accept("=") {
            self.generate_token(TokenKind::Leq)
        } else {
            self.generate_token(TokenKind::Les)
        }
    }

    /// Scan the slice that starts with `>`
    fn lex_greater(&mut self) {
        if self.accept("=") {
            return self.generate_token(TokenKind::Geq);
        }
        self.generate_token(TokenKind::Gre)
    }

    /// Scan the slice that starts with `.`
    fn lex_dot(&mut self) {
        if self.accept("+") {
            self.generate_token(TokenKind::DotPlus)
        } else if self.accept("-") {
            self.generate_token(TokenKind::DotMinus)
        } else if self.accept("*") {
            self.generate_token(TokenKind::DotStar)
        } else if self.accept("/") {
            self.generate_token(TokenKind::DotSlash)
        } else if self.accept("^") {
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
                '\\' => _ = self.next(),
                '"' => return self.generate_token(TokenKind::String),
                _ => (),
            }
        }
        self.generate_error(String::from("unclosed string"));
    }

    /// Scan the slice that is supposed to be a quoted identifier
    fn lex_qident(&mut self) {
        const ALLOWED: &str = "!#$%&()*+,-./:;<>=?@[]^{}|~ \"";
        while let Some(c) = self.next() {
            match c {
                '\\' => _ = self.next(),
                '\'' => return self.generate_token(TokenKind::Identifier),
                _ => {
                    if !(c.is_ascii_alphanumeric() || c == '_' || ALLOWED.contains(c)) {
                        return self.generate_error(format!(
                            "unexpected character inside Q-IDENT: '{}'",
                            c
                        ));
                    }
                }
            }
        }
        self.generate_error(String::from("unclosed Q-IDENT"));
    }

    /// Scan the slice that begins with `/`
    fn lex_slash(&mut self) {
        if let Some(c) = self.peek() {
            match c {
                '/' => self.lex_linecomment(),
                '*' => self.lex_blockcomment(),
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
                '\r' | '\n' => return self.generate_token(TokenKind::LineComment),
                _ => _ = self.next(),
            }
        }
        self.generate_token(TokenKind::LineComment);
    }

    /// Scan the slice that is supposed to be a block comment
    fn lex_blockcomment(&mut self) {
        while let Some(c) = self.next() {
            if c == '*' {
                if let Some(c) = self.peek() {
                    if c == '/' {
                        self.next();
                        return self.generate_token(TokenKind::BlockComment);
                    }
                } else {
                    return self.generate_error(String::from("unclosed block comment"));
                }
            }
        }
        self.generate_error(String::from("unclosed block comment"));
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
        if !self.accept(".") {
            if !self.accept("eE") {
                return self.generate_token(TokenKind::UInt);
            }
            self.accept("+-");
            self.accept_digits();
            return self.generate_token(TokenKind::UReal);
        }
        self.accept_digits();
        if self.accept("eE") {
            self.accept("+-");
            self.accept_digits();
        }
        self.generate_token(TokenKind::UReal)
    }

    /// Scan the slice that is supposed to be an indentifier or a keyword
    fn lex_nondigit(&mut self) {
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                break;
            }
            self.next();
        }
        let word: &str = &self.source[self.start.offset..self.current.offset];
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
        let (tokens, comments, errors) = lex("none", source);
        assert_eq!(tokens.len(), 46);
        assert_eq!(comments.len(), 2);
        assert_eq!(tokens[0].text, "within");
        assert_eq!(tokens[0].kind, TokenKind::Within);
        assert_eq!(tokens[0].start.line, 1);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens.last().unwrap().text, ";");
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Semicolon);
        assert_eq!(comments[0].kind, TokenKind::LineComment);
        assert_eq!(tokens.last().unwrap().start.line, 7);
        assert_eq!(tokens[0].start.col, 1);
        assert_eq!(tokens[1].start.col, 8);
        assert_eq!(comments[0].start.col, 9);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn lexing_erroneus_input() {
        let source = "Some.Name x1y_ = ! \"string\";";
        let (tokens, _, errors) = lex("none", source);
        assert_eq!(tokens.len(), 7);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "none:1:18: unexpected character: '!'");
    }

    #[test]
    fn lexing_unicode_string() {
        let source = "String s := \"stringą\";";
        let (tokens, _, errors) = lex("none", source);
        assert_eq!(errors.len(), 0);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[3].text, "\"stringą\"");
    }

    #[test]
    fn lexing_block_comment() {
        let source = "/** comment **/";
        let (tokens, comments, errors) = lex("none", source);
        assert_eq!(errors.len(), 0);
        assert_eq!(tokens.len(), 0);
        assert_eq!(comments.len(), 1);
    }
}
