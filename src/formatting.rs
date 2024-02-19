use std::{iter::Peekable, vec::IntoIter};

use crate::{markers::Marker, tree::Child, tree::Tree};
use moparse::*;

enum Blank {
    Required,
    Legal,
    Illegal,
}

pub fn format(tree: Tree, comments: Vec<Token>) -> Vec<Marker> {
    let mut f = Formatter::new(comments);
    match tree.kind {
        SyntaxKind::StoredDefinition => stored_definition(&mut f, tree),
        SyntaxKind::ClassDefinition => class_definition(&mut f, tree),
        SyntaxKind::ClassPrefixes => class_prefixes(&mut f, tree),
        SyntaxKind::ClassSpecifier => class_specifier(&mut f, tree),
        SyntaxKind::LongClassSpecifier => long_class_specifier(&mut f, tree),
        SyntaxKind::ShortClassSpecifier => short_class_specifier(&mut f, tree),
        SyntaxKind::DerClassSpecifier => der_class_specifier(&mut f, tree),
        SyntaxKind::BasePrefix => base_prefix(&mut f, tree),
        SyntaxKind::EnumList => enum_list(&mut f, tree, false),
        SyntaxKind::EnumerationLiteral => enumeration_literal(&mut f, tree),
        SyntaxKind::Composition => composition(&mut f, tree),
        SyntaxKind::LanguageSpecification => language_specification(&mut f, tree),
        SyntaxKind::ExternalFunctionCall => external_function_call(&mut f, tree),
        SyntaxKind::ElementList => element_list(&mut f, tree),
        SyntaxKind::Element => element(&mut f, tree),
        SyntaxKind::ImportClause => import_clause(&mut f, tree),
        SyntaxKind::ImportList => import_list(&mut f, tree, false),
        SyntaxKind::ExtendsClause => extends_clause(&mut f, tree),
        SyntaxKind::ConstrainingClause => constraining_clause(&mut f, tree),
        SyntaxKind::ClassOrInheritanceModification => {
            class_or_inheritance_modification(&mut f, tree)
        }
        SyntaxKind::ArgumentOrInheritanceModificationList => {
            argument_or_inheritance_modification_list(&mut f, tree, false)
        }
        SyntaxKind::InheritanceModification => inheritance_modification(&mut f, tree),
        SyntaxKind::ComponentClause => component_clause(&mut f, tree),
        SyntaxKind::TypePrefix => type_prefix(&mut f, tree),
        SyntaxKind::ComponentList => component_list(&mut f, tree),
        SyntaxKind::ComponentDeclaration => component_declaration(&mut f, tree),
        SyntaxKind::ConditionAttribute => condition_attribute(&mut f, tree),
        SyntaxKind::Declaration => declaration(&mut f, tree),
        SyntaxKind::Modification => modification(&mut f, tree),
        SyntaxKind::ModificationExpression => modification_expression(&mut f, tree),
        SyntaxKind::ClassModification => class_modification(&mut f, tree),
        SyntaxKind::ArgumentList => argument_list(&mut f, tree, false),
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
        SyntaxKind::Expression => _ = expression(&mut f, tree, false, false),
        SyntaxKind::SimpleExpression => _ = simple_expression(&mut f, tree, false),
        SyntaxKind::LogicalExpression => _ = logical_expression(&mut f, tree, false),
        SyntaxKind::LogicalTerm => _ = logical_term(&mut f, tree, false),
        SyntaxKind::LogicalFactor => _ = logical_factor(&mut f, tree, false),
        SyntaxKind::Relation => _ = relation(&mut f, tree, false),
        SyntaxKind::RelationalOperator => relational_operator(&mut f, tree),
        SyntaxKind::ArithmeticExpression => _ = arithmetic_expression(&mut f, tree, false),
        SyntaxKind::AddOperator => add_operator(&mut f, tree),
        SyntaxKind::Term => _ = term(&mut f, tree, false),
        SyntaxKind::MulOperator => mul_operator(&mut f, tree),
        SyntaxKind::Factor => _ = factor(&mut f, tree, false),
        SyntaxKind::Primary => _ = primary(&mut f, tree, false),
        SyntaxKind::TypeSpecifier => type_specifier(&mut f, tree),
        SyntaxKind::Name => name(&mut f, tree),
        SyntaxKind::ComponentReference => component_reference(&mut f, tree),
        SyntaxKind::ResultReference => result_reference(&mut f, tree),
        SyntaxKind::FunctionCallArgs => function_call_args(&mut f, tree),
        SyntaxKind::FunctionArguments => function_arguments(&mut f, tree, false),
        SyntaxKind::FunctionArgumentsNonFirst => function_arguments_non_first(&mut f, tree, false),
        SyntaxKind::ArrayArguments => array_arguments(&mut f, tree, false),
        SyntaxKind::ArrayArgumentsNonFirst => array_arguments_non_first(&mut f, tree, false),
        SyntaxKind::NamedArguments => named_arguments(&mut f, tree, false),
        SyntaxKind::NamedArgument => named_argument(&mut f, tree),
        SyntaxKind::FunctionArgument => function_argument(&mut f, tree),
        SyntaxKind::FunctionPartialApplication => function_partial_application(&mut f, tree),
        SyntaxKind::OutputExpressionList => _ = output_expression_list(&mut f, tree, false),
        SyntaxKind::ExpressionList => expression_list(&mut f, tree, false),
        SyntaxKind::ArraySubscripts => array_subscripts(&mut f, tree),
        SyntaxKind::Subscript => subscript(&mut f, tree),
        SyntaxKind::Description => description(&mut f, tree),
        SyntaxKind::DescriptionString => description_string(&mut f, tree),
        SyntaxKind::AnnotationClause => annotation_clause(&mut f, tree),
        SyntaxKind::Error => (),
    }
    f.markers.push(Marker::Break);
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
            self.handle_break(tok, Blank::Illegal);
        } else {
            self.markers.push(Marker::Space);
        }
    }

    fn handle_break(&mut self, tok: &Token, blanks: Blank) {
        let (inlines, comments) = self.comments_before(tok);
        for comment in inlines {
            if !self.markers.is_empty() {
                self.markers.push(Marker::Space);
            }
            self.markers.push(Marker::Token(comment.text));
        }
        if let Blank::Required = blanks {
            self.markers.push(Marker::Blank);
        }
        let mut line = self.prev_line;
        for comment in comments {
            if let Blank::Required = blanks {
                if line > self.prev_line {
                    if comment.start.line - line > 1 {
                        self.markers.push(Marker::Blank);
                    } else {
                        self.markers.push(Marker::Break);
                    }
                }
            } else if comment.start.line - line > 1 {
                self.markers.push(Marker::Blank);
            } else {
                self.markers.push(Marker::Break);
            }
            self.markers.push(Marker::Token(comment.text));
            line = comment.end.line;
        }
        if let Blank::Legal = blanks {
            if tok.start.line - line > 1 {
                self.markers.push(Marker::Blank);
            } else {
                self.markers.push(Marker::Break);
            }
        } else if let Blank::Illegal = blanks {
            self.markers.push(Marker::Break);
        } else if line > self.prev_line {
            if tok.start.line - line > 1 {
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
        let _ = self.comments_before(&tok);
        self.prev_line = tok.end.line;
        self.prev_tok = tok.kind;
        self.markers.push(Marker::Token(tok.text));
    }
}

fn stored_definition(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Name => name(f, tree),
                SyntaxKind::ClassDefinition => {
                    if f.prev_tok == ModelicaToken::Semicolon {
                        f.handle_break(tree.start(), Blank::Legal);
                    }
                    class_definition(f, tree);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                if kind == ModelicaToken::Final {
                    f.handle_break(&tok, Blank::Legal);
                } else if kind == ModelicaToken::Within && tok.idx > 0 {
                    f.handle_break(&tok, Blank::Illegal);
                }
                f.handle_token(tok);
                if kind == ModelicaToken::Final || kind == ModelicaToken::Within {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn class_definition(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ClassSpecifier => {
                    f.markers.push(Marker::Space);
                    class_specifier(f, tree);
                }
                SyntaxKind::ClassPrefixes => class_prefixes(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn class_prefixes(f: &mut Formatter, tree: Tree) {
    for (idx, child) in tree.children.into_iter().enumerate() {
        if let Child::Token(tok) = child {
            if idx > 0 {
                f.markers.push(Marker::Space);
            }
            f.handle_token(tok);
        }
    }
}

fn class_specifier(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::LongClassSpecifier => long_class_specifier(f, tree),
                SyntaxKind::ShortClassSpecifier => short_class_specifier(f, tree),
                SyntaxKind::DerClassSpecifier => der_class_specifier(f, tree),
                _ => unreachable!(),
            }
        }
    }
}

fn long_class_specifier(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::DescriptionString => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    description_string(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                SyntaxKind::ClassModification => class_modification(f, tree),
                SyntaxKind::Composition => composition(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                if kind == ModelicaToken::End {
                    f.handle_break(&tok, Blank::Required);
                }
                f.handle_token(tok);
                if kind == ModelicaToken::End || kind == ModelicaToken::Extends {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn short_class_specifier(f: &mut Formatter, tree: Tree) {
    let mut is_multiline = false;
    let mut children = tree.children.iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if token.kind == ModelicaToken::LParen {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if tok.kind == ModelicaToken::RParen {
                            is_multiline = tok.start.line > token.start.line;
                        }
                    }
                }
            }
        }
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::BasePrefix => {
                    let is_empty = tree.len() == 0;
                    base_prefix(f, tree);
                    if !is_empty {
                        f.markers.push(Marker::Space);
                    }
                }
                SyntaxKind::TypeSpecifier => type_specifier(f, tree),
                SyntaxKind::ArraySubscripts => array_subscripts(f, tree),
                SyntaxKind::ClassModification => class_modification(f, tree),
                SyntaxKind::EnumList => {
                    if !is_multiline {
                        // Enum list could be unwrapped, yet if it
                        // contains any description it should be wrapped
                        is_multiline = tree.contains(SyntaxKind::Description);
                    }
                    if is_multiline {
                        f.markers.push(Marker::Indent);
                        f.handle_break(tree.start(), Blank::Illegal);
                    }
                    enum_list(f, tree, is_multiline);
                    if is_multiline {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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

fn der_class_specifier(f: &mut Formatter, tree: Tree) {
    let mut is_multiline = false;
    let mut children = tree.children.iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if token.kind == ModelicaToken::LParen {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if tok.kind == ModelicaToken::RParen {
                            is_multiline = tok.start.line > token.start.line;
                        }
                    }
                }
            }
        }
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::TypeSpecifier => {
                    if is_multiline {
                        f.handle_break(tree.start(), Blank::Illegal);
                    }
                    type_specifier(f, tree);
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    description(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                if kind == ModelicaToken::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == ModelicaToken::Identifier && f.prev_tok == ModelicaToken::Comma {
                    f.break_or_space(is_multiline, &tok)
                }
                f.handle_token(tok);
                if kind == ModelicaToken::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == ModelicaToken::LParen {
                    f.markers.push(Marker::Indent);
                } else if kind == ModelicaToken::RParen {
                    f.markers.push(Marker::Dedent);
                }
            }
        }
    }
}

fn base_prefix(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn enum_list(f: &mut Formatter, tree: Tree, mut is_multiline: bool) {
    if !is_multiline {
        is_multiline = tree.is_multiline();
    }
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => enumeration_literal(f, t),
            Child::Token(tok) => {
                f.handle_token(tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, next_tree.start());
                }
            }
        }
    }
}

fn enumeration_literal(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => {
                f.markers.push(Marker::Indent);
                f.handle_break(tree.start(), Blank::Illegal);
                description(f, tree);
                f.markers.push(Marker::Dedent);
            }
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn composition(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ElementList => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Required);
                    element_list(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                SyntaxKind::EquationSection => {
                    f.handle_break(tree.start(), Blank::Required);
                    equation_section(f, tree);
                }
                SyntaxKind::AlgorithmSection => {
                    f.handle_break(tree.start(), Blank::Required);
                    algorithm_section(f, tree);
                }
                SyntaxKind::LanguageSpecification => {
                    f.markers.push(Marker::Space);
                    language_specification(f, tree);
                }
                SyntaxKind::ExternalFunctionCall => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Required);
                    external_function_call(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                SyntaxKind::AnnotationClause => {
                    f.markers.push(Marker::Indent);
                    let extern_element_annotation = [
                        ModelicaToken::RParen,
                        ModelicaToken::External,
                        ModelicaToken::String,
                    ]
                    .contains(&f.prev_tok);
                    if extern_element_annotation {
                        f.markers.push(Marker::Indent);
                    }
                    f.handle_break(
                        tree.start(),
                        if !extern_element_annotation {
                            Blank::Required
                        } else {
                            Blank::Illegal
                        },
                    );
                    annotation_clause(f, tree);
                    f.markers.push(Marker::Dedent);
                    if extern_element_annotation {
                        f.markers.push(Marker::Dedent);
                    }
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = tok.kind;
                if [
                    ModelicaToken::Protected,
                    ModelicaToken::Public,
                    ModelicaToken::External,
                ]
                .contains(&kind)
                {
                    f.handle_break(&tok, Blank::Required);
                }
                f.handle_token(tok);
            }
        }
    }
}

