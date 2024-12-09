use std::cell::Cell;

use super::events::SyntaxEvent;
use super::syntax::SyntaxKind;
use super::tokens::{ModelicaToken, Token};

pub fn events(
    name: &str,
    tokens: &Vec<Token>,
    start: SyntaxKind,
) -> (Vec<SyntaxEvent>, Vec<String>) {
    let mut parser = Parser::new(name, tokens);
    parser.parse(start);
    (parser.events, parser.errors)
}

/// Represents a Modelica parser
struct Parser<'a> {
    /// Source name
    name: &'a str,
    /// Scanned tokens
    tokens: &'a Vec<Token>,
    /// Collected syntax events
    events: Vec<SyntaxEvent>,
    /// Collection of errors
    errors: Vec<String>,
    /// Current position in the `TokenCollection`
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
    fn new(name: &'a str, tokens: &'a Vec<Token>) -> Self {
        let cap = tokens.len();
        Parser {
            name,
            tokens,
            events: Vec::with_capacity(cap),
            errors: Vec::new(),
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
        self.events.push(SyntaxEvent::Advance);
        self.pos += 1;
        self.lifes.set(100);
    }

    /// Return `true` if parser reached the end of file
    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    /// Return type of the n-th token counting from the current one.
    fn nth(&self, n: usize) -> ModelicaToken {
        if self.lifes.get() == 0 {
            self.blowup();
        }
        self.lifes.set(self.lifes.get() - 1);
        self.tokens
            .get(self.pos + n)
            .map_or(ModelicaToken::EOF, |tok| tok.kind)
    }

    fn blowup(&self) {
        let tok = if !self.eof() {
            self.tokens.get(self.pos).unwrap()
        } else {
            self.tokens.last().unwrap()
        };
        panic!(
            "{}:{}:{}: Parser stuck",
            self.name, tok.start.line, tok.start.col
        );
    }

    /// Return `true` if current token matches the specified type
    fn check(&self, typ: ModelicaToken) -> bool {
        self.nth(0) == typ
    }

    /// Return `true` if current token matches any of the specified types
    fn check_any(&self, typ: &[ModelicaToken]) -> bool {
        typ.contains(&self.nth(0))
    }

    /// Return `true` if current token matches the specified type and
    /// advance the parser. Otherwise return `false` and do not advance.
    fn consume(&mut self, typ: ModelicaToken) -> bool {
        if self.check(typ) {
            self.advance();
            return true;
        }
        false
    }

    /// Mark currently parsed token as erroneus.
    fn error(&mut self, msg: String) {
        if let Some(tok) = self.tokens.get(self.pos) {
            self.errors.push(format!(
                "{}:{}:{}: {}",
                self.name, tok.start.line, tok.start.col, msg
            ));
        } else {
            self.errors.push(format!(
                "{}:{}:{}: {}",
                self.name,
                self.tokens.last().unwrap().start.line,
                self.tokens.last().unwrap().start.col,
                msg
            ));
        }
    }

    fn advance_with_error(&mut self, msg: String) {
        self.error(msg);
        let mark = self.enter();
        self.advance();
        self.exit(mark, SyntaxKind::Error);
    }

    /// Advance the parser if current token is expected. Report error if
    /// current tokens doesn't match the specified type.
    fn expect(&mut self, typ: ModelicaToken) {
        if !self.consume(typ) {
            self.error(format!("expected {typ:?}, found {:?}", self.nth(0)));
        }
    }
}

// Useful constants used in the parsing process

const SECTION_BREAKERS: [ModelicaToken; 8] = [
    ModelicaToken::Protected,
    ModelicaToken::Public,
    ModelicaToken::Initial,
    ModelicaToken::Equation,
    ModelicaToken::Algorithm,
    ModelicaToken::End,
    ModelicaToken::Annotation,
    ModelicaToken::External,
];
const CLASS_PREFS: [ModelicaToken; 13] = [
    ModelicaToken::Partial,
    ModelicaToken::Class,
    ModelicaToken::Model,
    ModelicaToken::Record,
    ModelicaToken::Function,
    ModelicaToken::Block,
    ModelicaToken::Type,
    ModelicaToken::Operator,
    ModelicaToken::Connector,
    ModelicaToken::Package,
    ModelicaToken::Pure,
    ModelicaToken::Impure,
    ModelicaToken::Expandable,
];

// A.2.1 Stored Definition – Within

