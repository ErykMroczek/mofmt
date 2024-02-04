use std::{collections::hash_map::RandomState, iter::Peekable, vec::IntoIter};

use crate::{markers::Marker, tree::Child, tree::Tree};
use moparse::*;

pub fn format(tree: Tree, comments: Vec<Token>) -> Vec<Marker> {
    let mut f = Formatter::new(comments);
    match tree.kind {
        SyntaxKind::Argument => argument(&mut f, tree),
        SyntaxKind::ElementModificationOrReplaceable => {
            element_modification_or_replaceable(&mut f, tree)
        }
        SyntaxKind::ElementModification => element_modification(&mut f, tree),
        SyntaxKind::ElementRedeclaration => element_redeclaration(&mut f, tree),
        SyntaxKind::ElementReplaceable => element_replaceable(&mut f, tree),
        SyntaxKind::ComponentClause1 => component_clause1(&mut f, tree),
        SyntaxKind::ComponentDeclaration1 => component_declaration1(&mut f, tree),
        SyntaxKind::ShortClassDefinition => short_class_definition(&mut f, tree),
        SyntaxKind::EquationSection => equation_section(&mut f, tree),
        SyntaxKind::AlgorithmSection => algorithm_section(&mut f, tree),
        SyntaxKind::Equation => equation(&mut f, tree),
        SyntaxKind::Statement => statement(&mut f, tree),
        SyntaxKind::IfEquation => if_equation(&mut f, tree),
        SyntaxKind::IfStatement => if_statement(&mut f, tree),
        SyntaxKind::ForEquation => for_equation(&mut f, tree),
        SyntaxKind::ForStatement => for_statement(&mut f, tree),
        SyntaxKind::ForIndices => for_indices(&mut f, tree),
        SyntaxKind::ForIndex => for_index(&mut f, tree),
        SyntaxKind::WhileStatement => while_statement(&mut f, tree),
        SyntaxKind::WhenEquation => when_equation(&mut f, tree),
        SyntaxKind::WhenStatement => when_statement(&mut f, tree),
        SyntaxKind::ConnectEquation => connect_equation(&mut f, tree),
        SyntaxKind::Expression => expression(&mut f, tree),
        _ => (),
    }
    f.markers
}

struct Formatter {
    comments: Peekable<IntoIter<Token>>,
    markers: Vec<Marker>,
    prev_tok: ModelicaToken,
    prev_line: usize,
}

impl Formatter {
    fn new(comments: Vec<Token>) -> Self {
        Formatter {
            comments: comments.into_iter().peekable(),
            markers: Vec::new(),
            prev_tok: ModelicaToken::EOF,
            prev_line: 1,
        }
    }

    fn break_or_space(&mut self, is_multiline: bool, tok: &Token) {
        if is_multiline {
            self.handle_break(tok, false);
        } else {
            self.markers.push(Marker::Space);
        }
    }

    fn handle_break(&mut self, tok: &Token, allow_blanks: bool) {
        let section_opening =
            [ModelicaToken::Equation, ModelicaToken::Algorithm].contains(&self.prev_tok);
        let (inlines, comments) = self.comments_before(tok);
        for comment in inlines {
            self.markers.push(Marker::Space);
            self.markers.push(Marker::Token(comment.text));
        }
        if section_opening {
            self.markers.push(Marker::Blank);
        }
        let mut line = self.prev_line;
        for comment in comments {
            if !section_opening && line != self.prev_line {
                if comment.start.line - line > 1 {
                    self.markers.push(Marker::Blank);
                } else {
                    self.markers.push(Marker::Break);
                }
            }
            self.markers.push(Marker::Token(comment.text));
            line = comment.end.line;
        }
        if !(section_opening && line == self.prev_line) {
            if tok.start.line - line > 1 && allow_blanks {
                self.markers.push(Marker::Blank);
            } else {
                self.markers.push(Marker::Break);
            }
        }
    }

