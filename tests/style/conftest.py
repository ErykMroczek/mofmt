"""Pytest fixtures"""

from pathlib import Path

import pytest

from mofmt.io import read_file
from mofmt.parsing import parse_source
from mofmt.printing import Printer


@pytest.fixture(name="format_file", scope="function")
def format_file():
    """Fixture used for parsing and formatting code samples"""

    def _format(file, entry):
        code = read_file(Path(file))
        parsed = parse_source(code, entry)
        return Printer(parsed).pretty_print()

    return _format