fn language_specification(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn external_function_call(f: &mut Formatter, tree: Tree) {
    let mut is_multiline = false;
    f.markers.push(Marker::Indent);
    let mut children = tree.children.iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if token.kind == ModelicaToken::LParen {
                for child in children.by_ref() {
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
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ComponentReference => component_reference(f, tree),
                SyntaxKind::ExpressionList => expression_list(f, tree, is_multiline),
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
                } else if kind == ModelicaToken::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), Blank::Illegal);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn element_list(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => {
                if f.prev_tok == ModelicaToken::Semicolon {
                    f.handle_break(tree.start(), Blank::Legal);
                }
                element(f, tree);
            }
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn element(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ImportClause => import_clause(f, tree),
                SyntaxKind::ExtendsClause => extends_clause(f, tree),
                SyntaxKind::ClassDefinition => class_definition(f, tree),
                SyntaxKind::ComponentClause => component_clause(f, tree),
                SyntaxKind::ConstrainingClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    constraining_clause(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    description(f, tree);
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

fn import_clause(f: &mut Formatter, tree: Tree) {
    let mut is_multiline = false;
    let mut children = tree.children.iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if token.kind == ModelicaToken::LCurly {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if tok.kind == ModelicaToken::RCurly {
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
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Name => name(f, tree),
                SyntaxKind::ImportList => import_list(f, tree, is_multiline),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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
                if kind == ModelicaToken::Import || kind == ModelicaToken::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == ModelicaToken::LCurly && is_multiline {
                    f.markers.push(Marker::Indent);
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), Blank::Illegal);
                    }
                } else if kind == ModelicaToken::RCurly && is_multiline {
                    f.markers.push(Marker::Dedent);
                }
            }
        }
    }
}

