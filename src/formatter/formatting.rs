use std::{iter::Peekable, vec::IntoIter};

use crate::parser::*;

#[derive(PartialEq)]
pub enum Marker {
    Token(TokenID),
    Indent,
    Dedent,
    Space,
    Blank,
    Break,
}

/// Enum used for controling blank line insertion
enum Blank {
    Required,
    Legal,
    Illegal,
}

/// Return collection of markers that should be consumed to generate pretty printed string
pub fn format(cst: &ModelicaCST) -> Vec<Marker> {
    let mut f = Formatter::new(cst);
    match cst.kind(cst.root().unwrap()) {
        SyntaxKind::StoredDefinition => stored_definition(&mut f, cst.root().unwrap()),
        SyntaxKind::ClassDefinition => class_definition(&mut f, cst.root().unwrap()),
        SyntaxKind::ClassPrefixes => class_prefixes(&mut f, cst.root().unwrap()),
        SyntaxKind::ClassSpecifier => class_specifier(&mut f, cst.root().unwrap()),
        SyntaxKind::LongClassSpecifier => long_class_specifier(&mut f, cst.root().unwrap()),
        SyntaxKind::ShortClassSpecifier => short_class_specifier(&mut f, cst.root().unwrap()),
        SyntaxKind::DerClassSpecifier => der_class_specifier(&mut f, cst.root().unwrap()),
        SyntaxKind::BasePrefix => base_prefix(&mut f, cst.root().unwrap()),
        SyntaxKind::EnumList => enum_list(&mut f, cst.root().unwrap(), false),
        SyntaxKind::EnumerationLiteral => enumeration_literal(&mut f, cst.root().unwrap()),
        SyntaxKind::Composition => composition(&mut f, cst.root().unwrap()),
        SyntaxKind::LanguageSpecification => language_specification(&mut f, cst.root().unwrap()),
        SyntaxKind::ExternalFunctionCall => external_function_call(&mut f, cst.root().unwrap()),
        SyntaxKind::ElementList => element_list(&mut f, cst.root().unwrap()),
        SyntaxKind::Element => element(&mut f, cst.root().unwrap()),
        SyntaxKind::ImportClause => import_clause(&mut f, cst.root().unwrap()),
        SyntaxKind::ImportList => import_list(&mut f, cst.root().unwrap(), false),
        SyntaxKind::ExtendsClause => extends_clause(&mut f, cst.root().unwrap()),
        SyntaxKind::ConstrainingClause => constraining_clause(&mut f, cst.root().unwrap()),
        SyntaxKind::ClassOrInheritanceModification => {
            class_or_inheritance_modification(&mut f, cst.root().unwrap())
        }
        SyntaxKind::ArgumentOrInheritanceModificationList => {
            argument_or_inheritance_modification_list(&mut f, cst.root().unwrap(), false)
        }
        SyntaxKind::InheritanceModification => {
            inheritance_modification(&mut f, cst.root().unwrap())
        }
        SyntaxKind::ComponentClause => component_clause(&mut f, cst.root().unwrap()),
        SyntaxKind::TypePrefix => type_prefix(&mut f, cst.root().unwrap()),
        SyntaxKind::ComponentList => component_list(&mut f, cst.root().unwrap()),
        SyntaxKind::ComponentDeclaration => component_declaration(&mut f, cst.root().unwrap()),
        SyntaxKind::ConditionAttribute => condition_attribute(&mut f, cst.root().unwrap()),
        SyntaxKind::Declaration => declaration(&mut f, cst.root().unwrap()),
        SyntaxKind::Modification => modification(&mut f, cst.root().unwrap()),
        SyntaxKind::ModificationExpression => modification_expression(&mut f, cst.root().unwrap()),
        SyntaxKind::ClassModification => class_modification(&mut f, cst.root().unwrap()),
        SyntaxKind::ArgumentList => argument_list(&mut f, cst.root().unwrap(), false),
        SyntaxKind::Argument => argument(&mut f, cst.root().unwrap()),
        SyntaxKind::ElementModificationOrReplaceable => {
            element_modification_or_replaceable(&mut f, cst.root().unwrap())
        }
        SyntaxKind::ElementModification => element_modification(&mut f, cst.root().unwrap()),
        SyntaxKind::ElementRedeclaration => element_redeclaration(&mut f, cst.root().unwrap()),
        SyntaxKind::ElementReplaceable => element_replaceable(&mut f, cst.root().unwrap()),
        SyntaxKind::ComponentClause1 => component_clause1(&mut f, cst.root().unwrap()),
        SyntaxKind::ComponentDeclaration1 => component_declaration1(&mut f, cst.root().unwrap()),
        SyntaxKind::ShortClassDefinition => short_class_definition(&mut f, cst.root().unwrap()),
        SyntaxKind::EquationSection => equation_section(&mut f, cst.root().unwrap()),
        SyntaxKind::AlgorithmSection => algorithm_section(&mut f, cst.root().unwrap()),
        SyntaxKind::Equation => equation(&mut f, cst.root().unwrap()),
        SyntaxKind::Statement => statement(&mut f, cst.root().unwrap()),
        SyntaxKind::IfEquation => if_equation(&mut f, cst.root().unwrap()),
        SyntaxKind::IfStatement => if_statement(&mut f, cst.root().unwrap()),
        SyntaxKind::ForEquation => for_equation(&mut f, cst.root().unwrap()),
        SyntaxKind::ForStatement => for_statement(&mut f, cst.root().unwrap()),
        SyntaxKind::ForIndices => for_indices(&mut f, cst.root().unwrap()),
        SyntaxKind::ForIndex => for_index(&mut f, cst.root().unwrap()),
        SyntaxKind::WhileStatement => while_statement(&mut f, cst.root().unwrap()),
        SyntaxKind::WhenEquation => when_equation(&mut f, cst.root().unwrap()),
        SyntaxKind::WhenStatement => when_statement(&mut f, cst.root().unwrap()),
        SyntaxKind::ConnectEquation => connect_equation(&mut f, cst.root().unwrap()),
        SyntaxKind::Expression => _ = expression(&mut f, cst.root().unwrap(), false, false),
        SyntaxKind::SimpleExpression => _ = simple_expression(&mut f, cst.root().unwrap(), false),
        SyntaxKind::LogicalExpression => _ = logical_expression(&mut f, cst.root().unwrap(), false),
        SyntaxKind::LogicalTerm => _ = logical_term(&mut f, cst.root().unwrap(), false),
        SyntaxKind::LogicalFactor => _ = logical_factor(&mut f, cst.root().unwrap(), false),
        SyntaxKind::Relation => _ = relation(&mut f, cst.root().unwrap(), false),
        SyntaxKind::RelationalOperator => relational_operator(&mut f, cst.root().unwrap()),
        SyntaxKind::ArithmeticExpression => {
            _ = arithmetic_expression(&mut f, cst.root().unwrap(), false)
        }
        SyntaxKind::AddOperator => add_operator(&mut f, cst.root().unwrap()),
        SyntaxKind::Term => _ = term(&mut f, cst.root().unwrap(), false),
        SyntaxKind::MulOperator => mul_operator(&mut f, cst.root().unwrap()),
        SyntaxKind::Factor => _ = factor(&mut f, cst.root().unwrap(), false),
        SyntaxKind::Primary => _ = primary(&mut f, cst.root().unwrap(), false),
        SyntaxKind::TypeSpecifier => type_specifier(&mut f, cst.root().unwrap()),
        SyntaxKind::Name => name(&mut f, cst.root().unwrap()),
        SyntaxKind::ComponentReference => component_reference(&mut f, cst.root().unwrap()),
        SyntaxKind::ResultReference => result_reference(&mut f, cst.root().unwrap()),
        SyntaxKind::FunctionCallArgs => function_call_args(&mut f, cst.root().unwrap()),
        SyntaxKind::FunctionArguments => function_arguments(&mut f, cst.root().unwrap(), false),
        SyntaxKind::FunctionArgumentsNonFirst => {
            function_arguments_non_first(&mut f, cst.root().unwrap(), false)
        }
        SyntaxKind::ArrayArguments => array_arguments(&mut f, cst.root().unwrap(), false),
        SyntaxKind::ArrayArgumentsNonFirst => {
            array_arguments_non_first(&mut f, cst.root().unwrap(), false)
        }
        SyntaxKind::NamedArguments => named_arguments(&mut f, cst.root().unwrap(), false),
        SyntaxKind::NamedArgument => named_argument(&mut f, cst.root().unwrap()),
        SyntaxKind::FunctionArgument => function_argument(&mut f, cst.root().unwrap()),
        SyntaxKind::FunctionPartialApplication => {
            function_partial_application(&mut f, cst.root().unwrap())
        }
        SyntaxKind::OutputExpressionList => {
            _ = output_expression_list(&mut f, cst.root().unwrap(), false)
        }
        SyntaxKind::ExpressionList => expression_list(&mut f, cst.root().unwrap(), false),
        SyntaxKind::ArraySubscripts => array_subscripts(&mut f, cst.root().unwrap()),
        SyntaxKind::Subscript => subscript(&mut f, cst.root().unwrap()),
        SyntaxKind::Description => description(&mut f, cst.root().unwrap()),
        SyntaxKind::DescriptionString => description_string(&mut f, cst.root().unwrap()),
        SyntaxKind::AnnotationClause => annotation_clause(&mut f, cst.root().unwrap()),
        SyntaxKind::Error => (),
    }
    f.markers
}