fn stored_definition(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(ModelicaToken::Within) {
        if !p.check(ModelicaToken::Semicolon) {
            name(p);
        }
        p.expect(ModelicaToken::Semicolon);
    }
    while !p.eof() {
        p.consume(ModelicaToken::Final);
        class_definition(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.exit(mark, SyntaxKind::StoredDefinition);
}

// A.2.2 Class Definition

fn class_definition(p: &mut Parser) {
    let mark = p.enter();
    p.consume(ModelicaToken::Encapsulated);
    class_prefixes(p);
    class_specifier(p);
    p.exit(mark, SyntaxKind::ClassDefinition);
}

fn class_prefixes(p: &mut Parser) {
    let mark = p.enter();
    p.consume(ModelicaToken::Partial);
    let pref = p.nth(0);
    match pref {
        ModelicaToken::Class
        | ModelicaToken::Model
        | ModelicaToken::Block
        | ModelicaToken::Type
        | ModelicaToken::Package
        | ModelicaToken::Record
        | ModelicaToken::Connector
        | ModelicaToken::Function => {
            p.advance();
        }
        ModelicaToken::Expandable => {
            p.advance();
            p.expect(ModelicaToken::Connector);
        }
        ModelicaToken::Operator => {
            p.advance();
            if !p.consume(ModelicaToken::Record) {
                p.consume(ModelicaToken::Function);
            }
        }
        ModelicaToken::Pure | ModelicaToken::Impure => {
            p.advance();
            p.consume(ModelicaToken::Operator);
            p.expect(ModelicaToken::Function);
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
    if p.check(ModelicaToken::Extends) {
        long_class_specifier(p);
    } else if p.check(ModelicaToken::Identifier) {
        if p.nth(1) != ModelicaToken::Equal {
            long_class_specifier(p);
        } else if p.nth(2) == ModelicaToken::Der {
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
    if p.consume(ModelicaToken::Extends) {
        p.expect(ModelicaToken::Identifier);
        if p.check(ModelicaToken::LParen) {
            class_modification(p);
        }
    } else {
        p.expect(ModelicaToken::Identifier);
    }
    description_string(p);
    composition(p);
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::Identifier);
    p.exit(mark, SyntaxKind::LongClassSpecifier);
}

fn short_class_specifier(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    p.expect(ModelicaToken::Equal);
    if p.consume(ModelicaToken::Enumeration) {
        p.expect(ModelicaToken::LParen);
        if !p.consume(ModelicaToken::Colon) && p.check(ModelicaToken::Identifier) {
            enum_list(p);
        }
        p.expect(ModelicaToken::RParen);
    } else {
        base_prefix(p);
        type_specifier(p);
        if p.check(ModelicaToken::LBracket) {
            array_subscripts(p);
        }
        if p.check(ModelicaToken::LParen) {
            class_modification(p);
        }
    }
    description(p);
    p.exit(mark, SyntaxKind::ShortClassSpecifier);
}

fn der_class_specifier(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    p.expect(ModelicaToken::Equal);
    p.expect(ModelicaToken::Der);
    p.expect(ModelicaToken::LParen);
    type_specifier(p);
    p.expect(ModelicaToken::Comma);
    p.expect(ModelicaToken::Identifier);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        p.expect(ModelicaToken::Identifier);
    }
    p.expect(ModelicaToken::RParen);
    description(p);
    p.exit(mark, SyntaxKind::DerClassSpecifier);
}

fn base_prefix(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(ModelicaToken::Input) {
        p.consume(ModelicaToken::Output);
    }
    p.exit(mark, SyntaxKind::BasePrefix);
}

fn enum_list(p: &mut Parser) {
    let mark = p.enter();
    enumeration_literal(p);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        enumeration_literal(p);
    }
    p.exit(mark, SyntaxKind::EnumList);
}

fn enumeration_literal(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    description(p);
    p.exit(mark, SyntaxKind::EnumerationLiteral);
}

fn composition(p: &mut Parser) {
    let mark = p.enter();
    element_list(p);
    while !p.check_any(&[
        ModelicaToken::External,
        ModelicaToken::Annotation,
        ModelicaToken::End,
    ]) && !p.eof()
    {
        let k = p.nth(0);
        match k {
            ModelicaToken::Public | ModelicaToken::Protected => {
                p.advance();
                element_list(p);
            }
            ModelicaToken::Initial => match p.nth(1) {
                ModelicaToken::Equation => {
                    equation_section(p);
                }
                ModelicaToken::Algorithm => {
                    algorithm_section(p);
                }
                _ => p.advance_with_error(format!("unexpected token '{:?}' following 'initial'. Expected 'equation' or 'algorithm'", p.nth(1))),
            },
            ModelicaToken::Equation => {
                equation_section(p);
            }
            ModelicaToken::Algorithm => {
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
    if p.consume(ModelicaToken::External) {
        if p.check(ModelicaToken::String) {
            language_specification(p);
        }
        if p.check_any(&[ModelicaToken::Dot, ModelicaToken::Identifier]) {
            external_function_call(p);
        }
        if p.check(ModelicaToken::Annotation) {
            annotation_clause(p);
        }
        p.expect(ModelicaToken::Semicolon);
    }
    if p.check(ModelicaToken::Annotation) {
        annotation_clause(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.exit(mark, SyntaxKind::Composition);
}

fn language_specification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::String);
    p.exit(mark, SyntaxKind::LanguageSpecification);
}

fn external_function_call(p: &mut Parser) {
    let mark = p.enter();
    if p.nth(1) != ModelicaToken::LParen {
        component_reference(p);
        p.expect(ModelicaToken::Equal);
    }
    p.expect(ModelicaToken::Identifier);
    p.expect(ModelicaToken::LParen);
    if !p.check(ModelicaToken::RParen) {
        expression_list(p);
    }
    p.expect(ModelicaToken::RParen);
    p.exit(mark, SyntaxKind::ExternalFunctionCall);
}

fn element_list(p: &mut Parser) {
    let mark = p.enter();
    while !p.check_any(&SECTION_BREAKERS) && !p.eof() {
        element(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.exit(mark, SyntaxKind::ElementList);
}

fn element(p: &mut Parser) {
    let mark = p.enter();
    if p.check(ModelicaToken::Import) {
        import_clause(p);
    } else if p.check(ModelicaToken::Extends) {
        extends_clause(p);
    } else {
        p.consume(ModelicaToken::Redeclare);
        p.consume(ModelicaToken::Final);
        p.consume(ModelicaToken::Inner);
        p.consume(ModelicaToken::Outer);
        if p.consume(ModelicaToken::Replaceable) {
            if p.check_any(&CLASS_PREFS) || p.check(ModelicaToken::Encapsulated) {
                class_definition(p);
            } else {
                component_clause(p);
            }
            if p.check(ModelicaToken::Constrainedby) {
                constraining_clause(p);
                description(p);
            }
        } else if p.check_any(&CLASS_PREFS) || p.check(ModelicaToken::Encapsulated) {
            class_definition(p);
        } else {
            component_clause(p);
        }
    }
    p.exit(mark, SyntaxKind::Element);
}

fn import_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Import);
    if p.nth(1) == ModelicaToken::Equal {
        p.expect(ModelicaToken::Identifier);
        p.advance();
        name(p);
    } else {
        name(p);
        if !p.consume(ModelicaToken::DotStar) && p.consume(ModelicaToken::Dot) {
            p.expect(ModelicaToken::LCurly);
            import_list(p);
            p.expect(ModelicaToken::RCurly);
        }
    }
    description(p);
    p.exit(mark, SyntaxKind::ImportClause);
}

fn import_list(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        p.expect(ModelicaToken::Identifier);
    }
    p.exit(mark, SyntaxKind::ImportList);
}

// A.2.3 Extends

fn extends_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Extends);
    type_specifier(p);
    if p.check(ModelicaToken::LParen) {
        class_or_inheritance_modification(p);
    }
    if p.check(ModelicaToken::Annotation) {
        annotation_clause(p);
    }
    p.exit(mark, SyntaxKind::ExtendsClause);
}

fn constraining_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Constrainedby);
    type_specifier(p);
    if p.check(ModelicaToken::LParen) {
        class_modification(p);
    }
    p.exit(mark, SyntaxKind::ConstrainingClause);
}

fn class_or_inheritance_modification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::LParen);
    if !p.consume(ModelicaToken::RParen) {
        argument_or_inheritance_modification_list(p);
        p.expect(ModelicaToken::RParen);
    }
    p.exit(mark, SyntaxKind::ClassOrInheritanceModification);
}

