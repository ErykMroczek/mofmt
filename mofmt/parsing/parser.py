"""Classes used for parsing."""
# In future this code should be replaced by custom parser.

import antlr4 as antlr
from antlr4.Lexer import Lexer

from mofmt.collecting.collector import Collector

from .generated import Modelica, ModelicaLexer, ModelicaListener

NO_SPACE_BEFORE = (
    ModelicaLexer.RPAREN,
    ModelicaLexer.RBRACK,
    ModelicaLexer.RCURLY,
    ModelicaLexer.SEMICOLON,
    ModelicaLexer.COMMA,
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
        self.rule_stack = [0]
        self.group_stack: list[bool] = [False]
        self.wrap_stack: list[bool] = [False]
        # Number of unclosed brackets
        self.bracket_counter: int = 0

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
                self.collector.add_break()
            else:
                if self.prev_token == ModelicaLexer.SEMICOLON:
                    self.collector.add_blank()
            self.collector.add_comment(comment.text)
            line = comment.line
        if self.prev_token_line == 1:
            self.collector.add_break()
            return
        if len(tail) > 0:
            self.collector.append(tail)
            return
        line_diff = current_line - line
        if line_diff == 1:
            self.collector.add_break()
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
            if self.bracket_counter == 0:
                self.collector.add_break()
            else:
                self.collector.add_space()
            if not comments:
                if line - self.prev_token_line > 1 and kind not in NO_BREAK_BEFORE:
                    self.collector.add_blank()
        if comments:
            self.handle_comments(comments, line)

        # Handle special cases
        if kind == ModelicaLexer.LBRACK:
            self.bracket_counter += 1
            if self.prev_token != ModelicaLexer.IDENT:
                self.collector.add_space()
        elif kind == ModelicaLexer.RBRACK:
            self.bracket_counter -= 1
        elif kind == ModelicaLexer.FOR:
            self.break_or_space()
        elif kind == ModelicaLexer.DOT:
            # Only first dot in type specifiers etc. can be preceded with a space
            if self.prev_token not in (ModelicaLexer.IDENT, ModelicaLexer.RBRACK):
                self.collector.add_space()
        elif kind not in NO_SPACE_BEFORE and self.prev_token not in NO_SPACE_AFTER:
            self.collector.add_space()

        self.collector.add_token(token.text)
        if kind == ModelicaLexer.ANNOTATION:
            self.collector.add_space()
        self.prev_token = kind
        self.prev_token_line = line

    def enter_grouped_rule(self, ctx: antlr.ParserRuleContext) -> None:
        """If the rule was wrapped add info to the stack and increase indent"""
        self.group_stack.append(False)
        if is_multiline(ctx):
            self.group_stack[-1] = True
            if ctx.getRuleIndex() == Modelica.RULE_if_expression:
                if get_preceding_token(ctx, self.stream).type in (
                    ModelicaLexer.EQUAL,
                    ModelicaLexer.ASSIGN,
                ):
                    self.collector.add_indent()
            elif ctx.getRuleIndex() == Modelica.RULE_expression_list:
                if not self.group_stack[-2]:
                    self.collector.add_indent()
            else:
                self.collector.add_indent()

    def exit_grouped_rule(self, ctx: antlr.ParserRuleContext) -> None:
        """Decrease indent when leaving wrapped group"""
        if self.group_stack[-1]:
            if ctx.getRuleIndex() == Modelica.RULE_if_expression:
                if get_preceding_token(ctx, self.stream).type in (
                    ModelicaLexer.EQUAL,
                    ModelicaLexer.ASSIGN,
                ):
                    self.collector.add_dedent()
            elif ctx.getRuleIndex() == Modelica.RULE_expression_list:
                if not self.group_stack[-2]:
                    self.collector.add_dedent()
            else:
                self.collector.add_dedent()
        self.group_stack.pop()

    def break_or_space(self):
        """Insert line break or space"""
        if self.group_stack[-1]:
            self.collector.add_break()
        else:
            if self.prev_token not in NO_SPACE_AFTER:
                self.collector.add_space()

    def wrap_expression(self, ctx: antlr.ParserRuleContext):
        """Wrap the expression"""
        next_token = get_following_token(ctx, self.stream)
        # Check if there was a line break around the wrap point
        if next_token.line > self.prev_token_line:
            # Exclude unary expressions
            if ctx.parentCtx.getRuleIndex() != Modelica.RULE_unary_expression:
                self.collector.add_indent()
                self.collector.add_wrappoint()
                self.wrap_stack[-1] = True

    def enterEveryRule(self, ctx: antlr.ParserRuleContext) -> None:
        """
        Generic method called by Antlr listener every time it enters a
        grammar rule.
        """
        if len(ctx.getText()) == 0:
            return
        rule = ctx.getRuleIndex()
        self.rule_stack.append(rule)
        if rule == Modelica.RULE_description_string:
            self.collector.add_indent()
            self.collector.add_break()
        elif rule == Modelica.RULE_annotation:
            self.collector.add_indent()
            self.collector.add_break()
        elif rule == Modelica.RULE_constraining_clause:
            self.collector.add_indent()
            self.collector.add_break()
        elif rule == Modelica.RULE_conditional_equations:
            self.collector.add_indent()
            self.collector.add_break()
        elif rule == Modelica.RULE_conditional_statements:
            self.collector.add_indent()
            self.collector.add_break()
        elif rule == Modelica.RULE_enumeration_literal:
            self.collector.add_break()
        elif rule == Modelica.RULE_elseif_branch:
            self.collector.add_break()
        elif rule == Modelica.RULE_else_branch:
            self.collector.add_break()
        elif rule == Modelica.RULE_elsewhen_branch:
            self.collector.add_break()
        elif rule == Modelica.RULE_element_list:
            self.collector.add_indent()
            self.collector.add_blank()
        elif rule == Modelica.RULE_external_element:
            self.collector.add_indent()
            self.collector.add_blank()
        elif rule == Modelica.RULE_equation_list:
            self.collector.add_indent()
            self.collector.add_blank()
        elif rule == Modelica.RULE_statement_list:
            self.collector.add_indent()
            self.collector.add_blank()
        elif rule == Modelica.RULE_class_annotation:
            self.collector.add_indent()
            self.collector.add_blank()
        elif rule == Modelica.RULE_enum_list:
            self.collector.add_indent()
        elif rule == Modelica.RULE_conditional_expression:
            self.collector.add_indent()
            self.break_or_space()
        elif rule == Modelica.RULE_equation_section:
            self.collector.add_blank()
        elif rule == Modelica.RULE_algorithm_section:
            self.collector.add_blank()
        elif rule == Modelica.RULE_protected_element_list:
            self.collector.add_blank()
        elif rule == Modelica.RULE_public_element_list:
            self.collector.add_blank()
        elif rule == Modelica.RULE_end_clause:
            self.collector.add_blank()
        elif rule == Modelica.RULE_external_function_args:
            self.collector.add_ignore()
        elif rule == Modelica.RULE_enumerations:
            self.collector.add_ignore()
        elif rule == Modelica.RULE_unary_operand:
            self.collector.add_ignore()
        elif rule == Modelica.RULE_connected_components:
            self.collector.add_ignore()
        elif rule == Modelica.RULE_if_expression:
            self.enter_grouped_rule(ctx)
        elif rule == Modelica.RULE_primary:
            # Handle matrix or array
            if ctx.start.type in (ModelicaLexer.LBRACK, ModelicaLexer.LCURLY):
                self.enter_grouped_rule(ctx)
        elif rule == Modelica.RULE_function_call_args:
            self.enter_grouped_rule(ctx)
            self.collector.add_ignore()
        elif rule == Modelica.RULE_class_or_inheritance_modification:
            self.enter_grouped_rule(ctx)
            self.collector.add_ignore()
        elif rule == Modelica.RULE_class_modification:
            self.enter_grouped_rule(ctx)
            self.collector.add_ignore()
        elif rule == Modelica.RULE_array_subscripts:
            self.enter_grouped_rule(ctx)
            self.collector.add_ignore()
        elif rule == Modelica.RULE_expression_list:
            self.break_or_space()
            self.enter_grouped_rule(ctx)
        elif rule == Modelica.RULE_subscript:
            self.break_or_space()
        elif rule == Modelica.RULE_function_argument:
            # do not break if it is part of named arg
            if self.prev_token != ModelicaLexer.EQUAL:
                self.break_or_space()
        elif rule == Modelica.RULE_named_argument:
            self.break_or_space()
        elif rule == Modelica.RULE_argument:
            self.break_or_space()
        elif rule == Modelica.RULE_inheritance_modification:
            self.break_or_space()
        elif rule == Modelica.RULE_if_eval:
            self.break_or_space()
        elif rule == Modelica.RULE_elseif_eval:
            self.break_or_space()
        elif rule == Modelica.RULE_else_eval:
            self.break_or_space()
        elif rule == Modelica.RULE_conditional_expression:
            self.break_or_space()
        elif rule == Modelica.RULE_exp_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_mul_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_add_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_relational_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_cat_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_or_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_and_operator:
            self.wrap_expression(ctx)
        elif rule == Modelica.RULE_expression:
            # Handle arguments etc.
            if self.rule_stack[-2] in (
                Modelica.RULE_expression_list,
                Modelica.RULE_external_function_args,
                Modelica.RULE_array_arguments,
            ):
                self.break_or_space()
            self.wrap_stack.append(False)

    def exitEveryRule(self, ctx: antlr.ParserRuleContext) -> None:
        """
        Generic method called by Antlr listener every time it exits a
        grammar rule.
        """
        if len(ctx.getText()) == 0:
            return
        rule = self.rule_stack.pop()
        if rule == Modelica.RULE_description_string:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_annotation:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_constraining_clause:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_conditional_equations:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_conditional_statements:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_element_list:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_external_element:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_equation_list:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_statement_list:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_class_annotation:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_enum_list:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_conditional_expression:
            self.collector.add_dedent()
        elif rule == Modelica.RULE_if_expression:
            self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_primary:
            # Handle matrix
            if ctx.start.type in (ModelicaLexer.LBRACK, ModelicaLexer.LCURLY):
                self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_function_call_args:
            self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_class_or_inheritance_modification:
            self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_class_modification:
            self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_array_subscripts:
            self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_expression_list:
            self.exit_grouped_rule(ctx)
        elif rule == Modelica.RULE_expression:
            wrapped = self.wrap_stack.pop()
            if wrapped:
                self.collector.add_dedent()


# Helper functions


def is_multiline(ctx: antlr.ParserRuleContext) -> bool:
    """Return `True` if the rule is multiline"""
    # To satisfy the mypy
    result: bool = (ctx.stop.line - ctx.start.line) > 0
    return result


def get_preceding_token(
    ctx: antlr.ParserRuleContext, stream: antlr.CommonTokenStream
) -> antlr.Token:
    """Return token that precedes this rule"""
    prev_token_idx = stream.previousTokenOnChannel(
        ctx.start.tokenIndex - 1, Lexer.DEFAULT_TOKEN_CHANNEL
    )
    return stream.filterForChannel(
        prev_token_idx, prev_token_idx, Lexer.DEFAULT_TOKEN_CHANNEL
    )[0]


def get_following_token(
    ctx: antlr.ParserRuleContext, stream: antlr.CommonTokenStream
) -> antlr.Token:
    """Return token that follows this rule"""
    next_token_id = stream.nextTokenOnChannel(
        ctx.stop.tokenIndex + 1, Lexer.DEFAULT_TOKEN_CHANNEL
    )
    return stream.filterForChannel(
        next_token_id, next_token_id + 1, Lexer.DEFAULT_TOKEN_CHANNEL
    )[0]
