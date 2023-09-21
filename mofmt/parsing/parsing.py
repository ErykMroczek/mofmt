"""Helper functions used to parse source code"""

import sys
from pathlib import Path
from typing import Callable

import antlr4 as antlr
from antlr4.error.ErrorListener import ConsoleErrorListener

from mofmt.collecting.collector import Marker

from .generated import Modelica, ModelicaLexer
from .parser import Listener


def parse_source(
    path: Path,
    source: str,
    entry_rule: Callable[
        [Modelica], antlr.ParserRuleContext
    ] = Modelica.stored_definition,
) -> list[Marker]:
    """
    Return list of printing markers generated from Modelica source code.

    Parameters
    ----------
    source : str
        Modelica source code

    Returns
    -------
    list
        List of printing markers
    """
    input_stream = antlr.InputStream(source)
    lexer = ModelicaLexer(input_stream)
    stream = antlr.CommonTokenStream(lexer)
    parser = Modelica(stream)
    handler = ErrorHandler(path)
    parser.removeErrorListeners()
    parser.addErrorListener(handler)
    listener = Listener(stream)
    walker = antlr.ParseTreeWalker()
    walker.walk(listener, entry_rule(parser))
    # Append empty line
    listener.collector.add_hardbreak()
    if parser._syntaxErrors > 0:
        listener.collector.markers = []
    return listener.collector.markers


class ErrorHandler(ConsoleErrorListener):
    def __init__(self, path) -> None:
        self.path = path
        super().__init__()

    def syntaxError(self, recognizer, offendingSymbol, line, column, msg, e):
        print(
            f"{self.path}:{line}:{column}: {msg}",
            file=sys.stderr,
        )