fn argument_or_inheritance_modification_list(p: &mut Parser) {
    let mark = p.enter();
    if p.check(ModelicaToken::Break) {
        inheritance_modification(p);
    } else {
        argument(p);
    }
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        if p.check(ModelicaToken::Break) {
            inheritance_modification(p);
        } else {
            argument(p);
        }
    }
    p.exit(mark, SyntaxKind::ArgumentOrInheritanceModificationList);
}

fn inheritance_modification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Break);
    if p.check(ModelicaToken::Connect) {
        connect_equation(p);
    } else {
        p.expect(ModelicaToken::Identifier);
    }
    p.exit(mark, SyntaxKind::InheritanceModification);
}

// A.2.4 Component Clause

fn component_clause(p: &mut Parser) {
    let mark = p.enter();
    type_prefix(p);
    type_specifier(p);
    if p.check(ModelicaToken::LBracket) {
        array_subscripts(p);
    }
    component_list(p);
    p.exit(mark, SyntaxKind::ComponentClause);
}

fn type_prefix(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(ModelicaToken::Flow) {
        p.consume(ModelicaToken::Stream);
    }
    if !p.consume(ModelicaToken::Discrete) && !p.consume(ModelicaToken::Parameter) {
        p.consume(ModelicaToken::Constant);
    }
    if !p.consume(ModelicaToken::Input) {
        p.consume(ModelicaToken::Output);
    }
    p.exit(mark, SyntaxKind::TypePrefix);
}

fn component_list(p: &mut Parser) {
    let mark = p.enter();
    component_declaration(p);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        component_declaration(p);
    }
    p.exit(mark, SyntaxKind::ComponentList);
}

fn component_declaration(p: &mut Parser) {
    let mark = p.enter();
    declaration(p);
    if p.check(ModelicaToken::If) {
        condition_attribute(p);
    }
    description(p);
    p.exit(mark, SyntaxKind::ComponentDeclaration);
}

