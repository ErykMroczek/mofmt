// This is ANTLR-4 file defining Modelica language parser rules
// Grammar files were defined based on the Modelica Specification
// See: https://specification.modelica.org/master/modelica-concrete-syntax.html
// There are some modifications, mostly few new "wrapper" rules, that
// were introduced to simplify some logic in mofmt. Such rules were
// marked with proper comments.

// Grammar file created by Eryk Mroczek

parser grammar Modelica;
options { tokenVocab=ModelicaLexer; }

stored_definition
    : (WITHIN name? SEMICOLON)? (FINAL? class_definition SEMICOLON)*
    ;

class_definition
    : ENCAPSULATED? class_prefixes class_specifier
    ;

class_prefixes
    : PARTIAL?
    (
        CLASS
        | MODEL
        | OPERATOR? RECORD
        | BLOCK
        | EXPANDABLE? CONNECTOR
        | TYPE
        | PACKAGE
        | (PURE | IMPURE)? OPERATOR? FUNCTION
        | OPERATOR
    )
    ;

class_specifier
    : long_class_specifier
    | short_class_specifier
    | der_class_specifier
    ;

long_class_specifier
    : IDENT description_string composition end_clause
    | EXTENDS IDENT class_modification? description_string composition end_clause
    ;

// not present in spec
end_clause
    : END IDENT
    ;

short_class_specifier
    : IDENT EQUAL base_prefix type_specifier array_subscripts? class_modification? description
    | IDENT EQUAL ENUMERATION enumerations description
    ;

der_class_specifier
    : IDENT EQUAL DER LPAREN type_specifier COMMA IDENT (COMMA IDENT)* RPAREN description
    ;

base_prefix
    : (INPUT | OUTPUT)?
    ;

// not present in spec
enumerations
    : LPAREN ( enum_list? | COLON ) RPAREN
    ;

enum_list
    : enumeration_literal (COMMA enumeration_literal)*
    ;

enumeration_literal
    : IDENT description
    ;

composition
    : initial_element_list
    (
        public_element_list
        | protected_element_list
        | equation_section
        | algorithm_section
    )*
    external_element?
    class_annotation?
    ;

// not present in spec
class_annotation
    : ANNOTATION class_modification SEMICOLON
    ;

// not present in spec
external_element
    : (EXTERNAL language_specification? external_function_call?
        annotation? SEMICOLON)
    ;

language_specification
    : STRING
    ;

external_function_call
    : (component_reference EQUAL)? IDENT external_function_args
    ;

// not present in spec
external_function_args
    : LPAREN expression_list? RPAREN
    ;

// not present in spec
initial_element_list
    : element_list
    ;

// not present in spec
public_element_list
    : PUBLIC element_list
    ;

// not present in spec
protected_element_list
    : PROTECTED element_list
    ;

element_list
    : (element SEMICOLON)*
    ;

element
    : import_clause
    | extends_clause
    | declaration_clause
    ;

import_clause
    : IMPORT
    (
        IDENT EQUAL name
        | name (DOTSTAR | (DOT LCURLY import_list RCURLY))?
    )
    description
    ;

 import_list
    : IDENT (COMMA IDENT)*
    ;

// not present in spec
declaration_clause
    : REDECLARE? FINAL? INNER? OUTER? REPLACEABLE?
      (class_definition | component_clause) (constraining_clause description)?
    ;

extends_clause
    : EXTENDS type_specifier class_or_inheritance_modification? annotation?
    ;

constraining_clause
    : CONSTRAINEDBY type_specifier class_modification?
    ;

class_or_inheritance_modification
    : LPAREN argument_or_inheritance_modification_list? RPAREN
    ;

argument_or_inheritance_modification_list
    : (argument | inheritance_modification) (COMMA (argument | inheritance_modification))*
    ;

inheritance_modification
    : BREAK (connect_equation | IDENT)
    ;

component_clause
    : type_prefix type_specifier array_subscripts? component_list
    ;

type_prefix
    : (FLOW | STREAM)?
      (DISCRETE | PARAMETER | CONSTANT)?
      (INPUT | OUTPUT)?
    ;

component_list
    : component_declaration (COMMA component_declaration)*
    ;

component_declaration
    : declaration (IF expression)? description
    ;

declaration
    : IDENT array_subscripts? modification?
    ;

modification
    : class_modification (EQUAL modification_expression)?
    | EQUAL modification_expression
    | ASSIGN modification_expression
    ;

modification_expression
    : expression
    | BREAK
    ;

class_modification
    : LPAREN argument_list? RPAREN
    ;

argument_list
    : argument (COMMA argument)*
    ;

argument
    : element_modification_or_replaceable
    | element_redeclaration
    ;

element_modification_or_replaceable
    : EACH? FINAL? (element_modification | element_replaceable)
    ;

element_modification
    : name modification? description_string
    ;

element_redeclaration
    : REDECLARE EACH? FINAL?
    (
        short_definition
        | short_component_clause
        | element_replaceable
    )
    ;

element_replaceable
    : REPLACEABLE
    (
        short_definition
        | short_component_clause
    )
    constraining_clause?
    ;

// originally called component-clause1 in spec
short_component_clause
    : type_prefix type_specifier short_component_declaration
    ;

// originally called component-declaration1 in spec
short_component_declaration
    : declaration description
    ;

// originally called short-class-definition in spec
short_definition
    : class_prefixes short_class_specifier
    ;

equation_section
    : INITIAL? EQUATION equation_list
    ;

algorithm_section
    : INITIAL? ALGORITHM statement_list
    ;

// not present in spec
equation_list
    : (equation SEMICOLON)*
    ;

