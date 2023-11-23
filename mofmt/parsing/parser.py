"""Classes used for parsing."""
# In future this code should be replaced by custom parser.

import antlr4 as antlr

from mofmt.collecting.collector import Collector

from .generated import Modelica, ModelicaLexer, ModelicaListener

GROUPS = (
    Modelica.RULE_class_modification,
    Modelica.RULE_function_call_args,
    Modelica.RULE_array,
    Modelica.RULE_matrix,
    Modelica.RULE_if_expression,
    Modelica.RULE_class_or_inheritance_modification,
    Modelica.RULE_array_subscripts,
    Modelica.RULE_expression_list,
)

INDENT_AT = (
    Modelica.RULE_description_string,
    Modelica.RULE_annotation,
    Modelica.RULE_constraining_clause,
    Modelica.RULE_element_list,
    Modelica.RULE_equation_list,
    Modelica.RULE_statement_list,
    Modelica.RULE_conditional_expression,
    Modelica.RULE_conditional_equations,
    Modelica.RULE_conditional_statements,
    Modelica.RULE_enum_list,
    Modelica.RULE_external_element,
    Modelica.RULE_class_annotation,
)

HARD_BREAKS_AT = (
    Modelica.RULE_description_string,
    Modelica.RULE_annotation,
    Modelica.RULE_constraining_clause,
    Modelica.RULE_conditional_equations,
    Modelica.RULE_conditional_statements,
    Modelica.RULE_enumeration_literal,
    Modelica.RULE_elseif_branch,
    Modelica.RULE_else_branch,
    Modelica.RULE_elsewhen_branch,
)

SOFT_BREAKS_AT = (
    Modelica.RULE_subscript,
    Modelica.RULE_function_argument,
    Modelica.RULE_named_argument,
    Modelica.RULE_argument,
    Modelica.RULE_inheritance_modification,
    Modelica.RULE_array_argument,
    Modelica.RULE_matrix_row,
    Modelica.RULE_if_eval,
    Modelica.RULE_elseif_eval,
    Modelica.RULE_else_eval,
    Modelica.RULE_conditional_expression,
    Modelica.RULE_for_initializer,
    Modelica.RULE_expression_list_member,
)

BLANK_BEFORE = (
    Modelica.RULE_equation_section,
    Modelica.RULE_algorithm_section,
    Modelica.RULE_protected_element_list,
    Modelica.RULE_public_element_list,
    Modelica.RULE_element_list,
    Modelica.RULE_equation_list,
    Modelica.RULE_statement_list,
    Modelica.RULE_external_element,
    Modelica.RULE_end_clause,
    Modelica.RULE_class_annotation,
)

WRAP_AT = (
    Modelica.RULE_exp_operator,
    Modelica.RULE_mul_operator,
    Modelica.RULE_add_operator,
    Modelica.RULE_relational_operator,
    Modelica.RULE_cat_operator,
    Modelica.RULE_or_operator,
    Modelica.RULE_and_operator,
)

IGNORE_AT = (
    Modelica.RULE_function_call_args,
    Modelica.RULE_external_function_args,
    Modelica.RULE_class_modification,
    Modelica.RULE_enumerations,
    Modelica.RULE_unary_operand,
    Modelica.RULE_class_or_inheritance_modification,
    Modelica.RULE_connected_components,
    Modelica.RULE_array_subscripts,
)

NO_SPACE_BEFORE = (
    ModelicaLexer.RPAREN,
    ModelicaLexer.RBRACK,
    ModelicaLexer.RCURLY,
    ModelicaLexer.SEMICOLON,
    ModelicaLexer.COMMA,
    ModelicaLexer.DOT,
    ModelicaLexer.COLON,
)

NO_SPACE_AFTER = (
    ModelicaLexer.LPAREN,
    ModelicaLexer.DOT,
    ModelicaLexer.LBRACK,
    ModelicaLexer.LCURLY,
    ModelicaLexer.SEMICOLON,
    ModelicaLexer.COLON,
)

NO_BREAK_BEFORE = (
    ModelicaLexer.END,
    ModelicaLexer.ELSE,
    ModelicaLexer.ELSEIF,
    ModelicaLexer.ELSEWHEN,
)


