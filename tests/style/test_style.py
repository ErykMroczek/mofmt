"""Tests checking formatting style"""

from pathlib import Path

from mofmt.io import read_file
from mofmt.parsing.generated.Modelica import Modelica

from .conftest import format_file


def test_equations(format_file):
    """Check formatting in equations"""
    input_file = "tests/style/samples/equations/equations-input.mo"
    template_file = "tests/style/samples/equations/equations-output.mo"
    entry = Modelica.equation_list
    actual = format_file(input_file, entry)
    expected = read_file(Path(template_file))
    assert actual == expected


def test_elements(format_file):
    """Check formatting in elements"""
    input_file = "tests/style/samples/elements/elements-input.mo"
    template_file = "tests/style/samples/elements/elements-output.mo"
    entry = Modelica.element_list
    actual = format_file(input_file, entry)
    expected = read_file(Path(template_file))
    assert actual == expected


def test_classes(format_file):
    """Check formatting in classes"""
    input_file = "tests/style/samples/classes/classes-input.mo"
    template_file = "tests/style/samples/classes/classes-output.mo"
    entry = Modelica.stored_definition
    actual = format_file(input_file, entry)
    expected = read_file(Path(template_file))
    assert actual == expected