// not present in spec
statement_list
    : (statement SEMICOLON)*
    ;

equation
    : (
        simple_expression EQUAL expression
        | if_equation
        | for_equation
        | connect_equation
        | when_equation
        | component_reference function_call_args
    )
    description
    ;

statement
    : (
        component_reference (ASSIGN expression | function_call_args)
        | LPAREN output_expression_list RPAREN ASSIGN component_reference function_call_args
        | BREAK
        | RETURN
        | if_statement
        | for_statement
        | while_statement
        | when_statement
    )
    description
    ;

if_equation
    : if_branch
      conditional_equations
      (
        elseif_branch
        conditional_equations
      )*
      (
        else_branch
        conditional_equations
      )?
      END IF
    ;

// not present in spec
conditional_equations
    : (equation SEMICOLON)*
    ;

if_statement
    : if_branch
      conditional_statements
      (
        elseif_branch
        conditional_statements
      )*
      (
        else_branch
        conditional_statements
      )?
      END IF
    ;

// not present in spec
if_branch
    : IF expression THEN
    ;

// not present in spec
elseif_branch
    : ELSEIF expression THEN
    ;

// not present in spec
else_branch
    : ELSE
    ;

// not present in spec
conditional_statements
    : (statement SEMICOLON)*
    ;

for_equation
    : FOR for_indices LOOP
      conditional_equations
      END FOR
    ;

for_statement
    : FOR for_indices LOOP
      conditional_statements
      END FOR
    ;

for_indices
    : for_index (COMMA for_index)*
    ;

for_index
    :
    IDENT (IN expression)?
    ;

while_statement
    : WHILE expression LOOP
      conditional_statements
      END WHILE
    ;

when_equation
    : when_branch
      conditional_equations
      (
        elsewhen_branch
        conditional_equations
      )*
      END WHEN
    ;

when_statement
    : when_branch
      conditional_statements
      (
        elsewhen_branch
        conditional_statements
      )*
      END WHEN
    ;

// not present in spec
when_branch
    : WHEN expression THEN
    ;

// not present in spec
elsewhen_branch
    : ELSEWHEN expression THEN
    ;

connect_equation
    : CONNECT connected_components
    ;

// not present in spec
connected_components
    : LPAREN component_reference COMMA component_reference RPAREN
    ;

expression
    : simple_expression | if_expression
    ;

// not present in spec
if_expression
    : if_eval conditional_expression
      (elseif_eval conditional_expression)*
      else_eval conditional_expression
    ;

// not present in spec
if_eval
    : IF expression THEN
    ;

// not present in spec
elseif_eval
    : ELSEIF expression THEN
    ;

// not present in spec
else_eval
    : ELSE
    ;

// not present in spec
conditional_expression
    : expression
    ;

simple_expression
    : logical_expression (COLON logical_expression (COLON logical_expression)?)?
    ;

logical_expression
    : logical_term (or_operator logical_term)*
    ;

// not present in spec
or_operator
    : OR
    ;

logical_term
    : logical_factor (and_operator logical_factor)*
    ;

// not present in spec
and_operator
    : AND
    ;

logical_factor
    : NOT? relation
    ;

relation
    : arithmetic_expression (relational_operator arithmetic_expression)?
    ;

relational_operator
    : GRE | GEQ
    | LES | LEQ
    | NEQ | EEQ
    ;

arithmetic_expression
    : term (add_operator term)*
    ;

// not present in spec
unary_expression
    : add_operator unary_operand
    ;

// not present in spec
unary_operand
    : primary
    ;

add_operator
    : PLUS | MINUS
    | DOTPLUS | DOTMINUS
    ;

term
    : factor (mul_operator factor)*
    ;

mul_operator
    : STAR | SLASH
    | DOTSTAR | DOTSLASH
    ;

factor
    : primary (exp_operator primary)?
    ;

exp_operator
    : FLEX | DOTFLEX
    ;


primary
    : UNUM
    | STRING
    | BOOL
    | unary_expression
    | (component_reference | DER | INITIAL | PURE) function_call_args
    | component_reference
    | LPAREN output_expression_list RPAREN
    | LBRACK expression_list (SEMICOLON expression_list)* RBRACK
    | LCURLY array_arguments RCURLY
    | END
    ;

type_specifier
    : DOT? IDENT (DOT IDENT)*  ;

name
    : IDENT (DOT IDENT)*
    ;

component_reference
    : DOT? IDENT array_subscripts? (DOT IDENT array_subscripts?)*
    ;

function_call_args
    : LPAREN function_arguments? RPAREN
    ;

function_arguments
    : expression FOR for_indices
    | function_argument (COMMA function_argument)* (COMMA named_arguments)?
    | named_arguments
    ;

named_arguments
    : named_argument (COMMA named_argument)*
    ;

named_argument
    : IDENT EQUAL function_argument
    ;

function_argument
    : function_partial_application
    | expression
    ;

function_partial_application
    : FUNCTION type_specifier LPAREN named_arguments? RPAREN
    ;

output_expression_list
    : expression? (COMMA expression?)*
    ;

expression_list
    : expression (COMMA expression)*
    ;

array_arguments
    : expression ((COMMA expression)* | FOR for_indices)
    ;

array_subscripts
    : LBRACK subscript (COMMA subscript)* RBRACK
    ;

subscript
    : COLON | expression
    ;

description
    : description_string annotation?
    ;


description_string
    : (STRING (cat_operator STRING)*)?
    ;

// not present in spec
cat_operator
    : PLUS
    ;

annotation
    : ANNOTATION class_modification
    ;
