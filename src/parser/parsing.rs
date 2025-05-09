use std::cell::Cell;

use super::tokens::{TokenID, TokenKind, Tokenized};

pub fn events(tokens: &Tokenized, start: SyntaxKind) -> Vec<SyntaxEvent> {
    let mut parser = Parser::new(tokens);
    parser.parse(start);
    parser.events
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents Modelica grammar rules as defined in [Modelica
/// Specification
/// 3.7](https://specification.modelica.org/maint/3.7/modelica-concrete-syntax.html).
pub enum SyntaxKind {
    /// Custom production type used to indicate a faulty syntax tree
    Error,
    StoredDefinition,
    ClassDefinition,
    ClassPrefixes,
    ClassSpecifier,
    LongClassSpecifier,
    ShortClassSpecifier,
    DerClassSpecifier,
    BasePrefix,
    EnumList,
    EnumerationLiteral,
    Composition,
    LanguageSpecification,
    ExternalFunctionCall,
    ElementList,
    Element,
    ImportClause,
    ImportList,
    ExtendsClause,
    ConstrainingClause,
    ClassOrInheritanceModification,
    ArgumentOrInheritanceModificationList,
    InheritanceModification,
    ComponentClause,
    TypePrefix,
    ComponentList,
    ComponentDeclaration,
    ConditionAttribute,
    Declaration,
    Modification,
    ModificationExpression,
    ClassModification,
    ArgumentList,
    Argument,
    ElementModificationOrReplaceable,
    ElementModification,
    ElementRedeclaration,
    ElementReplaceable,
    ComponentClause1,
    ComponentDeclaration1,
    ShortClassDefinition,
    EquationSection,
    AlgorithmSection,
    Equation,
    Statement,
    IfEquation,
    IfStatement,
    ForEquation,
    ForStatement,
    ForIndices,
    ForIndex,
    WhileStatement,
    WhenEquation,
    WhenStatement,
    ConnectEquation,
    Expression,
    SimpleExpression,
    LogicalExpression,
    LogicalTerm,
    LogicalFactor,
    Relation,
    RelationalOperator,
    ArithmeticExpression,
    AddOperator,
    Term,
    MulOperator,
    Factor,
    Primary,
    TypeSpecifier,
    Name,
    ComponentReference,
    ResultReference,
    FunctionCallArgs,
    FunctionArguments,
    FunctionArgumentsNonFirst,
    ArrayArguments,
    ArrayArgumentsNonFirst,
    NamedArguments,
    NamedArgument,
    FunctionArgument,
    FunctionPartialApplication,
    OutputExpressionList,
    ExpressionList,
    ArraySubscripts,
    Subscript,
    Description,
    DescriptionString,
    AnnotationClause,
}

#[derive(Debug)]
/// Represents a single Modelica syntax event.
///
/// Syntax event may mark starts and ends of productions or terminals.
/// The list of such syntax events should be consumed to build a parse
/// tree or an AST.
pub enum SyntaxEvent {
    /// Event indicating beginning of the Modelica production.
    Enter(SyntaxKind),
    /// Event indicating an end of some Modelica production.
    Exit,
    /// Event indicating a token.
    Advance(TokenID),
    Error(TokenID, String),
}

/// Represents a Modelica parser
struct Parser<'a> {
    /// Scanned tokens
    tokens: &'a Tokenized,
    indices: Vec<TokenID>,
    /// Collected syntax events
    events: Vec<SyntaxEvent>,
    /// Current position in the `indices`
    pos: usize,
    /// Parser lifes
    lifes: Cell<u32>,
}

impl<'a> Parser<'a> {
    fn parse(&mut self, start: SyntaxKind) {
        match start {
            SyntaxKind::StoredDefinition => stored_definition(self),
            SyntaxKind::ClassDefinition => class_definition(self),
            SyntaxKind::ClassPrefixes => class_prefixes(self),
            SyntaxKind::ClassSpecifier => class_specifier(self),
            SyntaxKind::LongClassSpecifier => long_class_specifier(self),
            SyntaxKind::ShortClassSpecifier => short_class_specifier(self),
            SyntaxKind::DerClassSpecifier => der_class_specifier(self),
            SyntaxKind::BasePrefix => base_prefix(self),
            SyntaxKind::EnumList => enum_list(self),
            SyntaxKind::EnumerationLiteral => enumeration_literal(self),
            SyntaxKind::Composition => composition(self),
            SyntaxKind::LanguageSpecification => language_specification(self),
            SyntaxKind::ExternalFunctionCall => external_function_call(self),
            SyntaxKind::ElementList => element_list(self),
            SyntaxKind::Element => element(self),
            SyntaxKind::ImportClause => import_clause(self),
            SyntaxKind::ImportList => import_list(self),
            SyntaxKind::ExtendsClause => extends_clause(self),
            SyntaxKind::ConstrainingClause => constraining_clause(self),
            SyntaxKind::ClassOrInheritanceModification => class_or_inheritance_modification(self),
            SyntaxKind::ArgumentOrInheritanceModificationList => {
                argument_or_inheritance_modification_list(self)
            }
            SyntaxKind::InheritanceModification => inheritance_modification(self),
            SyntaxKind::ComponentClause => component_clause(self),
            SyntaxKind::TypePrefix => type_prefix(self),
            SyntaxKind::ComponentList => component_list(self),
            SyntaxKind::ComponentDeclaration => component_declaration(self),
            SyntaxKind::ConditionAttribute => condition_attribute(self),
            SyntaxKind::Declaration => declaration(self),
            SyntaxKind::Modification => modification(self),
            SyntaxKind::ModificationExpression => modification_expression(self),
            SyntaxKind::ClassModification => class_modification(self),
            SyntaxKind::ArgumentList => argument_list(self),
            SyntaxKind::Argument => argument(self),
            SyntaxKind::ElementModificationOrReplaceable => {
                element_modification_or_replaceable(self)
            }
            SyntaxKind::ElementModification => element_modification(self),
            SyntaxKind::ElementRedeclaration => element_redeclaration(self),
            SyntaxKind::ElementReplaceable => element_replaceable(self),
            SyntaxKind::ComponentClause1 => component_clause1(self),
            SyntaxKind::ComponentDeclaration1 => component_declaration1(self),
            SyntaxKind::ShortClassDefinition => short_class_definition(self),
            SyntaxKind::EquationSection => equation_section(self),
            SyntaxKind::AlgorithmSection => algorithm_section(self),
            SyntaxKind::Equation => equation(self),
            SyntaxKind::Statement => statement(self),
            SyntaxKind::IfEquation => if_equation(self),
            SyntaxKind::IfStatement => if_statement(self),
            SyntaxKind::ForEquation => for_equation(self),
            SyntaxKind::ForStatement => for_statement(self),
            SyntaxKind::ForIndices => for_indices(self),
            SyntaxKind::ForIndex => for_index(self),
            SyntaxKind::WhileStatement => while_statement(self),
            SyntaxKind::WhenEquation => when_equation(self),
            SyntaxKind::WhenStatement => when_statement(self),
            SyntaxKind::ConnectEquation => connect_equation(self),
            SyntaxKind::Expression => expression(self),
            SyntaxKind::SimpleExpression => simple_expression(self),
            SyntaxKind::LogicalExpression => logical_expression(self),
            SyntaxKind::LogicalTerm => logical_term(self),
            SyntaxKind::LogicalFactor => logical_factor(self),
            SyntaxKind::Relation => relation(self),
            SyntaxKind::RelationalOperator => relational_operator(self),
            SyntaxKind::ArithmeticExpression => arithmetic_expression(self),
            SyntaxKind::AddOperator => add_operator(self),
            SyntaxKind::Term => term(self),
            SyntaxKind::MulOperator => mul_operator(self),
            SyntaxKind::Factor => factor(self),
            SyntaxKind::Primary => primary(self),
            SyntaxKind::TypeSpecifier => type_specifier(self),
            SyntaxKind::Name => name(self),
            SyntaxKind::ComponentReference => component_reference(self),
            SyntaxKind::ResultReference => result_reference(self),
            SyntaxKind::FunctionCallArgs => function_call_args(self),
            SyntaxKind::FunctionArguments => function_arguments(self),
            SyntaxKind::FunctionArgumentsNonFirst => function_arguments_non_first(self),
            SyntaxKind::ArrayArguments => array_arguments(self),
            SyntaxKind::ArrayArgumentsNonFirst => array_arguments_non_first(self),
            SyntaxKind::NamedArguments => named_arguments(self),
            SyntaxKind::NamedArgument => named_argument(self),
            SyntaxKind::FunctionArgument => function_argument(self),
            SyntaxKind::FunctionPartialApplication => function_partial_application(self),
            SyntaxKind::OutputExpressionList => output_expression_list(self),
            SyntaxKind::ExpressionList => expression_list(self),
            SyntaxKind::ArraySubscripts => array_subscripts(self),
            SyntaxKind::Subscript => subscript(self),
            SyntaxKind::Description => description(self),
            SyntaxKind::DescriptionString => description_string(self),
            SyntaxKind::AnnotationClause => annotation_clause(self),
            SyntaxKind::Error => panic!("cannot parse error node"),
        }
    }

    /// Return a new parser instance
    fn new(tokens: &'a Tokenized) -> Self {
        let indices = tokens.tokens();
        Parser {
            tokens,
            indices,
            events: Vec::new(),
            pos: 0,
            lifes: Cell::new(100),
        }
    }

    /// Return the vector position where the grammar rule starts. Push a
    /// new `SyntaxEvent` into the vector.
    ///
    /// Assume that rule is erroneous. It is updated during `exit()`.
    fn enter(&mut self) -> usize {
        let mark = self.events.len();
        self.events.push(SyntaxEvent::Enter(SyntaxKind::Error));
        mark
    }

    /// Close the production scope and update its type
    ///
    /// * `m`: position of the corresponding enter event
    /// * `typ`: type of the production
    fn exit(&mut self, m: usize, typ: SyntaxKind) {
        self.events[m] = SyntaxEvent::Enter(typ);
        self.events.push(SyntaxEvent::Exit);
    }

    /// Advance the parser, consume the token and push it into the events vector
    fn advance(&mut self) {
        assert!(!self.eof());
        self.events
            .push(SyntaxEvent::Advance(*self.indices.get(self.pos).unwrap()));
        self.pos += 1;
        self.lifes.set(100);
    }

    /// Return `true` if parser reached the end of file
    fn eof(&self) -> bool {
        self.pos == self.indices.len()
    }
    /// Return type of the n-th token counting from the current one.
    fn nth(&self, n: usize) -> TokenKind {
        if self.lifes.get() == 0 {
            self.blowup();
        }
        self.lifes.set(self.lifes.get() - 1);
        self.indices
            .get(self.pos + n)
            .map_or(TokenKind::Eof, |i| self.tokens.kind(*i))
    }

    // FIXME Get rid of panic
    fn blowup(&self) {
        let id = if let Some(i) = self.indices.get(self.pos) {
            *i
        } else {
            self.tokens.last()
        };
        let tok = self.tokens.get(id);
        panic!(
            "{}:{}:{}: Parser stuck",
            tok.source, tok.start.line, tok.start.col
        );
    }

    /// Return `true` if current token matches the specified type
    fn check(&self, typ: TokenKind) -> bool {
        self.nth(0) == typ
    }

    /// Return `true` if current token matches any of the specified types
    fn check_any(&self, typ: &[TokenKind]) -> bool {
        typ.contains(&self.nth(0))
    }

    /// Return `true` if current token matches the specified type and
    /// advance the parser. Otherwise return `false` and do not advance.
    fn consume(&mut self, typ: TokenKind) -> bool {
        if self.check(typ) {
            self.advance();
            return true;
        }
        false
    }

    /// Mark currently parsed token as erroneus.
    fn error(&mut self, msg: String) {
        self.events.push(SyntaxEvent::Error(
            *self
                .indices
                .get(self.pos)
                .unwrap_or_else(|| self.indices.last().unwrap()),
            msg,
        ));
    }

    fn advance_with_error(&mut self, msg: String) {
        let mark = self.enter();
        self.error(msg);
        self.advance();
        self.exit(mark, SyntaxKind::Error);
    }

    /// Advance the parser if current token is expected. Report error if
    /// current tokens doesn't match the specified kind.
    fn expect(&mut self, kind: TokenKind) {
        if !self.consume(kind) {
            self.error(format!("expected {kind:?}, found {:?}", self.nth(0)));
        }
    }
}

// Useful constants used in the parsing process

const SECTION_BREAKERS: [TokenKind; 8] = [
    TokenKind::Protected,
    TokenKind::Public,
    TokenKind::Initial,
    TokenKind::Equation,
    TokenKind::Algorithm,
    TokenKind::End,
    TokenKind::Annotation,
    TokenKind::External,
];
const CLASS_PREFS: [TokenKind; 13] = [
    TokenKind::Partial,
    TokenKind::Class,
    TokenKind::Model,
    TokenKind::Record,
    TokenKind::Function,
    TokenKind::Block,
    TokenKind::Type,
    TokenKind::Operator,
    TokenKind::Connector,
    TokenKind::Package,
    TokenKind::Pure,
    TokenKind::Impure,
    TokenKind::Expandable,
];

// A.2.1 Stored Definition – Within

fn stored_definition(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(TokenKind::Within) {
        if !p.check(TokenKind::Semicolon) {
            name(p);
        }
        p.expect(TokenKind::Semicolon);
    }
    while !p.eof() {
        p.consume(TokenKind::Final);
        class_definition(p);
        p.expect(TokenKind::Semicolon);
    }
    p.exit(mark, SyntaxKind::StoredDefinition);
}

// A.2.2 Class Definition

fn class_definition(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Encapsulated);
    class_prefixes(p);
    class_specifier(p);
    p.exit(mark, SyntaxKind::ClassDefinition);
}

