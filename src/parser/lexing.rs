use super::tokens::{ModelicaToken, Position, Token};
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
                pos: 0,
                line: 1,
                col: 1,
            },
            current: Position {
                pos: 0,
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
                self.current.pos = i + 1;
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
    fn generate_token(&mut self, kind: ModelicaToken) {
        let start = self.start;
        let end = self.current;
        let token = Token {
            idx: self.count,
            text: String::from(&self.source[start.pos..end.pos]),
            kind,
            start,
            end,
        };
        if kind == ModelicaToken::LineComment || kind == ModelicaToken::BlockComment {
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
    fn accept_seq(&mut self, s: &str) {
        while self.accept(s) {}
    }

    /// Top-level lexing procedure
    fn lex_source(&mut self) {
        if let Some(c) = self.next() {
            match c {
                ';' => self.generate_token(ModelicaToken::Semicolon),
                ',' => self.generate_token(ModelicaToken::Comma),
                '+' => self.generate_token(ModelicaToken::Plus),
                '-' => self.generate_token(ModelicaToken::Minus),
                '*' => self.generate_token(ModelicaToken::Star),
                '^' => self.generate_token(ModelicaToken::Flex),
                '(' => self.generate_token(ModelicaToken::LParen),
                '{' => self.generate_token(ModelicaToken::LCurly),
                '[' => self.generate_token(ModelicaToken::LBracket),
                ')' => self.generate_token(ModelicaToken::RParen),
                '}' => self.generate_token(ModelicaToken::RCurly),
                ']' => self.generate_token(ModelicaToken::RBracket),
                ':' => self.lex_colon(),
                '=' => self.lex_equal(),
                '<' => self.lex_lesser(),
                '>' => self.lex_greater(),
                '.' => self.lex_dot(),
                '"' => self.lex_string(),
                '\'' => self.lex_qident(),
                '/' => self.lex_slash(),
                _ => {
                    if c.is_whitespace() {
                        return self.lex_space();
                    } else if c.is_numeric() {
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
            return self.generate_token(ModelicaToken::Assign);
        }
        self.generate_token(ModelicaToken::Colon)
    }

    /// Scan the slice that starts with `=`
    fn lex_equal(&mut self) {
        if self.accept("=") {
            return self.generate_token(ModelicaToken::Eq);
        }
        self.generate_token(ModelicaToken::Equal)
    }

    /// Scan the slice that starts with `<`
    fn lex_lesser(&mut self) {
        if self.accept(">") {
            self.generate_token(ModelicaToken::Neq)
        } else if self.accept("=") {
            self.generate_token(ModelicaToken::Leq)
        } else {
            self.generate_token(ModelicaToken::Les)
        }
    }

    /// Scan the slice that starts with `>`
    fn lex_greater(&mut self) {
        if self.accept("=") {
            return self.generate_token(ModelicaToken::Geq);
        }
        self.generate_token(ModelicaToken::Gre)
    }

    /// Scan the slice that starts with `.`
    fn lex_dot(&mut self) {
        if self.accept("+") {
            self.generate_token(ModelicaToken::DotPlus)
        } else if self.accept("-") {
            self.generate_token(ModelicaToken::DotMinus)
        } else if self.accept("*") {
            self.generate_token(ModelicaToken::DotStar)
        } else if self.accept("/") {
            self.generate_token(ModelicaToken::DotSlash)
        } else if self.accept("^") {
            self.generate_token(ModelicaToken::DotFlex)
        } else {
            self.generate_token(ModelicaToken::Dot)
        }
    }

    /// Scan the slice that is supposed to be a string
    fn lex_string(&mut self) {
        while let Some(c) = self.next() {
            match c {
                // Skip the escaped character
                '\\' => _ = self.next(),
                '"' => return self.generate_token(ModelicaToken::String),
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
                '\'' => return self.generate_token(ModelicaToken::Identifier),
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
                _ => self.generate_token(ModelicaToken::Slash),
            }
        } else {
            self.generate_token(ModelicaToken::Slash);
        }
    }

    /// Scan the slice that is supposed to be a line comment
    fn lex_linecomment(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '\r' | '\n' => return self.generate_token(ModelicaToken::LineComment),
                _ => _ = self.next(),
            }
        }
        self.generate_token(ModelicaToken::LineComment);
    }

    /// Scan the slice that is supposed to be a block comment
    fn lex_blockcomment(&mut self) {
        while let Some(c) = self.next() {
            if c == '*' {
                if let Some(c) = self.peek() {
                    if c == '/' {
                        self.next();
                        return self.generate_token(ModelicaToken::BlockComment);
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
            if c.is_whitespace() {
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
        const DIGITS: &str = "0123456789";
        self.accept_seq(DIGITS);
        if !self.accept(".") {
            if !self.accept("eE") {
                return self.generate_token(ModelicaToken::UInt);
            }
            self.accept("+-");
            self.accept_seq(DIGITS);
            return self.generate_token(ModelicaToken::UReal);
        }
        self.accept_seq(DIGITS);
        if self.accept("eE") {
            self.accept("+-");
            self.accept_seq(DIGITS);
        }
        self.generate_token(ModelicaToken::UReal)
    }

    /// Scan the slice that is supposed to be an indentifier or a keyword
    fn lex_nondigit(&mut self) {
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                break;
            }
            self.next();
        }
        let word: &str = &self.source[self.start.pos..self.current.pos];
        match word {
            "not" => self.generate_token(ModelicaToken::Not),
            "and" => self.generate_token(ModelicaToken::And),
            "or" => self.generate_token(ModelicaToken::Or),
            "in" => self.generate_token(ModelicaToken::In),
            "for" => self.generate_token(ModelicaToken::For),
            "if" => self.generate_token(ModelicaToken::If),
            "else" => self.generate_token(ModelicaToken::Else),
            "elseif" => self.generate_token(ModelicaToken::ElseIf),
            "then" => self.generate_token(ModelicaToken::Then),
            "when" => self.generate_token(ModelicaToken::When),
            "elsewhen" => self.generate_token(ModelicaToken::ElseWhen),
            "while" => self.generate_token(ModelicaToken::While),
            "loop" => self.generate_token(ModelicaToken::Loop),
            "break" => self.generate_token(ModelicaToken::Break),
            "return" => self.generate_token(ModelicaToken::Return),
            "partial" => self.generate_token(ModelicaToken::Partial),
            "class" => self.generate_token(ModelicaToken::Class),
            "operator" => self.generate_token(ModelicaToken::Operator),
            "expandable" => self.generate_token(ModelicaToken::Expandable),
            "model" => self.generate_token(ModelicaToken::Model),
            "function" => self.generate_token(ModelicaToken::Function),
            "record" => self.generate_token(ModelicaToken::Record),
            "type" => self.generate_token(ModelicaToken::Type),
            "block" => self.generate_token(ModelicaToken::Block),
            "connector" => self.generate_token(ModelicaToken::Connector),
            "package" => self.generate_token(ModelicaToken::Package),
            "pure" => self.generate_token(ModelicaToken::Pure),
            "impure" => self.generate_token(ModelicaToken::Impure),
            "end" => self.generate_token(ModelicaToken::End),
            "der" => self.generate_token(ModelicaToken::Der),
            "connect" => self.generate_token(ModelicaToken::Connect),
            "initial" => self.generate_token(ModelicaToken::Initial),
            "equation" => self.generate_token(ModelicaToken::Equation),
            "algorithm" => self.generate_token(ModelicaToken::Algorithm),
            "extends" => self.generate_token(ModelicaToken::Extends),
            "import" => self.generate_token(ModelicaToken::Import),
            "public" => self.generate_token(ModelicaToken::Public),
            "protected" => self.generate_token(ModelicaToken::Protected),
            "within" => self.generate_token(ModelicaToken::Within),
            "final" => self.generate_token(ModelicaToken::Final),
            "encapsulated" => self.generate_token(ModelicaToken::Encapsulated),
            "enumeration" => self.generate_token(ModelicaToken::Enumeration),
            "input" => self.generate_token(ModelicaToken::Input),
            "output" => self.generate_token(ModelicaToken::Output),
            "redeclare" => self.generate_token(ModelicaToken::Redeclare),
            "inner" => self.generate_token(ModelicaToken::Inner),
            "outer" => self.generate_token(ModelicaToken::Outer),
            "replaceable" => self.generate_token(ModelicaToken::Replaceable),
            "constrainedby" => self.generate_token(ModelicaToken::Constrainedby),
            "flow" => self.generate_token(ModelicaToken::Flow),
            "stream" => self.generate_token(ModelicaToken::Stream),
            "discrete" => self.generate_token(ModelicaToken::Discrete),
            "parameter" => self.generate_token(ModelicaToken::Parameter),
            "constant" => self.generate_token(ModelicaToken::Constant),
            "each" => self.generate_token(ModelicaToken::Each),
            "annotation" => self.generate_token(ModelicaToken::Annotation),
            "external" => self.generate_token(ModelicaToken::External),
            "true" | "false" => self.generate_token(ModelicaToken::Bool),
            _ => self.generate_token(ModelicaToken::Identifier),
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
        assert_eq!(tokens[0].kind, ModelicaToken::Within);
        assert_eq!(tokens[0].start.line, 1);
        assert_eq!(tokens[1].kind, ModelicaToken::Identifier);
        assert_eq!(tokens.last().unwrap().text, ";");
        assert_eq!(tokens.last().unwrap().kind, ModelicaToken::Semicolon);
        assert_eq!(comments[0].kind, ModelicaToken::LineComment);
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
