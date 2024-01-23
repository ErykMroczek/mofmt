use std::net;

use crate::markers::{self, Marker, MarkerCollector};
use moparse::*;

pub fn format(tokens: &TokenCollection, events: &Vec<SyntaxEvent>) -> (Vec<Marker>, Vec<String>) {
    let mut fmt = Formatter::new(tokens, events);
    (fmt.markers.markers, fmt.errors)
}

struct Formatter<'a> {
    tokens: &'a TokenCollection,
    events: &'a Vec<SyntaxEvent>,
    markers: MarkerCollector,
    errors: Vec<String>,
    prev_token: TokenKind,
    prev_line: usize,
    brackets: usize,
    groups: Vec<bool>,
    rules: Vec<SyntaxKind>,
    wraps: Vec<bool>,
    pos: usize,
}

const NO_SPACE_AFTER: [TokenKind; 7] = [
    TokenKind::LParen,
    TokenKind::Dot,
    TokenKind::LBracket,
    TokenKind::LCurly,
    TokenKind::Semi,
    TokenKind::Colon,
    TokenKind::Connect,
];
const NO_SPACE_BEFORE: [TokenKind; 6] = [
    TokenKind::RParen,
    TokenKind::RBracket,
    TokenKind::RCurly,
    TokenKind::Semi,
    TokenKind::Comma,
    TokenKind::Colon,
];
const NO_BREAK_BEFORE: [TokenKind; 4] = [
    TokenKind::End,
    TokenKind::Else,
    TokenKind::Elif,
    TokenKind::Elwhen,
];

impl<'a> Formatter<'a> {
    fn new(tokens: &'a TokenCollection, events: &'a Vec<SyntaxEvent>) -> Self {
        Formatter {
            tokens,
            events,
            markers: MarkerCollector::new(),
            errors: Vec::new(),
            prev_token: TokenKind::EOF,
            prev_line: 1,
            brackets: 0,
            groups: Vec::new(),
            rules: Vec::new(),
            wraps: Vec::new(),
            pos: 0,
        }
    }

    /// Handle comments and separate them if needed.
    fn handle_comments(&mut self, comments: Vec<&Token>, current_line: usize) {
        let mut line = self.prev_line;
        let mut diff = comments[0].start.line - line;
        // Handle inline comments
        let tail = if diff == 0 {
            None
        } else {
            self.markers.cache_tail()
        };
        for comment in comments {
            diff = comment.start.line - line;
            if diff == 0 {
                self.markers.push(Marker::Space);
            } else if diff == 1 {
                self.markers.push(Marker::Break);
            } else {
                if self.prev_token == TokenKind::Semi || line > self.prev_line {
                    self.markers.push(Marker::Blank);
                }
            }
            self.markers.push(Marker::Comment(comment.idx));
            line = comment.start.line;
        }
        diff = current_line - line;
        if self.prev_line == 1 {
            self.markers.push(Marker::Break);
        } else if let Some(mut tail) = tail {
            self.markers.append(&mut tail);
        } else if diff == 1 {
            self.markers.push(Marker::Break);
        } else if diff > 1 {
            self.markers.push(Marker::Blank);
        }
    }

    fn next(&self) -> Option<&SyntaxEvent> {
        self.events.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn token(&self, i: usize) -> &Token {
        self.tokens.get_token(i).unwrap()
    }

    fn next_token(&self, i: usize) -> &Token {
        self.tokens
            .get_token(i + 1)
            .unwrap_or_else(|| self.token(i))
    }

    fn prev_token(&self, i: usize) -> &Token {
        self.tokens
            .get_token(i - 1)
            .unwrap_or_else(|| self.token(i))
    }

    fn is_multiline(&self) -> bool {
        if let SyntaxEvent::Enter(enter) = self.next().unwrap() {
            if let SyntaxEvent::Exit(exit) = self.events.get(enter.pair).unwrap() {
                let first = self.token(enter.tok);
                let last = self.token(exit.tok);
                return first.start.line < last.end.line;
            }
        }
        false
    }

    fn is_empty(&self) -> bool {
        if let SyntaxEvent::Enter(enter) = self.next().unwrap() {
            return enter.pair - self.pos == 1;
        }
        false
    }
}

/// Return comments that precedes the token of the given index
fn preceding_comments(tokens: &TokenCollection, i: usize) -> Option<Vec<&Token>> {
    // Check if the current token is not a first one
    if i == 0 {
        return None;
    }
    let mut rest = i - 1;
    let mut comments = Vec::new();
    loop {
        let prev_item = tokens.get_item(rest).unwrap();
        if [TokenKind::LineComment, TokenKind::BlockComment].contains(&prev_item.typ) {
            comments.push(prev_item);
        } else {
            break;
        }
        if rest > 0 {
            rest -= 1;
        } else {
            break;
        }
    }
    if comments.len() == 0 {
        None
    } else {
        comments.reverse();
        Some(comments)
    }
}

fn wrap(f: &Formatter, i: usize, wrapped: &mut bool, markers: &mut Vec<Marker>) {
    let prev = f.prev_token(i);
    let next = f.next_token(i);
    if next.start.line > prev.end.line {
        if !*wrapped {
            markers.push(Marker::Indent);
            *wrapped = true;
        }
        markers.push(Marker::Break);
    } else {
        markers.push(Marker::Space);
    }
}

fn format_prod(f: &mut Formatter, prod: SyntaxKind) -> Marker {
    match prod {
        SyntaxKind::Name => name(f),
        SyntaxKind::TypeSpecifier => type_specifier(f),
        SyntaxKind::ArraySubscripts => array_subscripts(f),
        SyntaxKind::Subscript => subscript(f),
        SyntaxKind::Description => description(f),
        SyntaxKind::DescriptionString => description_string(f),
        SyntaxKind::AnnotationClause => annotation_clause(f),
        SyntaxKind::Error => error(f),
        _ => Marker::Sequence(Vec::new()),
    }
}
// Format `TypeSpecifier` production. There are no spaces between tokens.
fn type_specifier(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => markers.push(format_prod(f, p.typ)),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(i) => {
                    let tok = f.token(*i);
                    markers.push(Marker::Token(tok.idx));
                }
                Terminal::Error { .. } => unreachable!("Error in `TypeSpecifier` production"),
            },
        }
        f.advance();
    }
    Marker::Sequence(markers)
}