fn class_prefixes(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Partial);
    let pref = p.nth(0);
    match pref {
        TokenKind::Class
        | TokenKind::Model
        | TokenKind::Block
        | TokenKind::Type
        | TokenKind::Package
        | TokenKind::Record
        | TokenKind::Connector
        | TokenKind::Function => {
            p.advance();
        }
        TokenKind::Expandable => {
            p.advance();
            p.expect(TokenKind::Connector);
        }
        TokenKind::Operator => {
            p.advance();
            if !p.consume(TokenKind::Record) {
                p.consume(TokenKind::Function);
            }
        }
        TokenKind::Pure | TokenKind::Impure => {
            p.advance();
            p.consume(TokenKind::Operator);
            p.expect(TokenKind::Function);
        }
        _ => p.advance_with_error(format!(
            "unexpected token '{:?}' used as a class prefix",
            p.nth(0)
        )),
    }
    p.exit(mark, SyntaxKind::ClassPrefixes);
}

fn class_specifier(p: &mut Parser) {
    let mark = p.enter();
    if p.check(TokenKind::Extends) {
        long_class_specifier(p);
    } else if p.check(TokenKind::Identifier) {
        if p.nth(1) != TokenKind::Equal {
            long_class_specifier(p);
        } else if p.nth(2) == TokenKind::Der {
            der_class_specifier(p);
        } else {
            short_class_specifier(p);
        }
    } else {
        p.advance_with_error(format!(
            "unexpected token '{:?}': doesn't match any type of class specifier",
            p.nth(0)
        ));
    }
    p.exit(mark, SyntaxKind::ClassSpecifier);
}

