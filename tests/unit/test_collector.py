"""Unit tests for Collector class functionalities"""

from mofmt.collecting.collector import Collector, Marker


def test_tokens():
    """Check tokens collecting"""
    col = Collector()
    val_1, val_2 = "TEST1", "TEST2"
    col.add_token(val_1)
    col.add_token(val_2)
    # Test if number of markers is correct
    assert len(col.list) == 2
    # Test if markers has a correct type
    for m in col.list:
        assert m.typ == Marker.TOKEN
    # Test markers values and representations
    assert col.list[0].val == val_1
    assert col.list[-1].val == val_2
    assert col.list[0].val == col.list[0].rep