fn condition_attribute(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::If);
    expression(p);
    p.exit(mark, SyntaxKind::ConditionAttribute);
}

fn declaration(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    if p.check(ModelicaToken::LBracket) {
        array_subscripts(p);
    }
    if p.check_any(&[
        ModelicaToken::LParen,
        ModelicaToken::Equal,
        ModelicaToken::Assign,
    ]) {
        modification(p);
    }
    p.exit(mark, SyntaxKind::Declaration);
}

// A.2.5 Modification

fn modification(p: &mut Parser) {
    let mark = p.enter();
    if p.check_any(&[ModelicaToken::Equal, ModelicaToken::Assign]) {
        p.advance();
        modification_expression(p);
    } else {
        class_modification(p);
        if p.consume(ModelicaToken::Equal) {
            modification_expression(p);
        }
    }
    p.exit(mark, SyntaxKind::Modification);
}

fn modification_expression(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(ModelicaToken::Break) {
        expression(p);
    }
    p.exit(mark, SyntaxKind::ModificationExpression);
}

fn class_modification(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::LParen);
    if !p.consume(ModelicaToken::RParen) {
        argument_list(p);
        p.expect(ModelicaToken::RParen);
    }
    p.exit(mark, SyntaxKind::ClassModification);
}

fn argument_list(p: &mut Parser) {
    let mark = p.enter();
    argument(p);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        argument(p);
    }
    p.exit(mark, SyntaxKind::ArgumentList);
}

fn argument(p: &mut Parser) {
    let mark = p.enter();
    if p.check(ModelicaToken::Redeclare) {
        element_redeclaration(p);
    } else {
        element_modification_or_replaceable(p);
    }
    p.exit(mark, SyntaxKind::Argument);
}

fn element_modification_or_replaceable(p: &mut Parser) {
    let mark = p.enter();
    p.consume(ModelicaToken::Each);
    p.consume(ModelicaToken::Final);
    if p.check(ModelicaToken::Replaceable) {
        element_replaceable(p);
    } else {
        element_modification(p);
    }
    p.exit(mark, SyntaxKind::ElementModificationOrReplaceable);
}

fn element_modification(p: &mut Parser) {
    let mark = p.enter();
    name(p);
    if p.check_any(&[
        ModelicaToken::LParen,
        ModelicaToken::Equal,
        ModelicaToken::Assign,
    ]) {
        modification(p);
    }
    description_string(p);
    p.exit(mark, SyntaxKind::ElementModification);
}

fn element_redeclaration(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Redeclare);
    p.consume(ModelicaToken::Each);
    p.consume(ModelicaToken::Final);
    if p.check_any(&CLASS_PREFS) {
        short_class_definition(p);
    } else if p.check(ModelicaToken::Replaceable) {
        element_replaceable(p);
    } else {
        component_clause1(p);
    }
    p.exit(mark, SyntaxKind::ElementRedeclaration);
}