/// Helper structure that collects markers
struct Formatter<'a> {
    cst: &'a ModelicaCST,
    markers: Vec<Marker>,
    comments: Peekable<IntoIter<TokenID>>,
    prev_kind: TokenKind,
    prev_line: usize,
    prev_tok: TokenID,
}

impl<'a> Formatter<'a> {
    fn new(cst: &'a ModelicaCST) -> Self {
        Formatter {
            cst,
            markers: Vec::new(),
            comments: cst.tokens().comments().into_iter().peekable(),
            prev_kind: TokenKind::EOF,
            prev_line: 1,
            prev_tok: cst.tokens().first(),
        }
    }

    /// Insert whitespace or linebreak marker
    fn break_or_space(&mut self, is_multiline: bool, tok: TokenID) {
        if is_multiline {
            self.handle_break(tok, Blank::Illegal);
        } else {
            self.markers.push(Marker::Space);
        }
    }

    /// Find and insert comments, and check if blank line may be inserted
    fn handle_break(&mut self, tok: TokenID, blanks: Blank) {
        let (inlines, comments) = self.comments_before(tok);
        for comment in inlines {
            if !self.markers.is_empty() {
                self.markers.push(Marker::Space);
            }
            self.markers.push(Marker::Token(comment));
        }
        if let Blank::Required = blanks {
            self.markers.push(Marker::Blank);
        }
        let mut line = self.prev_line;
        for comment in comments {
            if let Blank::Required = blanks {
                if line > self.prev_line {
                    if self.cst.tokens().start(comment).line - line > 1 {
                        self.markers.push(Marker::Blank);
                    } else {
                        self.markers.push(Marker::Break);
                    }
                }
            } else if self.cst.tokens().start(comment).line - line > 1 {
                self.markers.push(Marker::Blank);
            } else {
                self.markers.push(Marker::Break);
            }
            self.markers.push(Marker::Token(comment));
            line = self.cst.tokens().end(comment).line;
        }
        if let Blank::Legal = blanks {
            if self.cst.tokens().start(tok).line - line > 1 {
                self.markers.push(Marker::Blank);
            } else {
                self.markers.push(Marker::Break);
            }
        } else if let Blank::Illegal = blanks {
            self.markers.push(Marker::Break);
        } else if line > self.prev_line {
            if self.cst.tokens().start(tok).line - line > 1 {
                self.markers.push(Marker::Blank);
            } else {
                self.markers.push(Marker::Break);
            }
        }
    }