class Listener(ModelicaListener):  # type: ignore
    """Custom listener for parsing Modelica source"""

    def __init__(self, stream: antlr.CommonTokenStream) -> None:
        super().__init__()
        self.stream = stream
        self.collector = Collector()
        self.prev_token_line: int = 1
        self.prev_token: int = 0
        self.ignore_semi: bool = False
        self.group_stack: list[bool] = [False]
        self.group_precedent: list[int] = [0]
        self.wrap_stack: list[bool] = [False]

    def handle_comments(self, comments: list[antlr.Token], current_line: int) -> None:
        """
        Handle comments and separate them if needed.

        Parameters
        ----------
        comments : list
            List of comments that were originally located before token
        current_line : int
            Line where token occured
        """
        line = self.prev_token_line
        line_diff = comments[0].line - line
        tail = []
        if line_diff == 0:
            tail = self.collector.cache_tail()
        for comment in comments:
            line_diff = comment.line - line
            if line_diff == 0:
                self.collector.add_space()
            elif line_diff == 1:
                self.collector.add_hardbreak()
            else:
                if self.prev_token == ModelicaLexer.SEMICOLON:
                    self.collector.add_blank()
            self.collector.add_comment(comment.text)
            line = comment.line
        if self.prev_token_line == 1:
            self.collector.add_hardbreak()
            return
        if len(tail) > 0:
            self.collector.append(tail)
            return
        line_diff = current_line - line
        if line_diff == 1:
            self.collector.add_hardbreak()
            return
        if line_diff > 1:
            self.collector.add_blank()

    def visitTerminal(self, node: antlr.TerminalNode) -> None:
        """
        Generic method called by Antlr listener every time it finds
        terminal.
        """
        token: antlr.Token = node.getSymbol()
        kind = token.type
        line = token.line
        comments = self.stream.getHiddenTokensToLeft(
            token.tokenIndex, ModelicaLexer.COMMENTS
        )
        if self.prev_token == ModelicaLexer.SEMICOLON:
            if not self.ignore_semi:
                self.collector.add_hardbreak()
            else:
                self.ignore_semi = False
                self.collector.add_space()
            if not comments:
                if line - self.prev_token_line > 1 and kind not in NO_BREAK_BEFORE:
                    self.collector.add_blank()
        if comments:
            self.handle_comments(comments, line)
        if kind not in NO_SPACE_BEFORE and self.prev_token not in NO_SPACE_AFTER:
            self.collector.add_space()
        self.collector.add_token(token.text)
        if kind == ModelicaLexer.ANNOTATION:
            self.collector.add_space()
        self.prev_token = kind
        self.prev_token_line = line

    def enterEveryRule(self, ctx: antlr.ParserRuleContext) -> None:
        """
        Generic method called by Antlr listener every time it enters a
        grammar rule.
        """
        rule = ctx.getRuleIndex()
        if rule == Modelica.RULE_matrix_row:
            self.ignore_semi = True
        if rule in INDENT_AT:
            self.collector.add_indent()
        if rule in GROUPS:
            self.group_stack.append(False)
            self.group_precedent.append(self.prev_token)
            if ctx.stop.line - ctx.start.line > 0:
                self.group_stack[-1] = True
                if (
                    rule != Modelica.RULE_if_expression
                    or self.group_precedent[-1] == ModelicaLexer.EQUAL
                ):
                    self.collector.add_indent()
        if rule in WRAP_AT:
            token: antlr.Token = ctx.stop
            next_token_id = self.stream.nextTokenOnChannel(
                token.tokenIndex + 1, token.channel
            )
            next_token = self.stream.getTokens(next_token_id, next_token_id + 1)[0]
            if next_token.line > self.prev_token_line:
                if ctx.parentCtx.getRuleIndex() != Modelica.RULE_unary_expression:
                    self.collector.add_wrappoint()
                    self.wrap_stack[-1] = True
        if len(ctx.getText()) == 0:
            return
        if rule in HARD_BREAKS_AT:
            self.collector.add_hardbreak()
        if rule in SOFT_BREAKS_AT:
            if self.group_stack[-1]:
                if self.wrap_stack[-1]:
                    self.collector.add_softbreak()
                else:
                    self.collector.add_hardbreak()
        if rule in BLANK_BEFORE:
            self.collector.add_blank()
        if rule in IGNORE_AT:
            self.collector.add_ignore()

    def exitEveryRule(self, ctx: antlr.ParserRuleContext) -> None:
        """
        Generic method called by Antlr listener every time it exits a
        grammar rule.
        """
        rule = ctx.getRuleIndex()
        self.ignore_semi = False
        if rule in GROUPS:
            if self.group_stack[-1]:
                if (
                    rule != Modelica.RULE_if_expression
                    or self.group_precedent[-1] == ModelicaLexer.EQUAL
                ):
                    self.collector.add_dedent()
            self.group_stack.pop()
            self.group_precedent.pop()
        if rule in INDENT_AT:
            self.collector.add_dedent()

    def enterExpression(self, ctx: antlr.ParserRuleContext) -> None:
        self.wrap_stack.append(False)

    def exitExpression(self, ctx: antlr.ParserRuleContext) -> None:
        self.wrap_stack.pop()

    def enterType_specifier(self, ctx: antlr.ParserRuleContext):
        if ctx.start.type == ModelicaLexer.DOT:
            self.collector.add_space()