fn element_replaceable(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Replaceable);
    if p.check_any(&CLASS_PREFS) {
        short_class_definition(p);
    } else {
        component_clause1(p);
    }
    if p.check(ModelicaToken::Constrainedby) {
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
    p.consume(ModelicaToken::Initial);
    p.expect(ModelicaToken::Equation);
    while !p.check_any(&SECTION_BREAKERS) && !p.eof() {
        equation(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.exit(mark, SyntaxKind::EquationSection);
}

fn algorithm_section(p: &mut Parser) {
    let mark = p.enter();
    p.consume(ModelicaToken::Initial);
    p.expect(ModelicaToken::Algorithm);
    while !p.check_any(&SECTION_BREAKERS) && !p.eof() {
        statement(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.exit(mark, SyntaxKind::AlgorithmSection);
}

fn equation(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        ModelicaToken::If => if_equation(p),
        ModelicaToken::For => for_equation(p),
        ModelicaToken::When => when_equation(p),
        ModelicaToken::Connect => connect_equation(p),
        _ => {
            // FIXME: It is somewhat simplified for now. Specification
            // allows only `component-reference func-call-args`, so not
            // every `simple-expression` that is not followed by the `=`
            // can be accepted
            simple_expression(p);
            if p.consume(ModelicaToken::Equal) {
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
        ModelicaToken::If => if_statement(p),
        ModelicaToken::For => for_statement(p),
        ModelicaToken::While => while_statement(p),
        ModelicaToken::When => when_statement(p),
        ModelicaToken::Break | ModelicaToken::Return => p.advance(),
        ModelicaToken::LParen => {
            p.advance();
            output_expression_list(p);
            p.expect(ModelicaToken::RParen);
            p.expect(ModelicaToken::Assign);
            component_reference(p);
            function_call_args(p);
        }
        _ => {
            component_reference(p);
            if p.consume(ModelicaToken::Assign) {
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
    p.expect(ModelicaToken::If);
    expression(p);
    p.expect(ModelicaToken::Then);
    while !p.check_any(&[
        ModelicaToken::ElseIf,
        ModelicaToken::Else,
        ModelicaToken::End,
    ]) && !p.eof()
    {
        equation(p);
        p.expect(ModelicaToken::Semicolon);
    }
    while !p.check_any(&[ModelicaToken::Else, ModelicaToken::End]) & !p.eof() {
        p.expect(ModelicaToken::ElseIf);
        expression(p);
        p.expect(ModelicaToken::Then);
        while !p.check_any(&[
            ModelicaToken::ElseIf,
            ModelicaToken::Else,
            ModelicaToken::End,
        ]) && !p.eof()
        {
            equation(p);
            p.expect(ModelicaToken::Semicolon);
        }
    }
    if p.consume(ModelicaToken::Else) {
        while !p.check(ModelicaToken::End) && !p.eof() {
            equation(p);
            p.expect(ModelicaToken::Semicolon);
        }
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::If);
    p.exit(mark, SyntaxKind::IfEquation);
}

fn if_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::If);
    expression(p);
    p.expect(ModelicaToken::Then);
    while !p.check_any(&[
        ModelicaToken::ElseIf,
        ModelicaToken::Else,
        ModelicaToken::End,
    ]) && !p.eof()
    {
        statement(p);
        p.expect(ModelicaToken::Semicolon);
    }
    while !p.check_any(&[ModelicaToken::Else, ModelicaToken::End]) & !p.eof() {
        p.expect(ModelicaToken::ElseIf);
        expression(p);
        p.expect(ModelicaToken::Then);
        while !p.check_any(&[
            ModelicaToken::ElseIf,
            ModelicaToken::Else,
            ModelicaToken::End,
        ]) && !p.eof()
        {
            statement(p);
            p.expect(ModelicaToken::Semicolon);
        }
    }
    if p.consume(ModelicaToken::Else) {
        while !p.check(ModelicaToken::End) && !p.eof() {
            statement(p);
            p.expect(ModelicaToken::Semicolon);
        }
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::If);
    p.exit(mark, SyntaxKind::IfStatement);
}

fn for_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::For);
    for_indices(p);
    p.expect(ModelicaToken::Loop);
    while !p.check(ModelicaToken::End) && !p.eof() {
        equation(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::For);
    p.exit(mark, SyntaxKind::ForEquation);
}

fn for_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::For);
    for_indices(p);
    p.expect(ModelicaToken::Loop);
    while !p.check(ModelicaToken::End) && !p.eof() {
        statement(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::For);
    p.exit(mark, SyntaxKind::ForStatement);
}

fn for_indices(p: &mut Parser) {
    let mark = p.enter();
    for_index(p);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        for_index(p);
    }
    p.exit(mark, SyntaxKind::ForIndices);
}

fn for_index(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    if p.consume(ModelicaToken::In) {
        expression(p);
    }
    p.exit(mark, SyntaxKind::ForIndex);
}

fn while_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::While);
    expression(p);
    p.expect(ModelicaToken::Loop);
    while !p.check(ModelicaToken::End) && !p.eof() {
        statement(p);
        p.expect(ModelicaToken::Semicolon);
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::While);
    p.exit(mark, SyntaxKind::WhileStatement);
}

fn when_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::When);
    expression(p);
    p.expect(ModelicaToken::Then);
    while !p.check_any(&[ModelicaToken::ElseWhen, ModelicaToken::End]) && !p.eof() {
        equation(p);
        p.expect(ModelicaToken::Semicolon);
    }
    while !p.check(ModelicaToken::End) & !p.eof() {
        p.expect(ModelicaToken::ElseWhen);
        expression(p);
        p.expect(ModelicaToken::Then);
        while !p.check_any(&[ModelicaToken::ElseWhen, ModelicaToken::End]) && !p.eof() {
            equation(p);
            p.expect(ModelicaToken::Semicolon);
        }
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::When);
    p.exit(mark, SyntaxKind::WhenEquation);
}

fn when_statement(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::When);
    expression(p);
    p.expect(ModelicaToken::Then);
    while !p.check_any(&[ModelicaToken::ElseWhen, ModelicaToken::End]) && !p.eof() {
        statement(p);
        p.expect(ModelicaToken::Semicolon);
    }
    while !p.check(ModelicaToken::End) & !p.eof() {
        p.expect(ModelicaToken::ElseWhen);
        expression(p);
        p.expect(ModelicaToken::Then);
        while !p.check_any(&[ModelicaToken::ElseWhen, ModelicaToken::End]) && !p.eof() {
            statement(p);
            p.expect(ModelicaToken::Semicolon);
        }
    }
    p.expect(ModelicaToken::End);
    p.expect(ModelicaToken::When);
    p.exit(mark, SyntaxKind::WhenStatement);
}

fn connect_equation(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Connect);
    p.expect(ModelicaToken::LParen);
    component_reference(p);
    p.expect(ModelicaToken::Comma);
    component_reference(p);
    p.expect(ModelicaToken::RParen);
    p.exit(mark, SyntaxKind::ConnectEquation);
}

