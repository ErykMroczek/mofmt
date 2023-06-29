"""Unit tests for Collector class functionalities"""

from mofmt.collecting.collector import Collector, Marker


def test_tokens():
    """Check tokens collecting"""
    col = Collector()
    val_1, val_2 = "TOK1", "TOK2"
    col.add_token(val_1)
    col.add_token(val_2)
    # Test if number of markers is correct
    assert len(col.markers) == 2
    # Test if markers has a correct type
    for m in col.markers:
        assert m.typ == Marker.TOKEN
    # Test markers values and representations
    assert col.markers[0].val == val_1
    assert col.markers[-1].val == val_2
    assert col.markers[0].val == col.markers[0].rep


def test_comments():
    """Check comments collecting"""
    col = Collector()
    val_1, val_2 = "COM1", "COM2"
    col.add_comment(val_1)
    col.add_comment(val_2)
    # Test if number of markers is correct
    assert len(col.markers) == 2
    # Test if markers has a correct type
    for m in col.markers:
        assert m.typ == Marker.COMMENT
    # Test markers values and representations
    assert col.markers[0].val == val_1
    assert col.markers[-1].val == val_2
    assert col.markers[0].val == col.markers[0].rep


def test_tail_caching():
    """Check collector tail caching that is used in comments handling"""
    col = Collector()
    col.add_token("tok")
    col.add_space()
    col.add_indent()
    col.add_linebreak()
    col.add_space()
    tail = col.cache_tail()
    # There should be just indent and break
    assert len(tail) == 2
    assert tail[0].typ == Marker.INDENT
    assert tail[1].typ == Marker.LINEBREAK
    # Only single token should be left in collector
    assert len(col.markers) == 1
    assert col.markers[0].typ == Marker.TOKEN


def test_space():
    """Check space collecting"""
    col = Collector()
    # Initial spaces should be discarded
    col.add_space()
    col.add_space()
    assert len(col.markers) == 0
    # But should be accepted when following a token or comment
    col.add_token("tok")
    col.add_space()
    assert col.markers[-1].typ == Marker.SPACE
    # But they should be discarded while following ignore marker or
    # newline
    col.add_ignore()
    col.add_space()
    assert col.markers[-1].typ == Marker.IGNORE
    col.add_linebreak()
    col.add_space()
    assert col.markers[-1].typ == Marker.LINEBREAK


def test_blank():
    """Check blanks collecting"""
    col = Collector()
    col.add_token("tok")
    col.add_linebreak()
    col.wrapped = True
    col.add_blank()
    # Blank should overwrite previous newline marker and add dedent when
    # expression was wrapped
    assert len(col.markers) == 3
    assert col.markers[1].typ == Marker.DEDENT