fn long_class_specifier(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(TokenKind::Extends) {
        p.expect(TokenKind::Identifier);
        if p.check(TokenKind::LParen) {
            class_modification(p);
        }
    } else {
        p.expect(TokenKind::Identifier);
    }
    description_string(p);
    composition(p);
    p.expect(TokenKind::End);
    p.expect(TokenKind::Identifier);
    p.exit(mark, SyntaxKind::LongClassSpecifier);
}

fn short_class_specifier(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Equal);
    if p.consume(TokenKind::Enumeration) {
        p.expect(TokenKind::LParen);
        if !p.consume(TokenKind::Colon) && p.check(TokenKind::Identifier) {
            enum_list(p);
        }
        p.expect(TokenKind::RParen);
    } else {
        base_prefix(p);
        type_specifier(p);
        if p.check(TokenKind::LBracket) {
            array_subscripts(p);
        }
        if p.check(TokenKind::LParen) {
            class_modification(p);
        }
    }
    description(p);
    p.exit(mark, SyntaxKind::ShortClassSpecifier);
}

fn der_class_specifier(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Equal);
    p.expect(TokenKind::Der);
    p.expect(TokenKind::LParen);
    type_specifier(p);
    p.expect(TokenKind::Comma);
    p.expect(TokenKind::Identifier);
    while p.consume(TokenKind::Comma) && !p.eof() {
        p.expect(TokenKind::Identifier);
    }
    p.expect(TokenKind::RParen);
    description(p);
    p.exit(mark, SyntaxKind::DerClassSpecifier);
}