    fn comments_before(&mut self, tok: &Token) -> (Vec<Token>, Vec<Token>) {
        let mut comments = Vec::new();
        let mut inlines = Vec::new();
        while let Some(comment) = self.comments.peek() {
            if comment.idx < tok.idx {
                if comment.start.line == self.prev_line {
                    inlines.push(self.comments.next().unwrap());
                } else {
                    comments.push(self.comments.next().unwrap());
                }
            } else {
                break;
            }
        }
        (inlines, comments)
    }

    fn handle_token(&mut self, tok: Token) {
        self.prev_line = tok.end.line;
        self.prev_tok = tok.kind;
        self.markers.push(Marker::Token(tok.text));
    }
}

fn class_prefixes(f: &mut Formatter, tree: Tree) {}

fn short_class_specifier(f: &mut Formatter, tree: Tree) {}

fn type_prefix(f: &mut Formatter, tree: Tree) {}

fn declaration(f: &mut Formatter, tree: Tree) {}

fn constraining_clause(f: &mut Formatter, tree: Tree) {}

fn modification(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ClassModification => class_modification(f, tree),
                SyntaxKind::ModificationExpression => modification_expression(f, tree),
                _ => unreachable!(),
            }
            Child::Token(tok) => {
                f.markers.push(Marker::Space);
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn modification_expression(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => expression(f, tree),
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn class_modification(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    let is_multiline = tree.is_multiline();
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument_list(f, t, is_multiline),
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), false);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn argument_list(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument(f, t),
            Child::Token(tok) => {
                f.handle_token(tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, next_tree.start());
                }
            }
        }
    }
}

fn argument(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::ElementModificationOrReplaceable => {
                    element_modification_or_replaceable(f, tree)
                }
                SyntaxKind::ElementRedeclaration => element_redeclaration(f, tree),
                _ => unreachable!(),
            }
        }
    }
}

