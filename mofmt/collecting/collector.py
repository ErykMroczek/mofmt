"""Classes used for collecting printing markers"""

import json


class Marker:
    """
    Represents single printing marker (token, space, newline etc.).

    Attributes
    ----------
    typ : int
        Type of marker.
    val : str
        String value of marker. Used for printing.
    rep : str
        String representation. Mostly for debugging purposes.
    """

    # Integers indicate type
    TOKEN = 0
    COMMENT = 1
    SPACE = 2
    INDENT = 3
    DEDENT = 4
    IGNORE = 5
    BLANK = 6
    LINEBREAK = 7
    WRAPPOINT = 8

    def __init__(self, typ: int, val: str, rep: str) -> None:
        self.typ: int = typ
        self.val: str = val
        self.rep: str = rep

    def __repr__(self) -> str:
        return self.rep

    def __str__(self) -> str:
        return self.val


class Collector:
    """Represents collector that gathers formatting markers"""

    def __init__(self) -> None:
        self.list: list[Marker] = []
        self.wrapped = False
        self.last = 0

    def add_marker(self, marker: Marker) -> None:
        """Add marker"""
        self.list.append(marker)

    def add_token(self, val: str) -> None:
        """Add a token marker"""
        self.add_marker(Marker(Marker.TOKEN, val, val))
        self.last = len(self.list) - 1

    def add_comment(self, val: str) -> None:
        """Add a comment marker"""
        self.add_marker(Marker(Marker.COMMENT, val, val))
        self.last = len(self.list) - 1

    def cache_tail(self) -> list:
        """Return last few markers that are not tokens or comments"""
        tail = []
        while len(self.list) > 0:
            if self.list[-1].typ <= Marker.COMMENT:
                break
            last = self.list.pop()
            if last.typ != Marker.SPACE:
                tail.append(last)
        tail.reverse()
        return tail

    def append(self, markers: list[Marker]) -> None:
        """Append cached markers"""
        self.list.extend(markers)

    def add_space(self) -> None:
        """Add a space marker"""
        if len(self.list) == 0:
            return
        if self.list[-1].typ >= Marker.IGNORE:
            return
        self.add_marker(Marker(Marker.SPACE, " ", "SPACE"))

    def add_ignore(self) -> None:
        """Add a ignore space marker"""
        self.add_marker(Marker(Marker.IGNORE, "", "IGNORE"))

    def add_blank(self) -> None:
        """Add a blank marker"""
        if self.list[-1].typ >= Marker.BLANK:
            self.list.pop()
        if self.wrapped:
            self.add_dedent()
            self.wrapped = False
        self.add_marker(Marker(Marker.BLANK, "\n\n", "BLANK"))

    def add_linebreak(self) -> None:
        """Add a linebreak marker"""
        if self.list[-1].typ >= Marker.BLANK:
            return
        if self.wrapped:
            self.add_dedent()
            self.wrapped = False
        self.add_marker(Marker(Marker.LINEBREAK, "\n", "LINEBREAK"))

    def add_wrappoint(self) -> None:
        """Add a soft break marker"""
        if not self.wrapped:
            self.add_indent()
            self.wrapped = True
        self.add_marker(Marker(Marker.WRAPPOINT, "\n", "WRAP"))

    def add_indent(self) -> None:
        """Increase indentation before next marker"""
        self.add_marker(Marker(Marker.INDENT, "", "INDENT"))

    def add_dedent(self) -> None:
        """Decrease indentation before next marker"""
        self.add_marker(Marker(Marker.DEDENT, "", "DEDENT"))

    def __repr__(self) -> str:
        return [n.rep for n in self.list].__repr__()

    def __str__(self) -> str:
        return json.dumps(
            self.__repr__(),
            indent=2,
        )