fn base_prefix(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(TokenKind::Input) {
        p.consume(TokenKind::Output);
    }
    p.exit(mark, SyntaxKind::BasePrefix);
}

fn enum_list(p: &mut Parser) {
    let mark = p.enter();
    enumeration_literal(p);
    while p.consume(TokenKind::Comma) && !p.eof() {
        enumeration_literal(p);
    }
    p.exit(mark, SyntaxKind::EnumList);
}

fn enumeration_literal(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    description(p);
    p.exit(mark, SyntaxKind::EnumerationLiteral);
}

fn composition(p: &mut Parser) {
    let mark = p.enter();
    element_list(p);
    while !p.check_any(&[TokenKind::External, TokenKind::Annotation, TokenKind::End]) && !p.eof() {
        let k = p.nth(0);
        match k {
            TokenKind::Public | TokenKind::Protected => {
                p.advance();
                element_list(p);
            }
            TokenKind::Initial => match p.nth(1) {
                TokenKind::Equation => {
                    equation_section(p);
                }
                TokenKind::Algorithm => {
                    algorithm_section(p);
                }
                _ => p.advance_with_error(format!("unexpected token '{:?}' following 'initial'. Expected 'equation' or 'algorithm'", p.nth(1))),
            },
            TokenKind::Equation => {
                equation_section(p);
            }
            TokenKind::Algorithm => {
                algorithm_section(p);
            }
            _ => p.advance_with_error(
                format!(
                    "unexpected token '{:?}' after element list inside composition. Expected 'protected', 'public', 'initial', 'equation', 'algorithm', 'external', 'annotation' or 'end'.",
                    p.nth(0)
                )
            ),
        }
    }
    if p.consume(TokenKind::External) {
        if p.check(TokenKind::String) {
            language_specification(p);
        }
        if p.check_any(&[TokenKind::Dot, TokenKind::Identifier]) {
            external_function_call(p);
        }
        if p.check(TokenKind::Annotation) {
            annotation_clause(p);
        }
        p.expect(TokenKind::Semicolon);
    }
    if p.check(TokenKind::Annotation) {
        annotation_clause(p);
        p.expect(TokenKind::Semicolon);
    }
    p.exit(mark, SyntaxKind::Composition);
}

fn language_specification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::String);
    p.exit(mark, SyntaxKind::LanguageSpecification);
}

fn external_function_call(p: &mut Parser) {
    let mark = p.enter();
    if p.nth(1) != TokenKind::LParen {
        component_reference(p);
        p.expect(TokenKind::Equal);
    }
    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::LParen);
    if !p.check(TokenKind::RParen) {
        expression_list(p);
    }
    p.expect(TokenKind::RParen);
    p.exit(mark, SyntaxKind::ExternalFunctionCall);
}

fn element_list(p: &mut Parser) {
    let mark = p.enter();
    while !p.check_any(&SECTION_BREAKERS) && !p.eof() {
        element(p);
        p.expect(TokenKind::Semicolon);
    }
    p.exit(mark, SyntaxKind::ElementList);
}

fn element(p: &mut Parser) {
    let mark = p.enter();
    if p.check(TokenKind::Import) {
        import_clause(p);
    } else if p.check(TokenKind::Extends) {
        extends_clause(p);
    } else {
        p.consume(TokenKind::Redeclare);
        p.consume(TokenKind::Final);
        p.consume(TokenKind::Inner);
        p.consume(TokenKind::Outer);
        if p.consume(TokenKind::Replaceable) {
            if p.check_any(&CLASS_PREFS) || p.check(TokenKind::Encapsulated) {
                class_definition(p);
            } else {
                component_clause(p);
            }
            if p.check(TokenKind::Constrainedby) {
                constraining_clause(p);
                description(p);
            }
        } else if p.check_any(&CLASS_PREFS) || p.check(TokenKind::Encapsulated) {
            class_definition(p);
        } else {
            component_clause(p);
        }
    }
    p.exit(mark, SyntaxKind::Element);
}

fn import_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Import);
    if p.nth(1) == TokenKind::Equal {
        p.expect(TokenKind::Identifier);
        p.advance();
        name(p);
    } else {
        name(p);
        if !p.consume(TokenKind::DotStar) && p.consume(TokenKind::Dot) {
            p.expect(TokenKind::LCurly);
            import_list(p);
            p.expect(TokenKind::RCurly);
        }
    }
    description(p);
    p.exit(mark, SyntaxKind::ImportClause);
}

fn import_list(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    while p.consume(TokenKind::Comma) && !p.eof() {
        p.expect(TokenKind::Identifier);
    }
    p.exit(mark, SyntaxKind::ImportList);
}

// A.2.3 Extends

fn extends_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Extends);
    type_specifier(p);
    if p.check(TokenKind::LParen) {
        class_or_inheritance_modification(p);
    }
    if p.check(TokenKind::Annotation) {
        annotation_clause(p);
    }
    p.exit(mark, SyntaxKind::ExtendsClause);
}

fn constraining_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Constrainedby);
    type_specifier(p);
    if p.check(TokenKind::LParen) {
        class_modification(p);
    }
    p.exit(mark, SyntaxKind::ConstrainingClause);
}

fn class_or_inheritance_modification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::LParen);
    if !p.consume(TokenKind::RParen) {
        argument_or_inheritance_modification_list(p);
        p.expect(TokenKind::RParen);
    }
    p.exit(mark, SyntaxKind::ClassOrInheritanceModification);
}

fn argument_or_inheritance_modification_list(p: &mut Parser) {
    let mark = p.enter();
    if p.check(TokenKind::Break) {
        inheritance_modification(p);
    } else {
        argument(p);
    }
    while p.consume(TokenKind::Comma) && !p.eof() {
        if p.check(TokenKind::Break) {
            inheritance_modification(p);
        } else {
            argument(p);
        }
    }
    p.exit(mark, SyntaxKind::ArgumentOrInheritanceModificationList);
}

