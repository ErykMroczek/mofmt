"""Helper functions used to parse source code"""

from typing import Callable

import antlr4 as antlr

from mofmt.collecting.collector import Marker

from .generated import Modelica, ModelicaLexer
from .parser import Listener


def parse_source(
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
    listener = Listener(stream)
    walker = antlr.ParseTreeWalker()
    walker.walk(listener, entry_rule(parser))
    # Append empty line
    listener.collector.add_linebreak()
    return listener.collector.markers
