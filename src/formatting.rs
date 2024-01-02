use moparse::{Payload, SyntaxEvent, SyntaxKind, Terminal, Token, TokenCollection, TokenKind};

enum Marker {
    Token(usize),
    Comment(usize),
    Space,
    Indent,
    Dedent,
    Ignore,
    Blank,
    Break,
    Wrap,
}

struct Formatter<'a> {
    tokens: &'a TokenCollection,
    events: &'a Vec<SyntaxEvent>,
    markers: Vec<Marker>,
    prev_token: TokenKind,
    prev_line: usize,
    brackets: usize,
    groups: Vec<bool>,
    rules: Vec<SyntaxKind>,
    wraps: Vec<bool>,
}

const NO_SPACE_AFTER: [TokenKind; 6] = [
    TokenKind::LParen,
    TokenKind::Dot,
    TokenKind::LBracket,
    TokenKind::LCurly,
    TokenKind::Semi,
    TokenKind::Colon,
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
            markers: Vec::new(),
            prev_token: TokenKind::EOF,
            prev_line: 1,
            brackets: 0,
            groups: Vec::new(),
            rules: Vec::new(),
            wraps: Vec::new(),
        }
    }

    /// Handle comments and separate them if needed.
    fn handle_comments(&mut self, comments: Vec<&Token>, current_line: usize) {
        let mut line = self.prev_line;
        let mut diff = comments[0].end.line - line;
        if diff == 0 {
            // TODO: Handle inline comments
            ();
        }
        for comment in comments {
            diff = comment.start.line - line;
            if diff == 0 {
                self.markers.push(Marker::Space);
            } else if diff == 1 {
                self.markers.push(Marker::Break);
            } else {
                if self.prev_token == TokenKind::Semi {
                    self.markers.push(Marker::Blank);
                }
            }
            self.markers.push(Marker::Comment(comment.idx));
            line = comment.end.line;
        }
        diff = current_line - line;
        if self.prev_line == 1 {
            self.markers.push(Marker::Break);
        } else if diff == 1 {
            self.markers.push(Marker::Break);
        } else if diff > 1 {
            self.markers.push(Marker::Blank);
        }
    }

    /// Insert line break or space
    fn break_or_space(&mut self) {
        match self.groups.last() {
            Some(b) => {
                if *b {
                    self.markers.push(Marker::Break);
                } else {
                    if !NO_SPACE_AFTER.contains(&self.prev_token) {
                        self.markers.push(Marker::Space);
                    }
                }
            }
            None => {
                if !NO_SPACE_AFTER.contains(&self.prev_token) {
                    self.markers.push(Marker::Space);
                }
            }
        }
    }

    fn enter_group(&mut self, typ: SyntaxKind, first: &Token, last: &Token) {
        // Mark the group as broken if group was multiline
        if first.start.line > last.end.line {
            // Handle conditional expression
            if first.typ == TokenKind::If
                && [TokenKind::Equal, TokenKind::Assign].contains(&self.prev_token)
            {
                self.markers.push(Marker::Indent);
            // Handle matrix row
            } else if typ == SyntaxKind::ExpressionList {
                if !*self.groups.last().unwrap() {
                    self.markers.push(Marker::Indent);
                }
            } else {
                self.markers.push(Marker::Indent);
            }
            self.groups.push(true);
        } else {
            self.groups.push(false);
        }
    }

    fn exit_group(&mut self, enter: &Payload, exit: &Payload) {
        let group_broken = self.groups.pop().unwrap();
        if group_broken {
            // Handle conditional expression
            if self.tokens.get_token(enter.tok).unwrap().typ == TokenKind::If
                && [TokenKind::Equal, TokenKind::Assign]
                    .contains(&self.tokens.get_token(enter.tok - 1).unwrap().typ)
            {
                self.markers.push(Marker::Dedent);
            // Handle matrix row
            } else if exit.typ == SyntaxKind::ExpressionList {
                if !*self.groups.last().unwrap() {
                    self.markers.push(Marker::Dedent);
                }
            } else {
                self.markers.push(Marker::Dedent);
            }
        }
    }

    /// Place the line break at the i-th token
    fn wrap_expression(&mut self, i: usize) {
        let next_tok = self.tokens.get_token(i + 1).unwrap();
        // Check if there was a line break around the wrap point
        if next_tok.start.line > self.prev_line {
            // Consider only binary operators
            if [
                TokenKind::RBracket,
                TokenKind::RParen,
                TokenKind::RCurly,
                TokenKind::Ident,
                TokenKind::String,
                TokenKind::Uint,
                TokenKind::Ureal,
                TokenKind::True,
                TokenKind::False,
                TokenKind::End,
            ]
            .contains(&self.prev_token)
            {
                // Only indent if this is a first wrap
                let wrapped = self.wraps.last_mut().unwrap();
                if !*wrapped {
                    self.markers.push(Marker::Indent);
                    *wrapped = true;
                }
                self.markers.push(Marker::Wrap);
            }
        }
    }

    fn handle_token(&mut self, i: usize) {
        let tok = self.tokens.get_item(i).unwrap();
        let kind = tok.typ;
        let parent = *self.rules.last().unwrap();
        let comments = preceding_comments(self.tokens, tok.idx);
        if self.prev_token == TokenKind::Semi {
            if self.brackets == 0 {
                self.markers.push(Marker::Break);
            } else {
                self.markers.push(Marker::Space);
            }
            if comments.is_none() {
                if tok.start.line - self.prev_line > 1 && !NO_BREAK_BEFORE.contains(&kind) {
                    self.markers.push(Marker::Blank);
                }
            }
        }

        // Handle comments
        if comments.is_some() {
            self.handle_comments(comments.unwrap(), tok.start.line);
        }

        match kind {
            TokenKind::LBracket => {
                self.brackets += 1;
                if self.prev_token != TokenKind::Ident && !NO_SPACE_AFTER.contains(&self.prev_token)
                {
                    self.markers.push(Marker::Space);
                }
            }
            TokenKind::RBracket => self.brackets -= 1,
            TokenKind::For => self.break_or_space(),
            TokenKind::End => {
                // Handle end clause in class specifiers
                if parent == SyntaxKind::LongClassSpecifier {
                    self.markers.push(Marker::Blank);
                } else {
                    self.markers.push(Marker::Space);
                }
            }
            TokenKind::Elif | TokenKind::Else | TokenKind::Elwhen => {
                // Handle conditional expression context
                if parent == SyntaxKind::Expression {
                    self.break_or_space();
                } else if [
                    SyntaxKind::IfEquation,
                    SyntaxKind::IfStatement,
                    SyntaxKind::WhenEquation,
                    SyntaxKind::WhenStatement,
                ]
                .contains(&parent)
                {
                    self.markers.push(Marker::Break);
                }
            }
            TokenKind::If => {
                // Handle conditional expressions
                if *self.groups.last().unwrap()
                    && [TokenKind::Equal, TokenKind::Assign].contains(&self.prev_token)
                {
                    self.markers.push(Marker::Indent);
                    self.break_or_space();
                }
            }
            TokenKind::Dot => {
                // Only first dot in type specifiers etc. can be preceded with a space
                if ![TokenKind::Ident, TokenKind::LBracket].contains(&self.prev_token)
                    && !NO_SPACE_AFTER.contains(&self.prev_token)
                {
                    self.markers.push(Marker::Space);
                }
            }
            TokenKind::Protected | TokenKind::Public => self.markers.push(Marker::Blank),
            TokenKind::External => {
                self.markers.push(Marker::Indent);
                self.markers.push(Marker::Blank);
            }
            TokenKind::Plus
            | TokenKind::DotPlus
            | TokenKind::Minus
            | TokenKind::DotMinus
            | TokenKind::Star
            | TokenKind::DotStar
            | TokenKind::Slash
            | TokenKind::DotSlash
            | TokenKind::Flex
            | TokenKind::DotFlex
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Gre
            | TokenKind::Geq
            | TokenKind::Les
            | TokenKind::Leq
            | TokenKind::Eq
            | TokenKind::Neq => {
                self.wrap_expression(i);
            }
            _ => {
                if !NO_SPACE_BEFORE.contains(&kind) && !NO_SPACE_AFTER.contains(&self.prev_token) {
                    self.markers.push(Marker::Space);
                }
            }
        }

        self.markers.push(Marker::Token(tok.idx));

        match kind {
            TokenKind::Annotation => self.markers.push(Marker::Space),
            TokenKind::Then | TokenKind::Else | TokenKind::Loop => {
                if [
                    SyntaxKind::IfEquation,
                    SyntaxKind::IfStatement,
                    SyntaxKind::WhenEquation,
                    SyntaxKind::WhenStatement,
                    SyntaxKind::WhileStatement,
                    SyntaxKind::ForEquation,
                    SyntaxKind::ForStatement,
                ]
                .contains(&parent)
                {
                    self.markers.push(Marker::Break);
                }
            }
            TokenKind::Equation | TokenKind::Algorithm => {
                self.markers.push(Marker::Blank);
            }
            _ => (),
        }

        self.prev_token = kind;
        self.prev_line = tok.start.line;
    }

    fn walk_events(&mut self) {
        for idx in 0..self.events.len() {
            match &self.events[idx] {
                SyntaxEvent::Advance(t) => match t {
                    Terminal::Token(i) => self.handle_token(*i),
                    _ => (),
                },
                SyntaxEvent::Enter(p) => {
                    let first = self.tokens.get_token(p.tok).unwrap();
                    let last = self.events[p.pair].get_token(self.tokens);
                    let parent = self.rules.last().unwrap();
                    match p.typ {
                        SyntaxKind::DescriptionString
                        | SyntaxKind::AnnotationClause
                        | SyntaxKind::ConstrainingClause
                        | SyntaxKind::EnumerationLiteral => {
                            self.markers.push(Marker::Indent);
                            // Handle class annotations
                            if p.typ == SyntaxKind::AnnotationClause
                                && *parent == SyntaxKind::Composition
                            {
                                self.markers.push(Marker::Blank);
                            } else {
                                self.markers.push(Marker::Break);
                            }
                        }
                        SyntaxKind::Equation | SyntaxKind::Statement | SyntaxKind::Element => {
                            self.markers.push(Marker::Indent);
                        }
                        SyntaxKind::ElementList
                        | SyntaxKind::EquationSection
                        | SyntaxKind::AlgorithmSection => {
                            self.markers.push(Marker::Blank);
                        }
                        SyntaxKind::Primary => {
                            // Handle matrix or array
                            if [TokenKind::LBracket, TokenKind::LCurly].contains(&first.typ) {
                                self.enter_group(p.typ, first, last);
                            }
                        }
                        SyntaxKind::FunctionCallArgs
                        | SyntaxKind::ClassOrInheritanceModification
                        | SyntaxKind::ClassModification
                        | SyntaxKind::ArraySubscripts => {
                            self.enter_group(p.typ, first, last);
                            // self.markers.push(Marker::Ignore);
                        }
                        SyntaxKind::ExpressionList => {
                            self.break_or_space();
                            self.enter_group(p.typ, first, last);
                        }
                        SyntaxKind::Subscript
                        | SyntaxKind::NamedArgument
                        | SyntaxKind::Argument
                        | SyntaxKind::InheritanceModification
                        | SyntaxKind::FunctionArgumentsNonFirst
                        | SyntaxKind::FunctionArguments
                        | SyntaxKind::ArrayArguments
                        | SyntaxKind::ArrayArgumentsNonFirst => {
                            self.break_or_space();
                        }
                        SyntaxKind::Expression => {
                            if *parent == SyntaxKind::ExpressionList {
                                self.break_or_space();
                            // Handle conditional expression
                            } else if first.typ == TokenKind::If {
                                self.enter_group(p.typ, first, last);
                            // Handle conditional parts in conditional expression
                            } else if [TokenKind::Then, TokenKind::Else].contains(&self.prev_token)
                                && *parent == SyntaxKind::Expression
                            {
                                self.markers.push(Marker::Indent);
                                self.break_or_space();
                            }
                            self.wraps.push(false);
                        }
                        _ => (),
                    }
                    self.rules.push(p.typ);
                }
                SyntaxEvent::Exit(p_end) => {
                    let typ = self.rules.pop().unwrap();
                    let p_start;
                    match &self.events[p_end.pair] {
                        SyntaxEvent::Enter(enter) => p_start = enter,
                        _ => unreachable!(),
                    }
                    let first = self.tokens.get_token(p_start.tok).unwrap();
                    let last = self.tokens.get_token(p_end.tok).unwrap();
                    match typ {
                        // TODO: external element
                        SyntaxKind::DescriptionString
                        | SyntaxKind::AnnotationClause
                        | SyntaxKind::ConstrainingClause
                        | SyntaxKind::Equation
                        | SyntaxKind::Statement
                        | SyntaxKind::Element
                        | SyntaxKind::EnumerationLiteral => self.markers.push(Marker::Dedent),
                        SyntaxKind::Primary => {
                            // Handle matrix or array
                            if [TokenKind::RBracket, TokenKind::RCurly].contains(&last.typ) {
                                self.exit_group(p_start, p_end);
                            }
                        }
                        SyntaxKind::FunctionCallArgs
                        | SyntaxKind::ClassOrInheritanceModification
                        | SyntaxKind::ClassModification
                        | SyntaxKind::ArraySubscripts
                        | SyntaxKind::ExpressionList => self.exit_group(p_start, p_end),
                        SyntaxKind::Expression => {
                            // Handle conditional expression
                            if first.typ == TokenKind::If {
                                self.exit_group(p_start, p_end);
                            // Handle conditional part of the expression
                            } else if [TokenKind::Then, TokenKind::Else].contains(&first.typ)
                                && *self.rules.last().unwrap() == SyntaxKind::Expression
                            {
                                self.markers.push(Marker::Dedent);
                            }
                            let wrapped = self.wraps.pop().unwrap();
                            if wrapped {
                                self.markers.push(Marker::Dedent);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
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
        rest -= 1;
        if [TokenKind::LineComment, TokenKind::BlockComment].contains(&prev_item.typ) {
            comments.push(prev_item);
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