    /// Return comments from before the specified token.
    /// First vector contains inline comments.
    fn comments_before(&mut self, tok: TokenID) -> (Vec<TokenID>, Vec<TokenID>) {
        let mut comments = Vec::new();
        let mut inlines = Vec::new();
        while let Some(comment) = self.comments.peek() {
            if *comment < tok {
                if self.cst.tokens().start(*comment).line == self.prev_line {
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

    /// Collect token marker and update the last token data
    fn handle_token(&mut self, tok: TokenID) {
        // Discard comments, as they are only allowed when line is wrapped
        let _ = self.comments_before(tok);
        self.prev_line = self.cst.tokens().end(tok).line;
        self.prev_kind = self.cst.tokens().kind(tok);
        self.prev_tok = tok;
        self.markers.push(Marker::Token(tok));
    }
}

fn stored_definition(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Name => name(f, *tree),
                SyntaxKind::ClassDefinition => {
                    if f.prev_kind == TokenKind::Semicolon {
                        f.handle_break(f.cst.start(*tree), Blank::Legal);
                    }
                    class_definition(f, *tree);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Final {
                    f.handle_break(*tok, Blank::Legal);
                } else if kind == TokenKind::Within && tok > &f.cst.tokens().first() {
                    f.handle_break(*tok, Blank::Illegal);
                }
                f.handle_token(*tok);
                if kind == TokenKind::Final || kind == TokenKind::Within {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn class_definition(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ClassSpecifier => {
                    f.markers.push(Marker::Space);
                    class_specifier(f, *tree);
                }
                SyntaxKind::ClassPrefixes => class_prefixes(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn class_prefixes(f: &mut Formatter, tree: TreeID) {
    for (idx, child) in f.cst.children(tree).into_iter().enumerate() {
        if let Child::Token(tok) = child {
            if idx > 0 {
                f.markers.push(Marker::Space);
            }
            f.handle_token(*tok);
        }
    }
}

fn class_specifier(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::LongClassSpecifier => long_class_specifier(f, *tree),
                SyntaxKind::ShortClassSpecifier => short_class_specifier(f, *tree),
                SyntaxKind::DerClassSpecifier => der_class_specifier(f, *tree),
                _ => unreachable!(),
            }
        }
    }
}

fn long_class_specifier(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::DescriptionString => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description_string(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                SyntaxKind::ClassModification => class_modification(f, *tree),
                SyntaxKind::Composition => composition(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::End {
                    f.handle_break(*tok, Blank::Required);
                }
                f.handle_token(*tok);
                if kind == TokenKind::End || kind == TokenKind::Extends {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn short_class_specifier(f: &mut Formatter, tree: TreeID) {
    let mut is_multiline = false;
    let mut children = f.cst.children(tree).iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if f.cst.tokens().kind(*token) == TokenKind::LParen {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if f.cst.tokens().kind(*tok) == TokenKind::RParen {
                            is_multiline =
                                f.cst.tokens().start(*tok).line > f.cst.tokens().start(*token).line;
                        }
                    }
                }
            }
        }
    }
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::BasePrefix => {
                    let is_empty = f.cst.is_empty(*tree);
                    base_prefix(f, *tree);
                    if !is_empty {
                        f.markers.push(Marker::Space);
                    }
                }
                SyntaxKind::TypeSpecifier => type_specifier(f, *tree),
                SyntaxKind::ArraySubscripts => array_subscripts(f, *tree),
                SyntaxKind::ClassModification => class_modification(f, *tree),
                SyntaxKind::EnumList => {
                    if !is_multiline {
                        // Enum list could be unwrapped, yet if it
                        // contains any description it should be wrapped
                        is_multiline = f.cst.contains(*tree, SyntaxKind::Description);
                    }
                    if is_multiline {
                        f.markers.push(Marker::Indent);
                        f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    }
                    enum_list(f, *tree, is_multiline);
                    if is_multiline {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn der_class_specifier(f: &mut Formatter, tree: TreeID) {
    let mut is_multiline = false;
    let mut children = f.cst.children(tree).iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if f.cst.tokens().kind(*token) == TokenKind::LParen {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if f.cst.tokens().kind(*tok) == TokenKind::RParen {
                            is_multiline =
                                f.cst.tokens().start(*tok).line > f.cst.tokens().start(*token).line;
                        }
                    }
                }
            }
        }
    }
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::TypeSpecifier => {
                    if is_multiline {
                        f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    }
                    type_specifier(f, *tree);
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == TokenKind::Identifier && f.prev_kind == TokenKind::Comma {
                    f.break_or_space(is_multiline, *tok)
                }
                f.handle_token(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == TokenKind::LParen {
                    f.markers.push(Marker::Indent);
                } else if kind == TokenKind::RParen {
                    f.markers.push(Marker::Dedent);
                }
            }
        }
    }
}

fn base_prefix(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            f.handle_token(*tok);
        }
    }
}

fn enum_list(f: &mut Formatter, tree: TreeID, mut is_multiline: bool) {
    if !is_multiline {
        is_multiline = f.cst.is_multiline(tree);
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => enumeration_literal(f, *t),
            Child::Token(tok) => {
                f.handle_token(*tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, f.cst.start(*next_tree));
                }
            }
        }
    }
}

