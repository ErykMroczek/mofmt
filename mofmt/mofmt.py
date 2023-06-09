import sys
from itertools import chain
from pathlib import Path

from mofmt.io import get_files_from_dir, read_file, write_file
from mofmt.parsing import parse_source
from mofmt.printing import Printer


class ParsingError(Exception):
    """Raised when parsing fails"""

    def __init__(self, f: Path) -> None:
        self.message = f"cannot parse {f}. Probably it is not a valid modelica file"
        super().__init__(self.message)


def main() -> None:
    format_files(sys.argv)


def format_files(args: list[str]) -> None:
    """Format files specified in argument list"""
    if len(args) < 2:
        raise SystemExit("mofmt takes at least one argument (file/directory path)")
    paths = [Path(arg) for arg in args[1:]]
    modelica_files = list(
        chain.from_iterable(
            (get_files_from_dir(p) if p.is_dir() else [p] for p in paths)
        )
    )
    for file in modelica_files:
        contents = read_file(file)
        try:
            parsed = parse_source(contents)
        except Exception as e:
            raise ParsingError(file) from e
        fmt = Printer(parsed).pretty_print()
        write_file(file, fmt)


if __name__ == "__main__":
    main()
