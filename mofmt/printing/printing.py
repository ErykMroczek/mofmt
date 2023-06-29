"""Functions used for pretty printing Modelica code"""

from mofmt.collecting.collector import Marker


class Printer:
    """Class used for markers printing"""

    INDENT = "  "
    __slots__ = ("lvl", "markers", "printable")

    def __init__(self, markers: list[Marker]) -> None:
        self.lvl: int = 0
        self.markers = markers
        self.printable: list[str] = []

    def print_marker(self, marker: Marker) -> str:
        """
        Return marker converted to string and update indentation level.

        Parameters
        ----------
        marker : Marker
            Single marker produced in collector

        Returns
        -------
        str
            String produced from marker
        """
        typ = marker.typ
        if typ == Marker.INDENT:
            self.lvl += 1
            return marker.val
        if typ == Marker.DEDENT:
            self.lvl -= 1
            return marker.val
        if typ >= Marker.BLANK:
            marker.val += self.lvl * Printer.INDENT
        return marker.val

    def pretty_print(self) -> str:
        """
        Return formatted Modelica code as a string.

        Returns
        -------
        str
            Pretty Modelica code
        """
        self.printable = list(map(self.print_marker, self.markers))
        return "".join(self.printable)
