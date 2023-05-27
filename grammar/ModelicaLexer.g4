// This is ANTLR-4 file defining Modelica language tokens
// Grammar files were defined based on the Modelica Specification
// See: https://specification.modelica.org/master/modelica-concrete-syntax.html

// Grammar defined by Eryk Mroczek

lexer grammar ModelicaLexer;

channels {
    WHITESPACE,
    COMMENTS
}

// Whitespace tokens
WS : [ \r\n\t] + -> channel (WHITESPACE) ;

// Misc tokens
DQUOTE : '"' ;
COMMA : ',' ;
DOT : '.' ;
SEMICOLON : ';' ;
COLON : ':' ;

// Parentheses tokens
LPAREN : '(' ;
RPAREN : ')' ;
LCURLY : '{' ;
RCURLY : '}' ;
LBRACK : '[' ;
RBRACK : ']' ;

// Operator tokens
EQUAL : '=' ;
ASSIGN : ':=' ;

PLUS : '+' ;
MINUS : '-' ;
STAR : '*' ;
SLASH : '/' ;
FLEX : '^' ;
DOTPLUS : '.+' ;
DOTMINUS : '.-' ;
DOTSTAR : '.*' ;
DOTSLASH : './' ;
DOTFLEX : '.^' ;

GRE : '>' ;
GEQ : '>=' ;
LES : '<' ;
LEQ : '<=' ;
NEQ : '<>' ;
EEQ : '==' ;

NOT : 'not' ;
AND : 'and' ;
OR : 'or' ;

// Control flow tokens
IN : 'in' ;
FOR : 'for' ;
IF : 'if' ;
ELSE : 'else' ;
ELSEIF : 'elseif' ;
THEN : 'then' ;
WHEN : 'when' ;
ELSEWHEN : 'elsewhen' ;
WHILE : 'while' ;
LOOP : 'loop' ;
BREAK : 'break' ;
RETURN : 'return' ;

// Class tokens
PARTIAL : 'partial' ;
OPERATOR : 'operator' ;
EXPANDABLE : 'expandable' ;
CLASS : 'class' ;
MODEL : 'model' ;
FUNCTION : 'function' ;
RECORD : 'record' ;
TYPE : 'type' ;
BLOCK : 'block' ;
CONNECTOR : 'connector' ;
PACKAGE : 'package' ;
PURE : 'pure' ;
IMPURE : 'impure' ;

// Keyword tokens
END : 'end' ;
DER : 'der' ;
CONNECT : 'connect' ;
INITIAL : 'initial' ;
EQUATION : 'equation' ;
ALGORITHM : 'algorithm' ;
WITHIN : 'within' ;
FINAL : 'final' ;
ENCAPSULATED : 'encapsulated' ;
EXTENDS : 'extends' ;
IMPORT : 'import' ;
ENUMERATION : 'enumeration' ;
INPUT : 'input' ;
OUTPUT : 'output' ;
PUBLIC : 'public' ;
PROTECTED : 'protected' ;
REDECLARE : 'redeclare' ;
INNER : 'inner' ;
OUTER : 'outer' ;
REPLACEABLE : 'replaceable' ;
CONSTRAINEDBY : 'constrainedby' ;
FLOW : 'flow' ;
STREAM : 'stream' ;
DISCRETE : 'discrete' ;
PARAMETER : 'parameter' ;
CONSTANT : 'constant' ;
EACH : 'each' ;
ANNOTATION : 'annotation' ;
EXTERNAL : 'external' ;

// Comment tokens
BLOCK_COMMENT : '/*' .*? '*/' -> channel (COMMENTS) ;
LINE_COMMENT : '//' ~[\r\n]* -> channel (COMMENTS) ;

// Fragment tokens
fragment S_ESCAPE : '\\' ('\'' | '"' | '?' | '\\' | 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v') ;
fragment DIGIT : [0-9] ;
fragment NONDIGIT : [a-zA-Z_] ;
fragment Q_CHAR
    : NONDIGIT
    | DIGIT
    | '!' | '#' | '$' | '%' | '&'
    | '(' | ')'
    | '*' | '+' | ',' | '-' | '.' | '/'
    | ':' | ';'
    | '<' | '>' | '='
    | '?' | '@'
    | '[' | ']'
    | '^'
    | '{' | '}'
    | '|' | '~'
    ;
fragment Q_IDENT : '\'' (Q_CHAR | S_ESCAPE)* '\'' ;
fragment S_CHAR : ~ ["\\] ;

fragment E : 'e' | 'E' ;
fragment UINT : DIGIT+ ;
fragment UREAL : UINT (DOT (UINT)?)? (E (PLUS | MINUS)? UINT)? ;

// Literal tokens
STRING : DQUOTE ( S_CHAR | S_ESCAPE )* DQUOTE ;
UNUM : UREAL | UINT ;
BOOL : 'true' | 'false' ;

IDENT : NONDIGIT (DIGIT | NONDIGIT)* | Q_IDENT ;