fn inheritance_modification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Break);
    if p.check(TokenKind::Connect) {
        connect_equation(p);
    } else {
        p.expect(TokenKind::Identifier);
    }
    p.exit(mark, SyntaxKind::InheritanceModification);
}

// A.2.4 Component Clause

fn component_clause(p: &mut Parser) {
    let mark = p.enter();
    type_prefix(p);
    type_specifier(p);
    if p.check(TokenKind::LBracket) {
        array_subscripts(p);
    }
    component_list(p);
    p.exit(mark, SyntaxKind::ComponentClause);
}

fn type_prefix(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(TokenKind::Flow) {
        p.consume(TokenKind::Stream);
    }
    if !p.consume(TokenKind::Discrete) && !p.consume(TokenKind::Parameter) {
        p.consume(TokenKind::Constant);
    }
    if !p.consume(TokenKind::Input) {
        p.consume(TokenKind::Output);
    }
    p.exit(mark, SyntaxKind::TypePrefix);
}

fn component_list(p: &mut Parser) {
    let mark = p.enter();
    component_declaration(p);
    while p.consume(TokenKind::Comma) && !p.eof() {
        component_declaration(p);
    }
    p.exit(mark, SyntaxKind::ComponentList);
}

fn component_declaration(p: &mut Parser) {
    let mark = p.enter();
    declaration(p);
    if p.check(TokenKind::If) {
        condition_attribute(p);
    }
    description(p);
    p.exit(mark, SyntaxKind::ComponentDeclaration);
}

fn condition_attribute(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::If);
    expression(p);
    p.exit(mark, SyntaxKind::ConditionAttribute);
}

fn declaration(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    if p.check(TokenKind::LBracket) {
        array_subscripts(p);
    }
    if p.check_any(&[TokenKind::LParen, TokenKind::Equal, TokenKind::Assign]) {
        modification(p);
    }
    p.exit(mark, SyntaxKind::Declaration);
}

// A.2.5 Modification

fn modification(p: &mut Parser) {
    let mark = p.enter();
    if p.check_any(&[TokenKind::Equal, TokenKind::Assign]) {
        p.advance();
        modification_expression(p);
    } else {
        class_modification(p);
        if p.consume(TokenKind::Equal) {
            modification_expression(p);
        }
    }
    p.exit(mark, SyntaxKind::Modification);
}

fn modification_expression(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(TokenKind::Break) {
        expression(p);
    }
    p.exit(mark, SyntaxKind::ModificationExpression);
}

fn class_modification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::LParen);
    if !p.consume(TokenKind::RParen) {
        argument_list(p);
        p.expect(TokenKind::RParen);
    }
    p.exit(mark, SyntaxKind::ClassModification);
}

fn argument_list(p: &mut Parser) {
    let mark = p.enter();
    argument(p);
    while p.consume(TokenKind::Comma) && !p.eof() {
        argument(p);
    }
    p.exit(mark, SyntaxKind::ArgumentList);
}

fn argument(p: &mut Parser) {
    let mark = p.enter();
    if p.check(TokenKind::Redeclare) {
        element_redeclaration(p);
    } else {
        element_modification_or_replaceable(p);
    }
    p.exit(mark, SyntaxKind::Argument);
}

fn element_modification_or_replaceable(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Each);
    p.consume(TokenKind::Final);
    if p.check(TokenKind::Replaceable) {
        element_replaceable(p);
    } else {
        element_modification(p);
    }
    p.exit(mark, SyntaxKind::ElementModificationOrReplaceable);
}

fn element_modification(p: &mut Parser) {
    let mark = p.enter();
    name(p);
    if p.check_any(&[TokenKind::LParen, TokenKind::Equal, TokenKind::Assign]) {
        modification(p);
    }
    description_string(p);
    p.exit(mark, SyntaxKind::ElementModification);
}

fn element_redeclaration(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Redeclare);
    p.consume(TokenKind::Each);
    p.consume(TokenKind::Final);
    if p.check_any(&CLASS_PREFS) {
        short_class_definition(p);
    } else if p.check(TokenKind::Replaceable) {
        element_replaceable(p);
    } else {
        component_clause1(p);
    }
    p.exit(mark, SyntaxKind::ElementRedeclaration);
}

fn element_replaceable(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Replaceable);
    if p.check_any(&CLASS_PREFS) {
        short_class_definition(p);
    } else {
        component_clause1(p);
    }
    if p.check(TokenKind::Constrainedby) {
        constraining_clause(p);
    }
    p.exit(mark, SyntaxKind::ElementReplaceable);
}

fn component_clause1(p: &mut Parser) {
    let mark = p.enter();
    type_prefix(p);
    type_specifier(p);
    component_declaration1(p);
    p.exit(mark, SyntaxKind::ComponentClause1);
}

fn component_declaration1(p: &mut Parser) {
    let mark = p.enter();
    declaration(p);
    description(p);
    p.exit(mark, SyntaxKind::ComponentDeclaration1);
}

fn short_class_definition(p: &mut Parser) {
    let mark = p.enter();
    class_prefixes(p);
    short_class_specifier(p);
    p.exit(mark, SyntaxKind::ShortClassDefinition);
}

// A.2.6 Equations

fn equation_section(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Initial);
    p.expect(TokenKind::Equation);
    while !p.check_any(&SECTION_BREAKERS) && !p.eof() {
        equation(p);
        p.expect(TokenKind::Semicolon);
    }
    p.exit(mark, SyntaxKind::EquationSection);
}

fn algorithm_section(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Initial);
    p.expect(TokenKind::Algorithm);
    while !p.check_any(&SECTION_BREAKERS) && !p.eof() {
        statement(p);
        p.expect(TokenKind::Semicolon);
    }
    p.exit(mark, SyntaxKind::AlgorithmSection);
}