fn element_modification_or_replaceable(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ElementModification => element_modification(f, tree),
                SyntaxKind::ElementReplaceable => element_replaceable(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn element_modification(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::Name => name(f, tree),
                SyntaxKind::Modification => modification(f, tree),
                SyntaxKind::DescriptionString => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    description_string(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn element_redeclaration(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ShortClassDefinition => short_class_definition(f, tree),
                SyntaxKind::ComponentClause1 => component_clause1(f, tree),
                SyntaxKind::ConstrainingClause => element_replaceable(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn element_replaceable(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ShortClassDefinition => short_class_definition(f, tree),
                SyntaxKind::ComponentClause1 => component_clause1(f, tree),
                SyntaxKind::ConstrainingClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    constraining_clause(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn component_clause1(f: &mut Formatter, tree: Tree) {
    let children_count = tree.len();
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::TypePrefix => type_prefix(f, tree),
                SyntaxKind::TypeSpecifier => {
                    if children_count > 2 {
                        f.markers.push(Marker::Space);
                    }
                    type_specifier(f, tree);
                }
                SyntaxKind::ComponentDeclaration1 => {
                    f.markers.push(Marker::Space);
                    component_declaration1(f, tree);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn component_declaration1(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::Declaration => declaration(f, tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    description(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn short_class_definition(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::ClassPrefixes => class_prefixes(f, tree),
                SyntaxKind::ShortClassSpecifier => {
                    f.markers.push(Marker::Space);
                    short_class_specifier(f, tree);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn equation_section(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    for child in tree.children {
        match child {
            Child::Tree(tree) => {
                f.handle_break(tree.start(), true);
                equation(f, tree);
            }
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::Initial {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn algorithm_section(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    for child in tree.children {
        match child {
            Child::Tree(tree) => {
                f.handle_break(tree.start(), true);
                statement(f, tree);
            }
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::Initial {
                    f.markers.push(Marker::Space);
                } else if kind == ModelicaToken::Algorithm {
                    f.markers.push(Marker::Blank);
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn equation(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::SimpleExpression => simple_expression(f, tree),
                SyntaxKind::Expression => expression(f, tree),
                SyntaxKind::IfEquation => if_equation(f, tree),
                SyntaxKind::ForEquation => for_equation(f, tree),
                SyntaxKind::ConnectEquation => connect_equation(f, tree),
                SyntaxKind::WhenEquation => when_equation(f, tree),
                SyntaxKind::ComponentReference => component_reference(f, tree),
                SyntaxKind::FunctionCallArgs => function_call_args(f, tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    description(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                if kind == ModelicaToken::Equal {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(tok);
                if kind == ModelicaToken::Equal {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn statement(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ComponentReference => component_reference(f, tree),
                SyntaxKind::Expression => expression(f, tree),
                SyntaxKind::FunctionCallArgs => function_call_args(f, tree),
                SyntaxKind::OutputExpressionList => {
                    let is_multiline = f.prev_line < tree.start().start.line || tree.is_multiline();
                    if is_multiline {
                        f.markers.push(Marker::Indent);
                        f.handle_break(tree.start(), false);
                    }
                    output_expression_list(f, tree, is_multiline);
                    if is_multiline {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::IfStatement => if_statement(f, tree),
                SyntaxKind::ForStatement => for_statement(f, tree),
                SyntaxKind::WhileStatement => while_statement(f, tree),
                SyntaxKind::WhenStatement => when_statement(f, tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    description(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                if kind == ModelicaToken::Assign {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(tok);
                if kind == ModelicaToken::Assign {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn if_equation(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    equation(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::If && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if [
                    ModelicaToken::ElseIf,
                    ModelicaToken::Else,
                    ModelicaToken::End,
                ]
                .contains(&tok.kind)
                {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn if_statement(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::If && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if [
                    ModelicaToken::ElseIf,
                    ModelicaToken::Else,
                    ModelicaToken::End,
                ]
                .contains(&tok.kind)
                {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn for_equation(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ForIndices => {
                    f.markers.push(Marker::Space);
                    for_indices(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    equation(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::For && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn for_statement(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ForIndices => {
                    f.markers.push(Marker::Space);
                    for_indices(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::For && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn for_indices(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => for_index(f, tree),
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn for_index(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => expression(f, tree),
            Child::Token(tok) => {
                let kind = tok.kind;
                if kind == ModelicaToken::In {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(tok);
                if kind == ModelicaToken::In {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn while_statement(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::While && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn when_equation(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    equation(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::When && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::ElseWhen || tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn when_statement(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), false);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::When && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::ElseWhen || tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, false);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn connect_equation(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    f.markers.push(Marker::Indent);
    for (idx, child) in tree.children.into_iter().enumerate() {
        match child {
            Child::Tree(tree) => {
                if idx == 2 {
                    if is_multiline {
                        f.handle_break(tree.start(), false);
                    }
                } else {
                    f.break_or_space(is_multiline, tree.start());
                }
                component_reference(f, tree);
            }
            Child::Token(tok) => f.handle_token(tok),
        }
    }
    f.markers.push(Marker::Dedent);
}

fn expression(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let mut conditional = false;
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => {
                    if conditional {
                        f.break_or_space(is_multiline, tree.start());
                    } else {
                        f.markers.push(Marker::Space);
                    }
                    expression(f, tree);
                    if conditional {
                        f.markers.push(Marker::Dedent);
                        if let Some(next_child) = children.peek() {
                            if let Child::Token(next_tok) = next_child {
                                f.break_or_space(is_multiline, next_tok);
                            }
                        }
                    } else {
                        f.markers.push(Marker::Space);
                    }
                    conditional = false;
                }
                SyntaxKind::SimpleExpression => simple_expression(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::Then || kind == ModelicaToken::Else {
                    conditional = true;
                    f.markers.push(Marker::Indent);
                }
            }
        }
    }
}

fn simple_expression(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => logical_expression(f, tree),
            Child::Token(tok) => {
                f.break_or_space(is_multiline, &tok);
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn logical_expression(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => logical_term(f, tree),
            Child::Token(tok) => {
                f.break_or_space(is_multiline, &tok);
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn logical_term(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => logical_factor(f, tree),
            Child::Token(tok) => {
                f.break_or_space(is_multiline, &tok);
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn logical_factor(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => relation(f, tree),
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn relation(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        if let Child::Tree(tree) = child {
            if tree.kind == SyntaxKind::RelationalOperator {
                f.break_or_space(is_multiline, tree.start());
                relational_operator(f, tree);
                f.markers.push(Marker::Space);
            } else {
                arithmetic_expression(f, tree);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn relational_operator(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn arithmetic_expression(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for (idx, child) in tree.children.into_iter().enumerate() {
        if let Child::Tree(tree) = child {
            if tree.kind == SyntaxKind::AddOperator {
                if idx > 0 {
                    f.break_or_space(is_multiline, tree.start());
                }
                add_operator(f, tree);
                if idx > 0 {
                    f.markers.push(Marker::Space);
                }
            } else {
                term(f, tree);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn add_operator(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn term(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        if let Child::Tree(tree) = child {
            if tree.kind == SyntaxKind::MulOperator {
                f.break_or_space(is_multiline, tree.start());
                mul_operator(f, tree);
                f.markers.push(Marker::Space);
            } else {
                factor(f, tree);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn mul_operator(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn factor(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let length = tree.len();
    if is_multiline && length > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => primary(f, tree),
            Child::Token(tok) => {
                f.break_or_space(is_multiline, &tok);
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    if is_multiline && length > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn primary(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let children_count = tree.children.len();
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Token(tok) => match tok.kind {
                ModelicaToken::UInt
                | ModelicaToken::UReal
                | ModelicaToken::String
                | ModelicaToken::Bool
                | ModelicaToken::Der
                | ModelicaToken::Initial
                | ModelicaToken::Pure
                | ModelicaToken::End => f.handle_token(tok),
                ModelicaToken::Semicolon => {
                    f.handle_token(tok);
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.break_or_space(is_multiline, next_tree.start());
                    }
                }
                // Arrays etc.
                ModelicaToken::LCurly | ModelicaToken::LBracket | ModelicaToken::LParen => {
                    f.handle_token(tok);
                    f.markers.push(Marker::Indent);
                    if is_multiline {
                        if let Child::Tree(next_tree) = children.peek().unwrap() {
                            f.handle_break(next_tree.start(), false);
                        }
                    }
                }
                ModelicaToken::RCurly | ModelicaToken::RBracket | ModelicaToken::RParen => {
                    f.markers.push(Marker::Dedent);
                    f.handle_token(tok);
                }
                _ => unreachable!(),
            },
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ComponentReference => component_reference(f, tree),
                SyntaxKind::FunctionCallArgs => function_call_args(f, tree),
                SyntaxKind::ArraySubscripts => array_subscripts(f, tree),
                SyntaxKind::ArrayArguments => array_arguments(f, tree, is_multiline),
                SyntaxKind::ExpressionList => {
                    expression_list(f, tree, is_multiline && children_count == 3)
                }
                SyntaxKind::OutputExpressionList => output_expression_list(f, tree, is_multiline),
                _ => unreachable!(),
            },
        }
    }
}

fn type_specifier(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(t) => name(f, t),
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn name(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn component_reference(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(t) => array_subscripts(f, t),
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn result_reference(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(t) => component_reference(f, t),
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::Comma {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn function_call_args(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let mut children = tree.children.into_iter().peekable();
    f.markers.push(Marker::Indent);
    while let Some(child) = children.next() {
        match child {
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), false);
                    }
                }
            }
            Child::Tree(tree) => function_arguments(f, tree, is_multiline),
        }
    }
    f.markers.push(Marker::Dedent);
}

fn function_arguments(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => expression(f, tree),
                SyntaxKind::FunctionPartialApplication => function_partial_application(f, tree),
                SyntaxKind::ForIndices => for_indices(f, tree),
                SyntaxKind::FunctionArgumentsNonFirst => {
                    f.break_or_space(is_multiline, tree.start());
                    function_arguments_non_first(f, tree, is_multiline);
                }
                SyntaxKind::NamedArguments => named_arguments(f, tree, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::For {
                    f.break_or_space(is_multiline, &tok);
                    f.handle_token(tok);
                    f.markers.push(Marker::Space);
                } else {
                    f.handle_token(tok);
                }
            }
        }
    }
}

fn function_arguments_non_first(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::FunctionArgument => function_argument(f, tree),
                SyntaxKind::FunctionArgumentsNonFirst => {
                    f.break_or_space(is_multiline, tree.start());
                    function_arguments_non_first(f, tree, is_multiline);
                }
                SyntaxKind::NamedArguments => named_arguments(f, tree, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn array_arguments(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => expression(f, tree),
                SyntaxKind::ArrayArgumentsNonFirst => {
                    f.break_or_space(is_multiline, tree.start());
                    array_arguments_non_first(f, tree, is_multiline);
                }
                SyntaxKind::ForIndices => for_indices(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::For {
                    f.break_or_space(is_multiline, &tok);
                    f.handle_token(tok);
                    f.markers.push(Marker::Space);
                } else {
                    f.handle_token(tok);
                }
            }
        }
    }
}

fn array_arguments_non_first(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Expression => expression(f, tree),
                SyntaxKind::ArrayArgumentsNonFirst => {
                    f.break_or_space(is_multiline, tree.start());
                    array_arguments_non_first(f, tree, is_multiline);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn named_arguments(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::NamedArgument => named_argument(f, tree),
                SyntaxKind::NamedArguments => {
                    f.break_or_space(is_multiline, tree.start());
                    named_arguments(f, tree, is_multiline);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn named_argument(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            function_argument(f, tree);
        } else if let Child::Token(tok) = child {
            f.handle_token(tok);
            f.markers.push(Marker::Space);
        }
    }
}

fn function_argument(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::FunctionPartialApplication => function_partial_application(f, tree),
                SyntaxKind::Expression => expression(f, tree),
                _ => unreachable!(),
            }
        }
    }
}

fn function_partial_application(f: &mut Formatter, tree: Tree) {
    let mut is_multiline = false;
    let mut children = tree.children.iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if token.kind == ModelicaToken::LParen {
                while let Some(child) = children.next() {
                    if let Child::Token(tok) = child {
                        if tok.kind == ModelicaToken::RParen {
                            is_multiline = tok.start.line > token.start.line;
                        }
                    }
                }
            }
        }
    }
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => match t.kind {
                SyntaxKind::TypeSpecifier => {
                    f.markers.push(Marker::Space);
                    type_specifier(f, t);
                }
                SyntaxKind::NamedArguments => named_arguments(f, t, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), false);
                    }
                }
            }
        }
    }
}

fn output_expression_list(f: &mut Formatter, tree: Tree, is_multiline: bool) {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => expression(f, t),
            Child::Token(tok) => {
                if f.prev_tok == ModelicaToken::Comma {
                    f.break_or_space(is_multiline, &tok);
                }
                f.handle_token(tok);
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    f.break_or_space(is_multiline, next_tree.start());
                }
            }
        }
    }
}

fn expression_list(f: &mut Formatter, tree: Tree, mut is_multiline: bool) {
    // Expression list could be already wrapped in an outer production
    // at the brackets or parentheses
    if !is_multiline {
        is_multiline = tree.is_multiline();
    }
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => expression(f, t),
            Child::Token(tok) => {
                f.handle_token(tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, next_tree.start());
                }
            }
        }
    }
}

fn array_subscripts(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    let is_multiline = tree.is_multiline();
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => subscript(f, t),
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::LBracket && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), false);
                    }
                } else if kind == ModelicaToken::Comma {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.break_or_space(is_multiline, next_tree.start());
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn subscript(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(t) => expression(f, t),
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn description(f: &mut Formatter, tree: Tree) {
    for (idx, child) in tree.children.into_iter().enumerate() {
        if let Child::Tree(t) = child {
            match t.kind {
                SyntaxKind::DescriptionString => description_string(f, t),
                SyntaxKind::AnnotationClause => {
                    if idx > 0 {
                        f.handle_break(t.start(), false);
                    }
                    annotation_clause(f, t);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn description_string(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    f.markers.push(Marker::Indent);
    for child in tree.children {
        if let Child::Token(tok) = child {
            match tok.kind {
                ModelicaToken::Plus => {
                    f.break_or_space(is_multiline, &tok);
                    f.handle_token(tok);
                    f.markers.push(Marker::Space);
                }
                ModelicaToken::String => f.handle_token(tok),
                _ => unreachable!(),
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn annotation_clause(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
            Child::Tree(t) => {
                class_modification(f, t);
            }
        }
    }
}