fn import_list(f: &mut Formatter, tree: Tree, mut is_multiline: bool) {
    if !is_multiline {
        is_multiline = tree.is_multiline();
    }
    for (idx, child) in tree.children.into_iter().enumerate() {
        if let Child::Token(tok) = child {
            if tok.kind == ModelicaToken::Identifier && idx > 1 {
                f.break_or_space(is_multiline, &tok);
            }
            f.handle_token(tok);
        }
    }
}

fn extends_clause(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::TypeSpecifier => type_specifier(f, tree),
                SyntaxKind::ClassOrInheritanceModification => {
                    class_or_inheritance_modification(f, tree)
                }
                SyntaxKind::AnnotationClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    annotation_clause(f, tree);
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

fn constraining_clause(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::TypeSpecifier => type_specifier(f, tree),
                SyntaxKind::ClassModification => class_modification(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn class_or_inheritance_modification(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    let is_multiline = tree.is_multiline() && tree.len() > 2;
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument_or_inheritance_modification_list(f, t, is_multiline),
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), Blank::Illegal);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn argument_or_inheritance_modification_list(
    f: &mut Formatter,
    tree: Tree,
    mut is_multiline: bool,
) {
    if !is_multiline {
        is_multiline = tree.is_multiline();
    }
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::Argument => argument(f, tree),
                SyntaxKind::InheritanceModification => inheritance_modification(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, next_tree.start());
                }
            }
        }
    }
}