// A.2.7 Expressions

fn expression(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        ModelicaToken::If => {
            p.advance();
            expression(p);
            p.expect(ModelicaToken::Then);
            expression(p);
            while !p.check(ModelicaToken::Else) && !p.eof() {
                p.expect(ModelicaToken::ElseIf);
                expression(p);
                p.expect(ModelicaToken::Then);
                expression(p);
            }
            p.expect(ModelicaToken::Else);
            expression(p);
        }
        _ => simple_expression(p),
    }
    p.exit(mark, SyntaxKind::Expression);
}

fn simple_expression(p: &mut Parser) {
    let mark = p.enter();
    logical_expression(p);
    if p.consume(ModelicaToken::Colon) {
        logical_expression(p);
        if p.consume(ModelicaToken::Colon) {
            logical_expression(p);
        }
    }
    p.exit(mark, SyntaxKind::SimpleExpression);
}

fn logical_expression(p: &mut Parser) {
    let mark = p.enter();
    logical_term(p);
    while p.consume(ModelicaToken::Or) && !p.eof() {
        logical_term(p);
    }
    p.exit(mark, SyntaxKind::LogicalExpression);
}

fn logical_term(p: &mut Parser) {
    let mark = p.enter();
    logical_factor(p);
    while p.consume(ModelicaToken::And) && !p.eof() {
        logical_factor(p);
    }
    p.exit(mark, SyntaxKind::LogicalTerm);
}

fn logical_factor(p: &mut Parser) {
    let mark = p.enter();
    p.consume(ModelicaToken::Not);
    relation(p);
    p.exit(mark, SyntaxKind::LogicalFactor);
}