fn equation(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        TokenKind::If => if_equation(p),
        TokenKind::For => for_equation(p),
        TokenKind::When => when_equation(p),
        TokenKind::Connect => connect_equation(p),
        _ => {
            // FIXME: It is somewhat simplified for now. Specification
            // allows only `component-reference func-call-args`, so not
            // every `simple-expression` that is not followed by the `=`
            // can be accepted
            simple_expression(p);
            if p.consume(TokenKind::Equal) {
                expression(p);
            }
        }
    }
    description(p);
    p.exit(mark, SyntaxKind::Equation);
}

fn statement(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        TokenKind::If => if_statement(p),
        TokenKind::For => for_statement(p),
        TokenKind::While => while_statement(p),
        TokenKind::When => when_statement(p),
        TokenKind::Break | TokenKind::Return => p.advance(),
        TokenKind::LParen => {
            p.advance();
            output_expression_list(p);
            p.expect(TokenKind::RParen);
            p.expect(TokenKind::Assign);
            component_reference(p);
            function_call_args(p);
        }
        _ => {
            component_reference(p);
            if p.consume(TokenKind::Assign) {
                expression(p);
            } else {
                function_call_args(p);
            }
        }
    }
    description(p);
    p.exit(mark, SyntaxKind::Statement);
}

fn if_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::If);
    expression(p);
    p.expect(TokenKind::Then);
    while !p.check_any(&[TokenKind::ElseIf, TokenKind::Else, TokenKind::End]) && !p.eof() {
        equation(p);
        p.expect(TokenKind::Semicolon);
    }
    while !p.check_any(&[TokenKind::Else, TokenKind::End]) & !p.eof() {
        p.expect(TokenKind::ElseIf);
        expression(p);
        p.expect(TokenKind::Then);
        while !p.check_any(&[TokenKind::ElseIf, TokenKind::Else, TokenKind::End]) && !p.eof() {
            equation(p);
            p.expect(TokenKind::Semicolon);
        }
    }
    if p.consume(TokenKind::Else) {
        while !p.check(TokenKind::End) && !p.eof() {
            equation(p);
            p.expect(TokenKind::Semicolon);
        }
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::If);
    p.exit(mark, SyntaxKind::IfEquation);
}

fn if_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::If);
    expression(p);
    p.expect(TokenKind::Then);
    while !p.check_any(&[TokenKind::ElseIf, TokenKind::Else, TokenKind::End]) && !p.eof() {
        statement(p);
        p.expect(TokenKind::Semicolon);
    }
    while !p.check_any(&[TokenKind::Else, TokenKind::End]) & !p.eof() {
        p.expect(TokenKind::ElseIf);
        expression(p);
        p.expect(TokenKind::Then);
        while !p.check_any(&[TokenKind::ElseIf, TokenKind::Else, TokenKind::End]) && !p.eof() {
            statement(p);
            p.expect(TokenKind::Semicolon);
        }
    }
    if p.consume(TokenKind::Else) {
        while !p.check(TokenKind::End) && !p.eof() {
            statement(p);
            p.expect(TokenKind::Semicolon);
        }
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::If);
    p.exit(mark, SyntaxKind::IfStatement);
}

fn for_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::For);
    for_indices(p);
    p.expect(TokenKind::Loop);
    while !p.check(TokenKind::End) && !p.eof() {
        equation(p);
        p.expect(TokenKind::Semicolon);
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::For);
    p.exit(mark, SyntaxKind::ForEquation);
}

fn for_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::For);
    for_indices(p);
    p.expect(TokenKind::Loop);
    while !p.check(TokenKind::End) && !p.eof() {
        statement(p);
        p.expect(TokenKind::Semicolon);
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::For);
    p.exit(mark, SyntaxKind::ForStatement);
}

fn for_indices(p: &mut Parser) {
    let mark = p.enter();
    for_index(p);
    while p.consume(TokenKind::Comma) && !p.eof() {
        for_index(p);
    }
    p.exit(mark, SyntaxKind::ForIndices);
}

fn for_index(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    if p.consume(TokenKind::In) {
        expression(p);
    }
    p.exit(mark, SyntaxKind::ForIndex);
}

fn while_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::While);
    expression(p);
    p.expect(TokenKind::Loop);
    while !p.check(TokenKind::End) && !p.eof() {
        statement(p);
        p.expect(TokenKind::Semicolon);
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::While);
    p.exit(mark, SyntaxKind::WhileStatement);
}

fn when_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::When);
    expression(p);
    p.expect(TokenKind::Then);
    while !p.check_any(&[TokenKind::ElseWhen, TokenKind::End]) && !p.eof() {
        equation(p);
        p.expect(TokenKind::Semicolon);
    }
    while !p.check(TokenKind::End) & !p.eof() {
        p.expect(TokenKind::ElseWhen);
        expression(p);
        p.expect(TokenKind::Then);
        while !p.check_any(&[TokenKind::ElseWhen, TokenKind::End]) && !p.eof() {
            equation(p);
            p.expect(TokenKind::Semicolon);
        }
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::When);
    p.exit(mark, SyntaxKind::WhenEquation);
}

fn when_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::When);
    expression(p);
    p.expect(TokenKind::Then);
    while !p.check_any(&[TokenKind::ElseWhen, TokenKind::End]) && !p.eof() {
        statement(p);
        p.expect(TokenKind::Semicolon);
    }
    while !p.check(TokenKind::End) & !p.eof() {
        p.expect(TokenKind::ElseWhen);
        expression(p);
        p.expect(TokenKind::Then);
        while !p.check_any(&[TokenKind::ElseWhen, TokenKind::End]) && !p.eof() {
            statement(p);
            p.expect(TokenKind::Semicolon);
        }
    }
    p.expect(TokenKind::End);
    p.expect(TokenKind::When);
    p.exit(mark, SyntaxKind::WhenStatement);
}

