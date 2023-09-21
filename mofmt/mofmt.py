import sys
from itertools import chain
from pathlib import Path

from mofmt.io import get_files_from_dir, read_file, write_file
from mofmt.parsing import parse_source
from mofmt.printing import Printer


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
        parsed = parse_source(file, contents)
        if parsed:
            fmt = Printer(parsed).pretty_print()
            write_file(file, fmt)
        else:
            print(
                f"errors met while parsing {file}. Formatter will not modify it",
                file=sys.stderr,
            )


if __name__ == "__main__":
    main()
