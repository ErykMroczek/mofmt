use super::tokens::{TokenKind, Tokenized};

/// Return collections of Modelica tokens, comments and errors generated from the input.
pub fn lex(name: String, source: String) -> Tokenized {
    let mut lexer = Lexer::new(name, source);
    lexer.tokenize();
    lexer.tokens
}

/// Represents Modelica lexer/scanner.
struct Lexer {
    /// Starting offset of currently constructed token
    start: usize,
    /// Current offset of the lexer
    current: usize,
    /// Tokens collected so far
    tokens: Tokenized,
    /// `true` if lexer reached the end of file
    at_eof: bool,
}

impl Lexer {
    /// Return new `Lexer` instance
    ///
    /// Lexer is initialized and positioned at the beginning of the
    /// source code
    ///
    /// * source - reference to the source string
    fn new(source: String, text: String) -> Self {
        return Lexer {
            start: 0,
            current: 0,
            tokens: Tokenized::new(source, text),
            at_eof: false,
        };
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn lexing_correct_input() {
//         let source = r#"within Some.Library;
//         // Here goes a line comment!
//         parameter Real x(start = 0) = if true then 1e-3 else -2
//           "Some parameter";
//         /* End there goes
//         a block comment! */
//         final constant Some.Type 'quoted'(min = 0, max = 1) = func.call(x);"#;
//         let tokens = lex(String::from("none"), String::from(source));
//         let comments: Vec<Token> = tokens
//             .kinds
//             .iter()
//             .enumerate()
//             .filter(|(_, k)| **k == TokenKind::BlockComment || **k == TokenKind::LineComment)
//             .map(|(i, _)| tokens.get(i).unwrap())
//             .collect();
//         assert_eq!(tokens.kinds.len(), 48);
//         assert_eq!(comments.len(), 2);
//         assert_eq!(tokens.get(0).unwrap().text, "within");
//         assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Within);
//         assert_eq!(tokens.get(0).unwrap().start.line, 1);
//         assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Identifier);
//         assert_eq!(tokens.get(tokens.kinds.len() - 1).unwrap().text, ";");
//         assert_eq!(
//             tokens.get(tokens.kinds.len() - 1).unwrap().kind,
//             TokenKind::Semicolon
//         );
//         assert_eq!(comments[0].kind, TokenKind::LineComment);
//         assert_eq!(tokens.get(tokens.kinds.len() - 1).unwrap().start.line, 7);
//         assert_eq!(tokens.get(0).unwrap().start.col, 1);
//         assert_eq!(tokens.get(1).unwrap().start.col, 8);
//         assert_eq!(comments[0].start.col, 9);
//         assert_eq!(errors.len(), 0);
//     }

//     #[test]
//     fn lexing_erroneus_input() {
//         let source = "Some.Name x1y_ = ! \"string\";";
//         let (tokens, _, errors) = lex("none", source);
//         assert_eq!(tokens.len(), 7);
//         assert_eq!(errors.len(), 1);
//         assert_eq!(errors[0], "none:1:18: unexpected character: '!'");
//     }

//     #[test]
//     fn lexing_unicode_string() {
//         let source = "String s := \"stringą\";";
//         let (tokens, _, errors) = lex("none", source);
//         assert_eq!(errors.len(), 0);
//         assert_eq!(tokens.len(), 5);
//         assert_eq!(tokens[3].text, "\"stringą\"");
//     }

//     #[test]
//     fn lexing_block_comment() {
//         let source = "/** comment **/";
//         let (tokens, comments, errors) = lex("none", source);
//         assert_eq!(errors.len(), 0);
//         assert_eq!(tokens.len(), 0);
//         assert_eq!(comments.len(), 1);
//     }
// }