fn inheritance_modification(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => connect_equation(f, tree),
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn component_clause(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::TypePrefix => {
                    let is_empty = tree.len() == 0;
                    type_prefix(f, tree);
                    if !is_empty {
                        f.markers.push(Marker::Space);
                    }
                }
                SyntaxKind::TypeSpecifier => type_specifier(f, tree),
                SyntaxKind::ArraySubscripts => array_subscripts(f, tree),
                SyntaxKind::ComponentList => component_list(f, tree),
                _ => unreachable!(),
            }
        }
    }
}

fn type_prefix(f: &mut Formatter, tree: Tree) {
    for (idx, child) in tree.children.into_iter().enumerate() {
        if let Child::Token(tok) = child {
            if idx > 0 {
                f.markers.push(Marker::Space);
            }
            f.handle_token(tok);
        }
    }
}

fn component_list(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    let children_count = tree.len();
    if is_multiline && children_count > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in tree.children {
        match child {
            Child::Tree(tree) => {
                f.break_or_space(is_multiline && children_count > 1, tree.start());
                component_declaration(f, tree);
            }
            Child::Token(tok) => f.handle_token(tok),
        }
    }
    if is_multiline && children_count > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn component_declaration(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::Declaration => declaration(f, tree),
                SyntaxKind::ConditionAttribute => {
                    f.markers.push(Marker::Space);
                    condition_attribute(f, tree);
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    description(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn condition_attribute(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => _ = expression(f, tree, false, false),
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn declaration(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ArraySubscripts => array_subscripts(f, tree),
                SyntaxKind::Modification => modification(f, tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn modification(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ClassModification => class_modification(f, tree),
                SyntaxKind::ModificationExpression => {
                    let is_multiline_if =
                        tree.is_multiline() && tree.start().kind == ModelicaToken::If;
                    if is_multiline_if {
                        f.markers.push(Marker::Indent);
                    }
                    f.break_or_space(is_multiline_if, tree.start());
                    modification_expression(f, tree);
                    if is_multiline_if {
                        f.markers.push(Marker::Dedent);
                    }
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.markers.push(Marker::Space);
                f.handle_token(tok);
            }
        }
    }
}

fn modification_expression(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => _ = expression(f, tree, false, false),
            Child::Token(tok) => f.handle_token(tok),
        }
    }
}

fn class_modification(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    let is_multiline = tree.is_multiline()
        || tree.contains(SyntaxKind::DescriptionString)
        || tree.contains(SyntaxKind::Description);
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument_list(f, t, is_multiline),
            Child::Token(tok) => {
                let kind = tok.kind;
                f.handle_token(tok);
                if kind == ModelicaToken::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(next_tree.start(), Blank::Illegal);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn argument_list(f: &mut Formatter, tree: Tree, mut is_multiline: bool) {
    if !is_multiline {
        is_multiline = tree.is_multiline();
    }
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
                    f.handle_break(tree.start(), Blank::Illegal);
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

fn element_replaceable(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ShortClassDefinition => short_class_definition(f, tree),
                SyntaxKind::ComponentClause1 => component_clause1(f, tree),
                SyntaxKind::ConstrainingClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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
                    f.handle_break(tree.start(), Blank::Illegal);
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
                f.handle_break(
                    tree.start(),
                    if f.prev_tok == ModelicaToken::Equation {
                        Blank::Required
                    } else {
                        Blank::Legal
                    },
                );
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
                f.handle_break(
                    tree.start(),
                    if f.prev_tok == ModelicaToken::Algorithm {
                        Blank::Required
                    } else {
                        Blank::Legal
                    },
                );
                statement(f, tree);
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

fn equation(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::SimpleExpression => _ = simple_expression(f, tree, false),
                SyntaxKind::Expression => {
                    let is_multiline_if =
                        tree.is_multiline() && tree.start().kind == ModelicaToken::If;
                    if is_multiline_if {
                        f.markers.push(Marker::Indent);
                    }
                    f.break_or_space(is_multiline_if, tree.start());
                    expression(f, tree, false, false);
                    if is_multiline_if {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::IfEquation => if_equation(f, tree),
                SyntaxKind::ForEquation => for_equation(f, tree),
                SyntaxKind::ConnectEquation => connect_equation(f, tree),
                SyntaxKind::WhenEquation => when_equation(f, tree),
                SyntaxKind::ComponentReference => component_reference(f, tree),
                SyntaxKind::FunctionCallArgs => function_call_args(f, tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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
            }
        }
    }
}

fn statement(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(tree) => match tree.kind {
                SyntaxKind::ComponentReference => {
                    if f.prev_tok == ModelicaToken::Assign {
                        f.markers.push(Marker::Space);
                    }
                    component_reference(f, tree);
                }
                SyntaxKind::Expression => {
                    let is_multiline_if =
                        tree.is_multiline() && tree.start().kind == ModelicaToken::If;
                    if is_multiline_if {
                        f.markers.push(Marker::Indent);
                    }
                    f.break_or_space(is_multiline_if, tree.start());
                    expression(f, tree, false, false);
                    if is_multiline_if {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::FunctionCallArgs => function_call_args(f, tree),
                SyntaxKind::OutputExpressionList => _ = output_expression_list(f, tree, false),
                SyntaxKind::IfStatement => if_statement(f, tree),
                SyntaxKind::ForStatement => for_statement(f, tree),
                SyntaxKind::WhileStatement => while_statement(f, tree),
                SyntaxKind::WhenStatement => when_statement(f, tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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
                    expression(f, tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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
                    f.handle_break(&tok, Blank::Illegal);
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
                    expression(f, tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
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
                    f.handle_break(&tok, Blank::Illegal);
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
                    f.handle_break(tree.start(), Blank::Illegal);
                    equation(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::For && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, Blank::Illegal);
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
                    f.handle_break(tree.start(), Blank::Illegal);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::For && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, Blank::Illegal);
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
            Child::Tree(tree) => _ = expression(f, tree, false, false),
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
                    expression(f, tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::While && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, Blank::Illegal);
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
                    expression(f, tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    equation(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::When && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::ElseWhen || tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, Blank::Illegal);
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
                    expression(f, tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(tree.start(), Blank::Illegal);
                    statement(f, tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if tok.kind == ModelicaToken::When && f.prev_tok == ModelicaToken::End {
                    f.markers.push(Marker::Space);
                } else if tok.kind == ModelicaToken::ElseWhen || tok.kind == ModelicaToken::End {
                    f.handle_break(&tok, Blank::Illegal);
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
                        f.handle_break(tree.start(), Blank::Illegal);
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

fn expression(f: &mut Formatter, tree: Tree, mut wrapped: bool, in_oel: bool) -> bool {
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
                    expression(f, tree, false, false);
                    if conditional {
                        f.markers.push(Marker::Dedent);
                        if let Some(Child::Token(next_tok)) = children.peek() {
                            f.break_or_space(is_multiline, next_tok);
                        }
                    } else {
                        f.markers.push(Marker::Space);
                    }
                    conditional = false;
                }
                SyntaxKind::SimpleExpression => wrapped = simple_expression(f, tree, wrapped),
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
    if wrapped && !in_oel {
        f.markers.push(Marker::Dedent);
    }
    wrapped
}

fn simple_expression(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    for child in tree.children {
        match child {
            Child::Tree(tree) => wrapped = logical_expression(f, tree, wrapped),
            Child::Token(tok) => {
                f.markers.push(Marker::Space);
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn logical_expression(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => wrapped = logical_term(f, tree, wrapped),
            Child::Token(tok) => {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline = next_tree.start().start.line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, &tok);
                }
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn logical_term(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => wrapped = logical_factor(f, tree, wrapped),
            Child::Token(tok) => {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline = next_tree.start().start.line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, &tok);
                }
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn logical_factor(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    for child in tree.children {
        match child {
            Child::Tree(tree) => wrapped = relation(f, tree, wrapped),
            Child::Token(tok) => {
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn relation(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        if let Child::Tree(tree) = child {
            if tree.kind == SyntaxKind::RelationalOperator {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline = next_tree.start().start.line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, tree.start());
                }
                relational_operator(f, tree);
                f.markers.push(Marker::Space);
            } else {
                wrapped = arithmetic_expression(f, tree, wrapped);
            }
        }
    }
    wrapped
}

fn relational_operator(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn arithmetic_expression(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    let mut children = tree.children.into_iter().enumerate().peekable();
    while let Some((idx, child)) = children.next() {
        if let Child::Tree(tree) = child {
            if tree.kind == SyntaxKind::AddOperator {
                if idx > 0 {
                    if let Some((_, Child::Tree(next_tree))) = children.peek() {
                        let is_multiline = next_tree.start().start.line > f.prev_line;
                        if is_multiline && !wrapped {
                            f.markers.push(Marker::Indent);
                        }
                        if !wrapped {
                            wrapped = is_multiline;
                        }
                        f.break_or_space(is_multiline, tree.start());
                    }
                }
                add_operator(f, tree);
                if idx > 0 {
                    f.markers.push(Marker::Space);
                }
            } else {
                wrapped = term(f, tree, wrapped);
            }
        }
    }
    wrapped
}

fn add_operator(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn term(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        if let Child::Tree(tree) = child {
            if tree.kind == SyntaxKind::MulOperator {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline = next_tree.start().start.line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, tree.start());
                }
                mul_operator(f, tree);
                f.markers.push(Marker::Space);
            } else {
                wrapped = factor(f, tree, wrapped);
            }
        }
    }
    wrapped
}

fn mul_operator(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Token(tok) = child {
            f.handle_token(tok);
        }
    }
}

fn factor(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    let mut children = tree.children.into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => wrapped = primary(f, tree, wrapped),
            Child::Token(tok) => {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline = next_tree.start().start.line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, &tok);
                }
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn primary(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
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
                ModelicaToken::LCurly | ModelicaToken::LBracket => {
                    f.handle_token(tok);
                    f.markers.push(Marker::Indent);
                    if is_multiline {
                        if let Child::Tree(next_tree) = children.peek().unwrap() {
                            f.handle_break(next_tree.start(), Blank::Illegal);
                        }
                    }
                }
                ModelicaToken::RCurly | ModelicaToken::RBracket => {
                    f.markers.push(Marker::Dedent);
                    f.handle_token(tok);
                }
                ModelicaToken::LParen | ModelicaToken::RParen => f.handle_token(tok),
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
                SyntaxKind::OutputExpressionList => {
                    wrapped = output_expression_list(f, tree, wrapped)
                }
                _ => unreachable!(),
            },
        }
    }
    wrapped
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
                        f.handle_break(next_tree.start(), Blank::Illegal);
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
                SyntaxKind::Expression => _ = expression(f, tree, false, false),
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
                SyntaxKind::Expression => _ = expression(f, tree, false, false),
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
                SyntaxKind::Expression => _ = expression(f, tree, false, false),
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
            let is_multiline_if = tree.is_multiline() && tree.start().kind == ModelicaToken::If;
            if is_multiline_if {
                f.markers.push(Marker::Indent);
            }
            f.break_or_space(is_multiline_if, tree.start());
            function_argument(f, tree);
            if is_multiline_if {
                f.markers.push(Marker::Dedent);
            }
        } else if let Child::Token(tok) = child {
            if tok.kind == ModelicaToken::Equal {
                f.markers.push(Marker::Space);
            }
            f.handle_token(tok);
        }
    }
}

fn function_argument(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        if let Child::Tree(tree) = child {
            match tree.kind {
                SyntaxKind::FunctionPartialApplication => function_partial_application(f, tree),
                SyntaxKind::Expression => _ = expression(f, tree, false, false),
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
                for child in children.by_ref() {
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
                        f.handle_break(next_tree.start(), Blank::Illegal);
                    }
                }
            }
        }
    }
}

fn output_expression_list(f: &mut Formatter, tree: Tree, mut wrapped: bool) -> bool {
    for child in tree.children {
        match child {
            Child::Tree(t) => wrapped = expression(f, t, wrapped, true),
            Child::Token(tok) => {
                if f.prev_tok == ModelicaToken::LParen {
                    f.markers.push(Marker::Space);  
                }
                f.handle_token(tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
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
            Child::Tree(t) => _ = expression(f, t, false, false),
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
                        f.handle_break(next_tree.start(), Blank::Illegal);
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
            Child::Tree(t) => _ = expression(f, t, false, false),
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
                        f.handle_break(t.start(), Blank::Illegal);
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