fn relation(p: &mut Parser) {
    const RELOPS: [ModelicaToken; 6] = [
        ModelicaToken::Les,
        ModelicaToken::Leq,
        ModelicaToken::Gre,
        ModelicaToken::Geq,
        ModelicaToken::Eq,
        ModelicaToken::Neq,
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
    const ADDOPS: [ModelicaToken; 4] = [
        ModelicaToken::Plus,
        ModelicaToken::DotPlus,
        ModelicaToken::Minus,
        ModelicaToken::DotMinus,
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
    const MULOPS: [ModelicaToken; 4] = [
        ModelicaToken::Star,
        ModelicaToken::DotStar,
        ModelicaToken::Slash,
        ModelicaToken::DotSlash,
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
    if p.check_any(&[ModelicaToken::Flex, ModelicaToken::DotFlex]) {
        p.advance();
        primary(p);
    }
    p.exit(mark, SyntaxKind::Factor);
}

fn primary(p: &mut Parser) {
    let mark = p.enter();
    match p.nth(0) {
        ModelicaToken::UReal
        | ModelicaToken::UInt
        | ModelicaToken::String
        | ModelicaToken::Bool
        | ModelicaToken::End => p.advance(),
        ModelicaToken::LParen => {
            p.advance();
            output_expression_list(p);
            p.expect(ModelicaToken::RParen);
            if p.check(ModelicaToken::LBracket) {
                array_subscripts(p);
            }
        }
        ModelicaToken::LBracket => {
            p.advance();
            expression_list(p);
            while p.consume(ModelicaToken::Semicolon) && !p.eof() {
                expression_list(p);
            }
            p.expect(ModelicaToken::RBracket);
        }
        ModelicaToken::LCurly => {
            p.advance();
            array_arguments(p);
            p.expect(ModelicaToken::RCurly);
        }
        ModelicaToken::Der | ModelicaToken::Initial | ModelicaToken::Pure => {
            p.advance();
            function_call_args(p);
        }
        _ => {
            component_reference(p);
            if p.check(ModelicaToken::LParen) {
                function_call_args(p);
            }
        }
    }
    p.exit(mark, SyntaxKind::Primary);
}

fn type_specifier(p: &mut Parser) {
    let mark = p.enter();
    p.consume(ModelicaToken::Dot);
    name(p);
    p.exit(mark, SyntaxKind::TypeSpecifier);
}

fn name(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    while p.check(ModelicaToken::Dot) && !p.eof() {
        if p.nth(1) == ModelicaToken::Identifier {
            p.advance();
            p.advance();
        } else if p.nth(1) == ModelicaToken::LCurly {
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
    p.consume(ModelicaToken::Dot);
    p.expect(ModelicaToken::Identifier);
    if p.check(ModelicaToken::LBracket) {
        array_subscripts(p);
    }
    while p.consume(ModelicaToken::Dot) && !p.eof() {
        p.expect(ModelicaToken::Identifier);
        if p.check(ModelicaToken::LBracket) {
            array_subscripts(p);
        }
    }
    p.exit(mark, SyntaxKind::ComponentReference);
}

fn result_reference(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(ModelicaToken::Der) {
        p.expect(ModelicaToken::LParen);
        component_reference(p);
        if p.consume(ModelicaToken::Comma) {
            p.expect(ModelicaToken::UInt);
        }
        p.expect(ModelicaToken::RParen);
    } else {
        component_reference(p);
    }
    p.exit(mark, SyntaxKind::ResultReference);
}

fn function_call_args(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::LParen);
    if !p.consume(ModelicaToken::RParen) {
        function_arguments(p);
        p.expect(ModelicaToken::RParen);
    }
    p.exit(mark, SyntaxKind::FunctionCallArgs);
}

fn function_arguments(p: &mut Parser) {
    let mark = p.enter();
    if p.nth(1) == ModelicaToken::Equal {
        named_arguments(p);
    } else if !p.check(ModelicaToken::Function) {
        expression(p);
        if p.consume(ModelicaToken::Comma) {
            function_arguments_non_first(p);
        } else if p.consume(ModelicaToken::For) {
            for_indices(p);
        }
    } else {
        function_partial_application(p);
        if p.consume(ModelicaToken::Comma) {
            function_arguments_non_first(p);
        }
    }
    p.exit(mark, SyntaxKind::FunctionArguments);
}

fn function_arguments_non_first(p: &mut Parser) {
    let mark = p.enter();
    if p.nth(1) == ModelicaToken::Equal {
        named_arguments(p);
    } else {
        function_argument(p);
        if p.consume(ModelicaToken::Comma) {
            function_arguments_non_first(p);
        }
    }
    p.exit(mark, SyntaxKind::FunctionArgumentsNonFirst);
}

fn array_arguments(p: &mut Parser) {
    let mark = p.enter();
    expression(p);
    if p.consume(ModelicaToken::Comma) {
        array_arguments_non_first(p);
    } else if p.consume(ModelicaToken::For) {
        for_indices(p);
    }
    p.exit(mark, SyntaxKind::ArrayArguments);
}

fn array_arguments_non_first(p: &mut Parser) {
    let mark = p.enter();
    expression(p);
    if p.consume(ModelicaToken::Comma) {
        array_arguments_non_first(p);
    }
    p.exit(mark, SyntaxKind::ArrayArgumentsNonFirst);
}

fn named_arguments(p: &mut Parser) {
    let mark = p.enter();
    named_argument(p);
    if p.consume(ModelicaToken::Comma) {
        named_arguments(p);
    }
    p.exit(mark, SyntaxKind::NamedArguments);
}

fn named_argument(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Identifier);
    p.expect(ModelicaToken::Equal);
    function_argument(p);
    p.exit(mark, SyntaxKind::NamedArgument);
}

fn function_argument(p: &mut Parser) {
    let mark = p.enter();
    if p.check(ModelicaToken::Function) {
        function_partial_application(p);
    } else {
        expression(p);
    }
    p.exit(mark, SyntaxKind::FunctionArgument);
}

fn function_partial_application(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Function);
    type_specifier(p);
    p.expect(ModelicaToken::LParen);
    if p.check(ModelicaToken::Identifier) {
        named_arguments(p);
    }
    p.expect(ModelicaToken::RParen);
    p.exit(mark, SyntaxKind::FunctionPartialApplication);
}

fn output_expression_list(p: &mut Parser) {
    let mark = p.enter();
    // This production can only occur inside parentheses, so easiest way
    // is to check for right paren
    if !p.check(ModelicaToken::RParen) {
        if !p.check_any(&[ModelicaToken::RParen, ModelicaToken::Comma]) {
            expression(p);
        }
        while p.consume(ModelicaToken::Comma) && !p.eof() {
            if !p.check_any(&[ModelicaToken::RParen, ModelicaToken::Comma]) {
                expression(p);
            }
        }
    }
    p.exit(mark, SyntaxKind::OutputExpressionList);
}

fn expression_list(p: &mut Parser) {
    let mark = p.enter();
    expression(p);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        expression(p);
    }
    p.exit(mark, SyntaxKind::ExpressionList);
}

fn array_subscripts(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::LBracket);
    subscript(p);
    while p.consume(ModelicaToken::Comma) && !p.eof() {
        subscript(p);
    }
    p.expect(ModelicaToken::RBracket);
    p.exit(mark, SyntaxKind::ArraySubscripts);
}

fn subscript(p: &mut Parser) {
    let mark = p.enter();
    if !p.consume(ModelicaToken::Colon) {
        expression(p);
    }
    p.exit(mark, SyntaxKind::Subscript);
}

fn description(p: &mut Parser) {
    let mark = p.enter();
    description_string(p);
    if p.check(ModelicaToken::Annotation) {
        annotation_clause(p);
    }
    p.exit(mark, SyntaxKind::Description);
}

fn description_string(p: &mut Parser) {
    let mark = p.enter();
    if p.consume(ModelicaToken::String) {
        while p.consume(ModelicaToken::Plus) && !p.eof() {
            p.expect(ModelicaToken::String);
        }
    }
    p.exit(mark, SyntaxKind::DescriptionString);
}