fn enumeration_literal(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => {
                f.markers.push(Marker::Indent);
                f.handle_break(f.cst.start(*tree), Blank::Illegal);
                description(f, *tree);
                f.markers.push(Marker::Dedent);
            }
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn composition(f: &mut Formatter, tree: TreeID) {
    let mut prev_rule = SyntaxKind::Error;
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => {
                let kind = f.cst.kind(*tree);
                match f.cst.kind(*tree) {
                    SyntaxKind::ElementList => {
                        f.markers.push(Marker::Indent);
                        f.handle_break(f.cst.start(*tree), Blank::Required);
                        element_list(f, *tree);
                        f.markers.push(Marker::Dedent);
                    }
                    SyntaxKind::EquationSection => {
                        f.handle_break(f.cst.start(*tree), Blank::Required);
                        equation_section(f, *tree);
                    }
                    SyntaxKind::AlgorithmSection => {
                        f.handle_break(f.cst.start(*tree), Blank::Required);
                        algorithm_section(f, *tree);
                    }
                    SyntaxKind::LanguageSpecification => {
                        f.markers.push(Marker::Space);
                        language_specification(f, *tree);
                    }
                    SyntaxKind::ExternalFunctionCall => {
                        f.markers.push(Marker::Indent);
                        f.handle_break(f.cst.start(*tree), Blank::Required);
                        external_function_call(f, *tree);
                        f.markers.push(Marker::Dedent);
                    }
                    SyntaxKind::AnnotationClause => {
                        f.markers.push(Marker::Indent);
                        let extern_element_annotation = f.prev_kind == TokenKind::External
                            || ([
                                SyntaxKind::LanguageSpecification,
                                SyntaxKind::ExternalFunctionCall,
                            ]
                            .contains(&prev_rule)
                                && f.prev_kind != TokenKind::Semicolon);
                        if extern_element_annotation {
                            f.markers.push(Marker::Indent);
                        }
                        f.handle_break(
                            f.cst.start(*tree),
                            if !extern_element_annotation {
                                Blank::Required
                            } else {
                                Blank::Illegal
                            },
                        );
                        annotation_clause(f, *tree);
                        f.markers.push(Marker::Dedent);
                        if extern_element_annotation {
                            f.markers.push(Marker::Dedent);
                        }
                    }
                    _ => unreachable!(),
                }
                prev_rule = kind;
            }
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if [TokenKind::Protected, TokenKind::Public, TokenKind::External].contains(&kind) {
                    f.handle_break(*tok, Blank::Required);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn language_specification(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            f.handle_token(*tok);
        }
    }
}

fn external_function_call(f: &mut Formatter, tree: TreeID) {
    let mut is_multiline = false;
    f.markers.push(Marker::Indent);
    let mut children = f.cst.children(tree).iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if f.cst.tokens().kind(*token) == TokenKind::LParen {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if f.cst.tokens().kind(*tok) == TokenKind::RParen {
                            is_multiline =
                                f.cst.tokens().start(*tok).line > f.cst.tokens().start(*token).line;
                        }
                    }
                }
            }
        }
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ComponentReference => component_reference(f, *tree),
                SyntaxKind::ExpressionList => expression_list(f, *tree, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == TokenKind::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn element_list(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => {
                if f.prev_kind == TokenKind::Semicolon {
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                }
                element(f, *tree);
            }
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn element(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ImportClause => import_clause(f, *tree),
                SyntaxKind::ExtendsClause => extends_clause(f, *tree),
                SyntaxKind::ClassDefinition => class_definition(f, *tree),
                SyntaxKind::ComponentClause => component_clause(f, *tree),
                SyntaxKind::ConstrainingClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    constraining_clause(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn import_clause(f: &mut Formatter, tree: TreeID) {
    let mut is_multiline = false;
    let mut children = f.cst.children(tree).iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if f.cst.tokens().kind(*token) == TokenKind::LCurly {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if f.cst.tokens().kind(*tok) == TokenKind::RCurly {
                            is_multiline =
                                f.cst.tokens().start(*tok).line > f.cst.tokens().start(*token).line;
                        }
                    }
                }
            }
        }
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Name => name(f, *tree),
                SyntaxKind::ImportList => import_list(f, *tree, is_multiline),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
                if kind == TokenKind::Import || kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                } else if kind == TokenKind::LCurly && is_multiline {
                    f.markers.push(Marker::Indent);
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                } else if kind == TokenKind::RCurly && is_multiline {
                    f.markers.push(Marker::Dedent);
                }
            }
        }
    }
}

fn import_list(f: &mut Formatter, tree: TreeID, mut is_multiline: bool) {
    if !is_multiline {
        is_multiline = f.cst.is_multiline(tree);
    }
    for (idx, child) in f.cst.children(tree).into_iter().enumerate() {
        if let Child::Token(tok) = child {
            if f.cst.tokens().kind(*tok) == TokenKind::Identifier && idx > 1 {
                f.break_or_space(is_multiline, *tok);
            }
            f.handle_token(*tok);
        }
    }
}