// Format `Name` production. There are no spaces between tokens.
fn name(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    let is_multiline = f.is_multiline();
    if is_multiline {
        markers.push(Marker::Indent);
    }
    f.advance();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => markers.push(format_prod(f, p.typ)),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(i) => {
                    let tok = f.token(*i);
                    if is_multiline && tok.typ == TokenKind::Dot {
                        markers.push(Marker::Break);
                    }
                    markers.push(Marker::Token(tok.idx));
                }
                Terminal::Error { .. } => unreachable!("Error in `Name` production"),
            },
        }
        f.advance();
    }
    if is_multiline {
        markers.push(Marker::Dedent);
    }
    Marker::Sequence(markers)
}

fn array_subscripts(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    let is_multiline = f.is_multiline();
    if is_multiline {
        markers.push(Marker::Indent);
    }
    f.advance();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => markers.push(format_prod(f, p.typ)),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(i) => {
                    let tok = f.token(*i);
                    markers.push(Marker::Token(tok.idx));
                    match tok.typ {
                        TokenKind::Comma => {
                            if is_multiline {
                                markers.push(Marker::Break);
                            } else {
                                markers.push(Marker::Space);
                            }
                        }
                        TokenKind::LBracket => {
                            if is_multiline {
                                markers.push(Marker::Break);
                            }
                        }
                        _ => (),
                    }
                }
                Terminal::Error { .. } => unreachable!("Error in `Subscript` production"),
            },
        }
        f.advance();
    }
    if is_multiline {
        markers.push(Marker::Dedent);
    }
    Marker::Sequence(markers)
}

// Format `Subscript` production.
fn subscript(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    f.advance();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => markers.push(format_prod(f, p.typ)),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(i) => markers.push(Marker::Token(*i)),
                Terminal::Error { .. } => unreachable!("Error in `Subscript` production"),
            },
        }
        f.advance();
    }
    markers.pop().unwrap()
}

// Format `Description` production.
fn description(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    f.advance();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => {
                if let Some(Marker::Sequence(string)) = markers.first() {
                    if string.len() > 0 {
                        markers.push(Marker::Break);
                    }
                }
                markers.push(format_prod(f, p.typ));
            }
            SyntaxEvent::Advance(_) => unreachable!("Terminal in `Description` production"),
        }
        f.advance();
    }
    Marker::Sequence(markers)
}

// Format `DescriptionString` production.
fn description_string(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    f.advance();
    let mut wrapped = false;
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => markers.push(format_prod(f, p.typ)),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(i) => {
                    let tok = f.token(*i);
                    if tok.typ == TokenKind::Plus {
                        wrap(f, *i, &mut wrapped, &mut markers);
                        markers.push(Marker::Token(tok.idx));
                        markers.push(Marker::Space);
                    } else {
                        markers.push(Marker::Token(tok.idx));
                    }
                }
                Terminal::Error { .. } => unreachable!("Error in `DescriptionString` production"),
            },
        }
        f.advance();
    }
    if wrapped {
        markers.push(Marker::Dedent);
    }
    Marker::Sequence(markers)
}

// Format `AnnotationClause` production.
fn annotation_clause(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    f.advance();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(p) => markers.push(format_prod(f, p.typ)),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(i) => {
                    let tok = f.token(*i);
                    markers.push(Marker::Token(tok.idx));
                    markers.push(Marker::Space);
                }
                Terminal::Error { .. } => unreachable!("Error in `AnnotationClause` production"),
            },
        }
        f.advance();
    }
    Marker::Sequence(markers)
}

// Format `Error` production and store error message
fn error(f: &mut Formatter) -> Marker {
    let mut markers = Vec::new();
    f.advance();
    while let Some(event) = f.next() {
        match event {
            SyntaxEvent::Exit(_) => break,
            SyntaxEvent::Enter(_) => unreachable!("Another production in `Error`"),
            SyntaxEvent::Advance(terminal) => match terminal {
                Terminal::Token(_) => unreachable!("Token in `Error` production"),
                Terminal::Error { msg, tok } => {
                    let bad_tok = f.tokens.get_token(*tok).unwrap();
                    f.errors.push(format!(
                        "{}:{}: {}",
                        bad_tok.start.line, bad_tok.start.col, msg
                    ));
                    markers.push(Marker::Space);
                    markers.push(Marker::Token(bad_tok.idx));
                    markers.push(Marker::Space);
                }
            },
        }
        f.advance();
    }
    Marker::Sequence(markers)
}