fn annotation_clause(p: &mut Parser) {
    let mark = p.enter();
    p.expect(ModelicaToken::Annotation);
    class_modification(p);
    p.exit(mark, SyntaxKind::AnnotationClause);
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::lexing::lex;

    fn get_events(source: &str, start: SyntaxKind) -> (Vec<SyntaxEvent>, Vec<String>) {
        let (tokens, _, mut errors) = lex("none", source);
        let (events, mut parser_errors) = events("none", &tokens, start);
        errors.append(&mut parser_errors);
        (events, errors)
    }

    #[test]
    fn parse_imports() {
        let source: &str = "import Foo.Bar";
        let (_, errors) = get_events(source, SyntaxKind::ImportClause);
        assert_eq!(errors.len(), 0);
        let source: &str = "import Foo = Bar.Baz";
        let (_, errors) = get_events(source, SyntaxKind::ImportClause);
        assert_eq!(errors.len(), 0);
        let source: &str = "import Foo.*";
        let (_, errors) = get_events(source, SyntaxKind::ImportClause);
        assert_eq!(errors.len(), 0);
        let source: &str = "import Foo.{Bar, Baz}";
        let (_, errors) = get_events(source, SyntaxKind::ImportClause);
        assert_eq!(errors.len(), 0);
    }
    #[test]
    fn parse_expression() {
        let source: &str = "(-a) + 2 * foo(x ./ 2) ^ 3";
        let (_, errors) = get_events(source, SyntaxKind::Expression);
        assert_eq!(errors.len(), 0);
        let source: &str = "if x ^ 2 > 2 then 2 elseif x ^ 2 < 2 and x >= 0 then -1 else 0";
        let (_, errors) = get_events(source, SyntaxKind::Expression);
        assert_eq!(errors.len(), 0);
    }
    #[test]
    fn parse_primary() {
        let source: &str = "foo.bar(a=5)";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        let source: &str = "der(x)";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        // Output list
        let source: &str = "(x, , y, x + 5, )";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        // Matrix
        let source: &str = "[1, 2, 3; a, b, c; x, y, z]";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        // Arrays
        let source: &str = "{x * i for i in 1:n}";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        let source: &str = "{x + y, x, 1}";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        // Incorrect syntax
        let source: &str = "{x + y, x, 2 * i for i in 1:n}";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_ne!(errors.len(), 0);
    }
    #[test]
    fn parse_component_reference() {
        let source: &str = "a";
        let (_, errors) = get_events(source, SyntaxKind::ComponentReference);
        assert_eq!(errors.len(), 0);
        let source: &str = "foo.bar.baz";
        let (_, errors) = get_events(source, SyntaxKind::ComponentReference);
        assert_eq!(errors.len(), 0);
        let source: &str = ".foo.bar";
        let (_, errors) = get_events(source, SyntaxKind::ComponentReference);
        assert_eq!(errors.len(), 0);
        let source: &str = ".foo[1, 2].bar[3]";
        let (_, errors) = get_events(source, SyntaxKind::ComponentReference);
        assert_eq!(errors.len(), 0);
        let source: &str = "foo[1, 2].bar[:, x].baz";
        let (_, errors) = get_events(source, SyntaxKind::ComponentReference);
        assert_eq!(errors.len(), 0);
    }
    #[test]
    fn parse_function_call_args() {
        // Only named args
        let source: &str = "(a = 2, b = function .Foo.Bar(a = 5), c = x + y)";
        let (_, errors) = get_events(source, SyntaxKind::FunctionCallArgs);
        assert_eq!(errors.len(), 0);
        // Correctly mixed args
        let source: &str = "(function .Foo.Bar(a = 5), 5, b = 3, c = x + y)";
        let (_, errors) = get_events(source, SyntaxKind::FunctionCallArgs);
        assert_eq!(errors.len(), 0);
        // Incorrectly mixed args
        let source: &str = "(a = 2, 5, b = 3, c = x + y)";
        let (_, errors) = get_events(source, SyntaxKind::FunctionCallArgs);
        assert_ne!(errors.len(), 0);
    }
    #[test]
    fn parse_output_lists() {
        let source = "(x, y, z)";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        let source = "(, y, z)";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        let source = "(, , z)";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
        let source = "(, , )";
        let (_, errors) = get_events(source, SyntaxKind::Primary);
        assert_eq!(errors.len(), 0);
    }
    #[test]
    fn parse_array_subscripts() {
        let source = "[i, i+1, :]";
        let (_, errors) = get_events(source, SyntaxKind::ArraySubscripts);
        assert_eq!(errors.len(), 0);
    }
    #[test]
    fn parse_description() {
        const SOURCE1: &str = "\"Some text\" + \"and more text\" + \"and little more text\"";
        let (events, _) = get_events(SOURCE1, SyntaxKind::DescriptionString);
        assert_eq!(events.len(), 7);
        let source2: &str = &(String::from(SOURCE1)
            + " annotation (Dialog(tab = \"General\", group = \"General\"))");
        let (_, errors) = get_events(source2, SyntaxKind::Description);
        assert_eq!(errors.len(), 0);
    }
}