fn connect_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Connect);
    p.expect(TokenKind::LParen);
    component_reference(p);
    p.expect(TokenKind::Comma);
    component_reference(p);
    p.expect(TokenKind::RParen);
    p.exit(mark, SyntaxKind::ConnectEquation);
}

// A.2.7 Expressions

fn expression(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        TokenKind::If => {
            p.advance();
            expression(p);
            p.expect(TokenKind::Then);
            expression(p);
            while !p.check(TokenKind::Else) && !p.eof() {
                p.expect(TokenKind::ElseIf);
                expression(p);
                p.expect(TokenKind::Then);
                expression(p);
            }
            p.expect(TokenKind::Else);
            expression(p);
        }
        _ => simple_expression(p),
    }
    p.exit(mark, SyntaxKind::Expression);
}

fn simple_expression(p: &mut Parser) {
    let mark = p.enter();
    logical_expression(p);
    if p.consume(TokenKind::Colon) {
        logical_expression(p);
        if p.consume(TokenKind::Colon) {
            logical_expression(p);
        }
    }
    p.exit(mark, SyntaxKind::SimpleExpression);
}

fn logical_expression(p: &mut Parser) {
    let mark = p.enter();
    logical_term(p);
    while p.consume(TokenKind::Or) && !p.eof() {
        logical_term(p);
    }
    p.exit(mark, SyntaxKind::LogicalExpression);
}

fn logical_term(p: &mut Parser) {
    let mark = p.enter();
    logical_factor(p);
    while p.consume(TokenKind::And) && !p.eof() {
        logical_factor(p);
    }
    p.exit(mark, SyntaxKind::LogicalTerm);
}

fn logical_factor(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Not);
    relation(p);
    p.exit(mark, SyntaxKind::LogicalFactor);
}

fn relation(p: &mut Parser) {
    const RELOPS: [TokenKind; 6] = [
        TokenKind::Les,
        TokenKind::Leq,
        TokenKind::Gre,
        TokenKind::Geq,
        TokenKind::Eq,
        TokenKind::Neq,
    ];
    let mark = p.enter();
    arithmetic_expression(p);
    if p.check_any(&RELOPS) {
        relational_operator(p);
        arithmetic_expression(p);
    }
    p.exit(mark, SyntaxKind::Relation);
}

fn relational_operator(p: &mut Parser) {
    let mark = p.enter();
    // It is only called in `relation`, which already does the checking,
    // so no need to check token once again
    p.advance();
    p.exit(mark, SyntaxKind::RelationalOperator);
}

fn arithmetic_expression(p: &mut Parser) {
    const ADDOPS: [TokenKind; 4] = [
        TokenKind::Plus,
        TokenKind::DotPlus,
        TokenKind::Minus,
        TokenKind::DotMinus,
    ];
    let mark = p.enter();
    if p.check_any(&ADDOPS) {
        add_operator(p);
    }
    term(p);
    while p.check_any(&ADDOPS) && !p.eof() {
        add_operator(p);
        term(p);
    }
    p.exit(mark, SyntaxKind::ArithmeticExpression);
}

fn add_operator(p: &mut Parser) {
    let mark = p.enter();
    // It is only called in `arithmetic_expression`, which already does the checking,
    // so no need to check token once again
    p.advance();
    p.exit(mark, SyntaxKind::AddOperator);
}

fn term(p: &mut Parser) {
    const MULOPS: [TokenKind; 4] = [
        TokenKind::Star,
        TokenKind::DotStar,
        TokenKind::Slash,
        TokenKind::DotSlash,
    ];
    let mark = p.enter();
    factor(p);
    while p.check_any(&MULOPS) && !p.eof() {
        mul_operator(p);
        factor(p);
    }
    p.exit(mark, SyntaxKind::Term);
}

fn mul_operator(p: &mut Parser) {
    let mark = p.enter();
    // It is only called in `term`, which already does the checking,
    // so no need to check token once again
    p.advance();
    p.exit(mark, SyntaxKind::MulOperator);
}

fn factor(p: &mut Parser) {
    let mark = p.enter();
    primary(p);
    if p.check_any(&[TokenKind::Flex, TokenKind::DotFlex]) {
        p.advance();
        primary(p);
    }
    p.exit(mark, SyntaxKind::Factor);
}

fn primary(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        TokenKind::UReal
        | TokenKind::UInt
        | TokenKind::String
        | TokenKind::Bool
        | TokenKind::End => p.advance(),
        TokenKind::LParen => {
            p.advance();
            output_expression_list(p);
            p.expect(TokenKind::RParen);
            if p.check(TokenKind::LBracket) {
                array_subscripts(p);
            }
        }
        TokenKind::LBracket => {
            p.advance();
            expression_list(p);
            while p.consume(TokenKind::Semicolon) && !p.eof() {
                expression_list(p);
            }
            p.expect(TokenKind::RBracket);
        }
        TokenKind::LCurly => {
            p.advance();
            array_arguments(p);
            p.expect(TokenKind::RCurly);
        }
        TokenKind::Der | TokenKind::Initial | TokenKind::Pure => {
            p.advance();
            function_call_args(p);
        }
        _ => {
            component_reference(p);
            if p.check(TokenKind::LParen) {
                function_call_args(p);
            }
        }
    }
    p.exit(mark, SyntaxKind::Primary);
}

fn type_specifier(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Dot);
    name(p);
    p.exit(mark, SyntaxKind::TypeSpecifier);
}