fn extends_clause(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::TypeSpecifier => type_specifier(f, *tree),
                SyntaxKind::ClassOrInheritanceModification => {
                    class_or_inheritance_modification(f, *tree)
                }
                SyntaxKind::AnnotationClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    annotation_clause(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn constraining_clause(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::TypeSpecifier => type_specifier(f, *tree),
                SyntaxKind::ClassModification => class_modification(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn class_or_inheritance_modification(f: &mut Formatter, tree: TreeID) {
    f.markers.push(Marker::Indent);
    let is_multiline = f.cst.is_multiline(tree) && f.cst.children(tree).len() > 2;
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument_or_inheritance_modification_list(f, *t, is_multiline),
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn argument_or_inheritance_modification_list(
    f: &mut Formatter,
    tree: TreeID,
    mut is_multiline: bool,
) {
    if !is_multiline {
        is_multiline = f.cst.is_multiline(tree);
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Argument => argument(f, *tree),
                SyntaxKind::InheritanceModification => inheritance_modification(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, f.cst.start(*next_tree));
                }
            }
        }
    }
}

fn inheritance_modification(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => connect_equation(f, *tree),
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::Break {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn component_clause(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::TypePrefix => {
                    let is_empty = f.cst.is_empty(*tree);
                    type_prefix(f, *tree);
                    if !is_empty {
                        f.markers.push(Marker::Space);
                    }
                }
                SyntaxKind::TypeSpecifier => type_specifier(f, *tree),
                SyntaxKind::ArraySubscripts => array_subscripts(f, *tree),
                SyntaxKind::ComponentList => component_list(f, *tree),
                _ => unreachable!(),
            }
        }
    }
}

fn type_prefix(f: &mut Formatter, tree: TreeID) {
    for (idx, child) in f.cst.children(tree).into_iter().enumerate() {
        if let Child::Token(tok) = child {
            if idx > 0 {
                f.markers.push(Marker::Space);
            }
            f.handle_token(*tok);
        }
    }
}

fn component_list(f: &mut Formatter, tree: TreeID) {
    let is_multiline = f.cst.is_multiline(tree);
    let children_count = f.cst.children(tree).len();
    if is_multiline && children_count > 1 {
        f.markers.push(Marker::Indent);
    }
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => {
                f.break_or_space(is_multiline && children_count > 1, f.cst.start(*tree));
                component_declaration(f, *tree);
            }
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
    if is_multiline && children_count > 1 {
        f.markers.push(Marker::Dedent);
    }
}

fn component_declaration(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::Declaration => declaration(f, *tree),
                SyntaxKind::ConditionAttribute => {
                    f.markers.push(Marker::Space);
                    condition_attribute(f, *tree);
                }
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn condition_attribute(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => _ = expression(f, *tree, false, false),
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn declaration(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ArraySubscripts => array_subscripts(f, *tree),
                SyntaxKind::Modification => modification(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn modification(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ClassModification => class_modification(f, *tree),
                SyntaxKind::ModificationExpression => {
                    let is_multiline_if = f.cst.is_multiline(*tree)
                        && f.cst.tokens().kind(f.cst.start(*tree)) == TokenKind::If;
                    if is_multiline_if {
                        f.markers.push(Marker::Indent);
                    }
                    f.break_or_space(is_multiline_if, f.cst.start(*tree));
                    modification_expression(f, *tree);
                    if is_multiline_if {
                        f.markers.push(Marker::Dedent);
                    }
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.markers.push(Marker::Space);
                f.handle_token(*tok);
            }
        }
    }
}

fn modification_expression(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => _ = expression(f, *tree, false, false),
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn class_modification(f: &mut Formatter, tree: TreeID) {
    f.markers.push(Marker::Indent);
    let is_multiline = f.cst.is_multiline(tree)
        || f.cst.contains(tree, SyntaxKind::DescriptionString)
        || f.cst.contains(tree, SyntaxKind::Description);
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument_list(f, *t, is_multiline),
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn argument_list(f: &mut Formatter, tree: TreeID, mut is_multiline: bool) {
    if !is_multiline {
        is_multiline = f.cst.is_multiline(tree);
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => argument(f, *t),
            Child::Token(tok) => {
                f.handle_token(*tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, f.cst.start(*next_tree));
                }
            }
        }
    }
}

fn argument(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::ElementModificationOrReplaceable => {
                    element_modification_or_replaceable(f, *tree)
                }
                SyntaxKind::ElementRedeclaration => element_redeclaration(f, *tree),
                _ => unreachable!(),
            }
        }
    }
}

fn element_modification_or_replaceable(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ElementModification => element_modification(f, *tree),
                SyntaxKind::ElementReplaceable => element_replaceable(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn element_modification(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::Name => name(f, *tree),
                SyntaxKind::Modification => modification(f, *tree),
                SyntaxKind::DescriptionString => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description_string(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn element_redeclaration(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ShortClassDefinition => short_class_definition(f, *tree),
                SyntaxKind::ComponentClause1 => component_clause1(f, *tree),
                SyntaxKind::ElementReplaceable => element_replaceable(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn element_replaceable(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ShortClassDefinition => short_class_definition(f, *tree),
                SyntaxKind::ComponentClause1 => component_clause1(f, *tree),
                SyntaxKind::ConstrainingClause => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    constraining_clause(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn component_clause1(f: &mut Formatter, tree: TreeID) {
    let children_count = f.cst.children(tree).len();
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::TypePrefix => type_prefix(f, *tree),
                SyntaxKind::TypeSpecifier => {
                    if children_count > 2 {
                        f.markers.push(Marker::Space);
                    }
                    type_specifier(f, *tree);
                }
                SyntaxKind::ComponentDeclaration1 => {
                    f.markers.push(Marker::Space);
                    component_declaration1(f, *tree);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn component_declaration1(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::Declaration => declaration(f, *tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn short_class_definition(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::ClassPrefixes => class_prefixes(f, *tree),
                SyntaxKind::ShortClassSpecifier => {
                    f.markers.push(Marker::Space);
                    short_class_specifier(f, *tree);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn equation_section(f: &mut Formatter, tree: TreeID) {
    f.markers.push(Marker::Indent);
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => {
                f.handle_break(
                    f.cst.start(*tree),
                    if f.prev_kind == TokenKind::Equation {
                        Blank::Required
                    } else {
                        Blank::Legal
                    },
                );
                equation(f, *tree);
            }
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::Initial {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn algorithm_section(f: &mut Formatter, tree: TreeID) {
    f.markers.push(Marker::Indent);
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => {
                f.handle_break(
                    f.cst.start(*tree),
                    if f.prev_kind == TokenKind::Algorithm {
                        Blank::Required
                    } else {
                        Blank::Legal
                    },
                );
                statement(f, *tree);
            }
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::Initial {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn equation(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::SimpleExpression => _ = simple_expression(f, *tree, false),
                SyntaxKind::Expression => {
                    let is_multiline_if = f.cst.is_multiline(*tree)
                        && f.cst.tokens().kind(f.cst.start(*tree)) == TokenKind::If;
                    if is_multiline_if {
                        f.markers.push(Marker::Indent);
                    }
                    f.break_or_space(is_multiline_if, f.cst.start(*tree));
                    expression(f, *tree, false, false);
                    if is_multiline_if {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::IfEquation => if_equation(f, *tree),
                SyntaxKind::ForEquation => for_equation(f, *tree),
                SyntaxKind::ConnectEquation => connect_equation(f, *tree),
                SyntaxKind::WhenEquation => when_equation(f, *tree),
                SyntaxKind::ComponentReference => component_reference(f, *tree),
                SyntaxKind::FunctionCallArgs => function_call_args(f, *tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Equal {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn statement(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ComponentReference => {
                    if f.prev_kind == TokenKind::Assign {
                        f.markers.push(Marker::Space);
                    }
                    component_reference(f, *tree);
                }
                SyntaxKind::Expression => {
                    let is_multiline_if = f.cst.is_multiline(*tree)
                        && f.cst.tokens().kind(f.cst.start(*tree)) == TokenKind::If;
                    if is_multiline_if {
                        f.markers.push(Marker::Indent);
                    }
                    f.break_or_space(is_multiline_if, f.cst.start(*tree));
                    expression(f, *tree, false, false);
                    if is_multiline_if {
                        f.markers.push(Marker::Dedent);
                    }
                }
                SyntaxKind::FunctionCallArgs => function_call_args(f, *tree),
                SyntaxKind::OutputExpressionList => _ = output_expression_list(f, *tree, false),
                SyntaxKind::IfStatement => if_statement(f, *tree),
                SyntaxKind::ForStatement => for_statement(f, *tree),
                SyntaxKind::WhileStatement => while_statement(f, *tree),
                SyntaxKind::WhenStatement => when_statement(f, *tree),
                SyntaxKind::Description => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    description(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::Assign {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn if_equation(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, *tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    equation(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::If && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if [TokenKind::ElseIf, TokenKind::Else, TokenKind::End]
                    .contains(&f.cst.tokens().kind(*tok))
                {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn if_statement(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, *tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    statement(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::If && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if [TokenKind::ElseIf, TokenKind::Else, TokenKind::End]
                    .contains(&f.cst.tokens().kind(*tok))
                {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn for_equation(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ForIndices => {
                    f.markers.push(Marker::Space);
                    for_indices(f, *tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    equation(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::For && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if f.cst.tokens().kind(*tok) == TokenKind::End {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn for_statement(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ForIndices => {
                    f.markers.push(Marker::Space);
                    for_indices(f, *tree);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    statement(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::For && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if f.cst.tokens().kind(*tok) == TokenKind::End {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn for_indices(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => for_index(f, *tree),
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
}

fn for_index(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => _ = expression(f, *tree, false, false),
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                if kind == TokenKind::In {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
                if kind == TokenKind::In {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn while_statement(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, *tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    statement(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::While && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if f.cst.tokens().kind(*tok) == TokenKind::End {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn when_equation(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, *tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Equation => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    equation(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::When && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if f.cst.tokens().kind(*tok) == TokenKind::ElseWhen
                    || f.cst.tokens().kind(*tok) == TokenKind::End
                {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn when_statement(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => {
                    f.markers.push(Marker::Space);
                    expression(f, *tree, false, false);
                    f.markers.push(Marker::Space);
                }
                SyntaxKind::Statement => {
                    f.markers.push(Marker::Indent);
                    f.handle_break(f.cst.start(*tree), Blank::Legal);
                    statement(f, *tree);
                    f.markers.push(Marker::Dedent);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::When && f.prev_kind == TokenKind::End {
                    f.markers.push(Marker::Space);
                } else if f.cst.tokens().kind(*tok) == TokenKind::ElseWhen
                    || f.cst.tokens().kind(*tok) == TokenKind::End
                {
                    f.handle_break(*tok, Blank::Legal);
                }
                f.handle_token(*tok);
            }
        }
    }
}

fn connect_equation(f: &mut Formatter, tree: TreeID) {
    let is_multiline = f.cst.is_multiline(tree);
    f.markers.push(Marker::Indent);
    for (idx, child) in f.cst.children(tree).into_iter().enumerate() {
        match child {
            Child::Tree(tree) => {
                if idx == 2 {
                    if is_multiline {
                        f.handle_break(f.cst.start(*tree), Blank::Illegal);
                    }
                } else {
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                }
                component_reference(f, *tree);
            }
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
    f.markers.push(Marker::Dedent);
}

fn expression(f: &mut Formatter, tree: TreeID, mut wrapped: bool, in_oel: bool) -> bool {
    let is_multiline = f.cst.is_multiline(tree);
    let mut conditional = false;
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => {
                    if conditional {
                        f.break_or_space(is_multiline, f.cst.start(*tree));
                    } else {
                        f.markers.push(Marker::Space);
                    }
                    expression(f, *tree, false, false);
                    if conditional {
                        f.markers.push(Marker::Dedent);
                        if let Some(Child::Token(next_tok)) = children.peek() {
                            f.break_or_space(is_multiline, *next_tok);
                        }
                    } else {
                        f.markers.push(Marker::Space);
                    }
                    conditional = false;
                }
                SyntaxKind::SimpleExpression => wrapped = simple_expression(f, *tree, wrapped),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::Then || kind == TokenKind::Else {
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

fn simple_expression(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => wrapped = logical_expression(f, *tree, wrapped),
            Child::Token(tok) => {
                f.markers.push(Marker::Space);
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn logical_expression(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => wrapped = logical_term(f, *tree, wrapped),
            Child::Token(tok) => {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline =
                        f.cst.tokens().start(f.cst.start(*next_tree)).line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, *tok);
                }
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn logical_term(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => wrapped = logical_factor(f, *tree, wrapped),
            Child::Token(tok) => {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline =
                        f.cst.tokens().start(f.cst.start(*next_tree)).line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, *tok);
                }
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn logical_factor(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => wrapped = relation(f, *tree, wrapped),
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn relation(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        if let Child::Tree(tree) = child {
            if f.cst.kind(*tree) == SyntaxKind::RelationalOperator {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline =
                        f.cst.tokens().start(f.cst.start(*next_tree)).line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                }
                relational_operator(f, *tree);
                f.markers.push(Marker::Space);
            } else {
                wrapped = arithmetic_expression(f, *tree, wrapped);
            }
        }
    }
    wrapped
}

fn relational_operator(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            f.handle_token(*tok);
        }
    }
}

fn arithmetic_expression(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let mut children = f.cst.children(tree).into_iter().enumerate().peekable();
    while let Some((idx, child)) = children.next() {
        if let Child::Tree(tree) = child {
            if f.cst.kind(*tree) == SyntaxKind::AddOperator {
                if idx > 0 {
                    if let Some((_, Child::Tree(next_tree))) = children.peek() {
                        let is_multiline =
                            f.cst.tokens().start(f.cst.start(*next_tree)).line > f.prev_line;
                        if is_multiline && !wrapped {
                            f.markers.push(Marker::Indent);
                        }
                        if !wrapped {
                            wrapped = is_multiline;
                        }
                        f.break_or_space(is_multiline, f.cst.start(*tree));
                    }
                }
                add_operator(f, *tree);
                if idx > 0 {
                    f.markers.push(Marker::Space);
                }
            } else {
                wrapped = term(f, *tree, wrapped);
            }
        }
    }
    wrapped
}

fn add_operator(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            f.handle_token(*tok);
        }
    }
}

fn term(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        if let Child::Tree(tree) = child {
            if f.cst.kind(*tree) == SyntaxKind::MulOperator {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline =
                        f.cst.tokens().start(f.cst.start(*next_tree)).line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                }
                mul_operator(f, *tree);
                f.markers.push(Marker::Space);
            } else {
                wrapped = factor(f, *tree, wrapped);
            }
        }
    }
    wrapped
}

fn mul_operator(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            f.handle_token(*tok);
        }
    }
}

fn factor(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(tree) => wrapped = primary(f, *tree, wrapped),
            Child::Token(tok) => {
                if let Some(Child::Tree(next_tree)) = children.peek() {
                    let is_multiline =
                        f.cst.tokens().start(f.cst.start(*next_tree)).line > f.prev_line;
                    if is_multiline && !wrapped {
                        f.markers.push(Marker::Indent);
                    }
                    if !wrapped {
                        wrapped = is_multiline;
                    }
                    f.break_or_space(is_multiline, *tok);
                }
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn primary(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    let is_multiline = f.cst.is_multiline(tree);
    let children_count = f.cst.children(tree).len();
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Token(tok) => match f.cst.tokens().kind(*tok) {
                TokenKind::UInt
                | TokenKind::UReal
                | TokenKind::String
                | TokenKind::Bool
                | TokenKind::Der
                | TokenKind::Initial
                | TokenKind::Pure
                | TokenKind::End => f.handle_token(*tok),
                TokenKind::Semicolon => {
                    f.handle_token(*tok);
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.break_or_space(is_multiline, f.cst.start(*next_tree));
                    }
                }
                // Arrays etc.
                TokenKind::LCurly | TokenKind::LBracket => {
                    f.handle_token(*tok);
                    f.markers.push(Marker::Indent);
                    if is_multiline {
                        if let Child::Tree(next_tree) = children.peek().unwrap() {
                            f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                        }
                    }
                }
                TokenKind::RCurly | TokenKind::RBracket => {
                    f.markers.push(Marker::Dedent);
                    f.handle_token(*tok);
                }
                TokenKind::LParen | TokenKind::RParen => f.handle_token(*tok),
                _ => unreachable!(),
            },
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::ComponentReference => component_reference(f, *tree),
                SyntaxKind::FunctionCallArgs => function_call_args(f, *tree),
                SyntaxKind::ArraySubscripts => array_subscripts(f, *tree),
                SyntaxKind::ArrayArguments => array_arguments(f, *tree, is_multiline),
                SyntaxKind::ExpressionList => {
                    expression_list(f, *tree, is_multiline && children_count == 3)
                }
                SyntaxKind::OutputExpressionList => {
                    wrapped = output_expression_list(f, *tree, wrapped)
                }
                _ => unreachable!(),
            },
        }
    }
    wrapped
}

fn type_specifier(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(t) => name(f, *t),
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn name(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            f.handle_token(*tok);
        }
    }
}

fn component_reference(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(t) => array_subscripts(f, *t),
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn result_reference(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(t) => component_reference(f, *t),
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::Comma {
                    f.markers.push(Marker::Space);
                }
            }
        }
    }
}

fn function_call_args(f: &mut Formatter, tree: TreeID) {
    let is_multiline = f.cst.is_multiline(tree);
    let mut children = f.cst.children(tree).into_iter().peekable();
    f.markers.push(Marker::Indent);
    while let Some(child) = children.next() {
        match child {
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                }
            }
            Child::Tree(tree) => function_arguments(f, *tree, is_multiline),
        }
    }
    f.markers.push(Marker::Dedent);
}

fn function_arguments(f: &mut Formatter, tree: TreeID, is_multiline: bool) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => _ = expression(f, *tree, false, false),
                SyntaxKind::FunctionPartialApplication => function_partial_application(f, *tree),
                SyntaxKind::ForIndices => for_indices(f, *tree),
                SyntaxKind::FunctionArgumentsNonFirst => {
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                    function_arguments_non_first(f, *tree, is_multiline);
                }
                SyntaxKind::NamedArguments => named_arguments(f, *tree, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::For {
                    f.break_or_space(is_multiline, *tok);
                    f.handle_token(*tok);
                    f.markers.push(Marker::Space);
                } else {
                    f.handle_token(*tok);
                }
            }
        }
    }
}

fn function_arguments_non_first(f: &mut Formatter, tree: TreeID, is_multiline: bool) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::FunctionArgument => function_argument(f, *tree),
                SyntaxKind::FunctionArgumentsNonFirst => {
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                    function_arguments_non_first(f, *tree, is_multiline);
                }
                SyntaxKind::NamedArguments => named_arguments(f, *tree, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn array_arguments(f: &mut Formatter, tree: TreeID, is_multiline: bool) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => _ = expression(f, *tree, false, false),
                SyntaxKind::ArrayArgumentsNonFirst => {
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                    array_arguments_non_first(f, *tree, is_multiline);
                }
                SyntaxKind::ForIndices => for_indices(f, *tree),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                if f.cst.tokens().kind(*tok) == TokenKind::For {
                    f.break_or_space(is_multiline, *tok);
                    f.handle_token(*tok);
                    f.markers.push(Marker::Space);
                } else {
                    f.handle_token(*tok);
                }
            }
        }
    }
}

fn array_arguments_non_first(f: &mut Formatter, tree: TreeID, is_multiline: bool) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::Expression => _ = expression(f, *tree, false, false),
                SyntaxKind::ArrayArgumentsNonFirst => {
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                    array_arguments_non_first(f, *tree, is_multiline);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn named_arguments(f: &mut Formatter, tree: TreeID, is_multiline: bool) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(tree) => match f.cst.kind(*tree) {
                SyntaxKind::NamedArgument => named_argument(f, *tree),
                SyntaxKind::NamedArguments => {
                    f.break_or_space(is_multiline, f.cst.start(*tree));
                    named_arguments(f, *tree, is_multiline);
                }
                _ => unreachable!(),
            },
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn named_argument(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            let is_multiline_if = f.cst.is_multiline(*tree)
                && f.cst.tokens().kind(f.cst.start(*tree)) == TokenKind::If;
            if is_multiline_if {
                f.markers.push(Marker::Indent);
            }
            f.break_or_space(is_multiline_if, f.cst.start(*tree));
            function_argument(f, *tree);
            if is_multiline_if {
                f.markers.push(Marker::Dedent);
            }
        } else if let Child::Token(tok) = child {
            if f.cst.tokens().kind(*tok) == TokenKind::Equal {
                f.markers.push(Marker::Space);
            }
            f.handle_token(*tok);
        }
    }
}

fn function_argument(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        if let Child::Tree(tree) = child {
            match f.cst.kind(*tree) {
                SyntaxKind::FunctionPartialApplication => function_partial_application(f, *tree),
                SyntaxKind::Expression => _ = expression(f, *tree, false, false),
                _ => unreachable!(),
            }
        }
    }
}

fn function_partial_application(f: &mut Formatter, tree: TreeID) {
    let mut is_multiline = false;
    let mut children = f.cst.children(tree).iter();
    while let Some(child) = children.next() {
        if let Child::Token(token) = child {
            if f.cst.tokens().kind(*token) == TokenKind::LParen {
                for child in children.by_ref() {
                    if let Child::Token(tok) = child {
                        if f.cst.tokens().kind(*tok) == TokenKind::RParen {
                            is_multiline =
                                f.cst.tokens().start(*tok).line > f.cst.tokens().start(*token).line;
                        }
                    }
                }
            }
        }
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => match f.cst.kind(*t) {
                SyntaxKind::TypeSpecifier => {
                    f.markers.push(Marker::Space);
                    type_specifier(f, *t);
                }
                SyntaxKind::NamedArguments => named_arguments(f, *t, is_multiline),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::LParen && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                }
            }
        }
    }
}

fn output_expression_list(f: &mut Formatter, tree: TreeID, mut wrapped: bool) -> bool {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(t) => wrapped = expression(f, *t, wrapped, true),
            Child::Token(tok) => {
                if f.prev_kind == TokenKind::LParen {
                    f.markers.push(Marker::Space);
                }
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
        }
    }
    wrapped
}

fn expression_list(f: &mut Formatter, tree: TreeID, mut is_multiline: bool) {
    // Expression list could be already wrapped in an outer production
    // at the brackets or parentheses
    if !is_multiline {
        is_multiline = f.cst.is_multiline(tree);
    }
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => _ = expression(f, *t, false, false),
            Child::Token(tok) => {
                f.handle_token(*tok);
                if let Child::Tree(next_tree) = children.peek().unwrap() {
                    f.break_or_space(is_multiline, f.cst.start(*next_tree));
                }
            }
        }
    }
}

fn array_subscripts(f: &mut Formatter, tree: TreeID) {
    f.markers.push(Marker::Indent);
    let is_multiline = f.cst.is_multiline(tree);
    let mut children = f.cst.children(tree).into_iter().peekable();
    while let Some(child) = children.next() {
        match child {
            Child::Tree(t) => subscript(f, *t),
            Child::Token(tok) => {
                let kind = f.cst.tokens().kind(*tok);
                f.handle_token(*tok);
                if kind == TokenKind::LBracket && is_multiline {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.handle_break(f.cst.start(*next_tree), Blank::Illegal);
                    }
                } else if kind == TokenKind::Comma {
                    if let Child::Tree(next_tree) = children.peek().unwrap() {
                        f.break_or_space(is_multiline, f.cst.start(*next_tree));
                    }
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn subscript(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Tree(t) => _ = expression(f, *t, false, false),
            Child::Token(tok) => f.handle_token(*tok),
        }
    }
}

fn description(f: &mut Formatter, tree: TreeID) {
    for (idx, child) in f.cst.children(tree).into_iter().enumerate() {
        if let Child::Tree(t) = child {
            match f.cst.kind(*t) {
                SyntaxKind::DescriptionString => description_string(f, *t),
                SyntaxKind::AnnotationClause => {
                    if idx > 0 {
                        f.handle_break(f.cst.start(*t), Blank::Illegal);
                    }
                    annotation_clause(f, *t);
                }
                _ => unreachable!(),
            }
        }
    }
}

fn description_string(f: &mut Formatter, tree: TreeID) {
    let is_multiline = f.cst.is_multiline(tree);
    f.markers.push(Marker::Indent);
    for child in f.cst.children(tree) {
        if let Child::Token(tok) = child {
            match f.cst.tokens().kind(*tok) {
                TokenKind::Plus => {
                    f.break_or_space(is_multiline, *tok);
                    f.handle_token(*tok);
                    f.markers.push(Marker::Space);
                }
                TokenKind::String => f.handle_token(*tok),
                _ => unreachable!(),
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn annotation_clause(f: &mut Formatter, tree: TreeID) {
    for child in f.cst.children(tree) {
        match child {
            Child::Token(tok) => {
                f.handle_token(*tok);
                f.markers.push(Marker::Space);
            }
            Child::Tree(t) => {
                class_modification(f, *t);
            }
        }
    }
}