fn name(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    while p.check(TokenKind::Dot) && !p.eof() {
        if p.nth(1) == TokenKind::Identifier {
            p.advance();
            p.advance();
        } else if p.nth(1) == TokenKind::LCurly {
            break;
        } else {
            p.advance_with_error(format!(
                "unexpected token '{:?}' after '.'. Expected identifier or '{{'",
                p.nth(1)
            ));
        }
    }
    p.exit(mark, SyntaxKind::Name);
}

fn component_reference(p: &mut Parser) {
    let mark = p.enter();
    p.consume(TokenKind::Dot);
    p.expect(TokenKind::Identifier);
    if p.check(TokenKind::LBracket) {
        array_subscripts(p);
    }
    while p.consume(TokenKind::Dot) && !p.eof() {
        p.expect(TokenKind::Identifier);
        if p.check(TokenKind::LBracket) {
            array_subscripts(p);
        }
    }
    p.exit(mark, SyntaxKind::ComponentReference);
}

fn result_reference(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(TokenKind::Der) {
        p.expect(TokenKind::LParen);
        component_reference(p);
        if p.consume(TokenKind::Comma) {
            p.expect(TokenKind::UInt);
        }
        p.expect(TokenKind::RParen);
    } else {
        component_reference(p);
    }
    p.exit(mark, SyntaxKind::ResultReference);
}

fn function_call_args(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::LParen);
    if !p.consume(TokenKind::RParen) {
        function_arguments(p);
        p.expect(TokenKind::RParen);
    }
    p.exit(mark, SyntaxKind::FunctionCallArgs);
}

fn function_arguments(p: &mut Parser) {
    let mark = p.enter();
    if p.nth(1) == TokenKind::Equal {
        named_arguments(p);
    } else if !p.check(TokenKind::Function) {
        expression(p);
        if p.consume(TokenKind::Comma) {
            function_arguments_non_first(p);
        } else if p.consume(TokenKind::For) {
            for_indices(p);
        }
    } else {
        function_partial_application(p);
        if p.consume(TokenKind::Comma) {
            function_arguments_non_first(p);
        }
    }
    p.exit(mark, SyntaxKind::FunctionArguments);
}

fn function_arguments_non_first(p: &mut Parser) {
    let mark = p.enter();
    if p.nth(1) == TokenKind::Equal {
        named_arguments(p);
    } else {
        function_argument(p);
        if p.consume(TokenKind::Comma) {
            function_arguments_non_first(p);
        }
    }
    p.exit(mark, SyntaxKind::FunctionArgumentsNonFirst);
}

fn array_arguments(p: &mut Parser) {
    let mark = p.enter();
    expression(p);
    if p.consume(TokenKind::Comma) {
        array_arguments_non_first(p);
    } else if p.consume(TokenKind::For) {
        for_indices(p);
    }
    p.exit(mark, SyntaxKind::ArrayArguments);
}

fn array_arguments_non_first(p: &mut Parser) {
    let mark = p.enter();
    expression(p);
    if p.consume(TokenKind::Comma) {
        array_arguments_non_first(p);
    }
    p.exit(mark, SyntaxKind::ArrayArgumentsNonFirst);
}

fn named_arguments(p: &mut Parser) {
    let mark = p.enter();
    named_argument(p);
    if p.consume(TokenKind::Comma) {
        named_arguments(p);
    }
    p.exit(mark, SyntaxKind::NamedArguments);
}

fn named_argument(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Equal);
    function_argument(p);
    p.exit(mark, SyntaxKind::NamedArgument);
}

fn function_argument(p: &mut Parser) {
    let mark = p.enter();
    if p.check(TokenKind::Function) {
        function_partial_application(p);
    } else {
        expression(p);
    }
    p.exit(mark, SyntaxKind::FunctionArgument);
}

fn function_partial_application(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Function);
    type_specifier(p);
    p.expect(TokenKind::LParen);
    if p.check(TokenKind::Identifier) {
        named_arguments(p);
    }
    p.expect(TokenKind::RParen);
    p.exit(mark, SyntaxKind::FunctionPartialApplication);
}

fn output_expression_list(p: &mut Parser) {
    let mark = p.enter();
    // This production can only occur inside parentheses, so easiest way
    // is to check for right paren
    if !p.check(TokenKind::RParen) {
        if !p.check_any(&[TokenKind::RParen, TokenKind::Comma]) {
            expression(p);
        }
        while p.consume(TokenKind::Comma) && !p.eof() {
            if !p.check_any(&[TokenKind::RParen, TokenKind::Comma]) {
                expression(p);
            }
        }
    }
    p.exit(mark, SyntaxKind::OutputExpressionList);
}

fn expression_list(p: &mut Parser) {
    let mark = p.enter();
    expression(p);
    while p.consume(TokenKind::Comma) && !p.eof() {
        expression(p);
    }
    p.exit(mark, SyntaxKind::ExpressionList);
}

fn array_subscripts(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::LBracket);
    subscript(p);
    while p.consume(TokenKind::Comma) && !p.eof() {
        subscript(p);
    }
    p.expect(TokenKind::RBracket);
    p.exit(mark, SyntaxKind::ArraySubscripts);
}

fn subscript(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(TokenKind::Colon) {
        expression(p);
    }
    p.exit(mark, SyntaxKind::Subscript);
}

fn description(p: &mut Parser) {
    let mark = p.enter();
    description_string(p);
    if p.check(TokenKind::Annotation) {
        annotation_clause(p);
    }
    p.exit(mark, SyntaxKind::Description);
}

fn description_string(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(TokenKind::String) {
        while p.consume(TokenKind::Plus) && !p.eof() {
            p.expect(TokenKind::String);
        }
    }
    p.exit(mark, SyntaxKind::DescriptionString);
}

fn annotation_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(TokenKind::Annotation);
    class_modification(p);
    p.exit(mark, SyntaxKind::AnnotationClause);
}
