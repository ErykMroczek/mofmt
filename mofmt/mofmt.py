import sys
from pathlib import Path

from mofmt.io import get_files_from_dir, read_file, write_file
from mofmt.parsing import parse_source
from mofmt.printing import pretty_print


class ParsingError(Exception):
    """Raised when parsing fails"""

    def __init__(self, f: Path) -> None:
        self.message = f"cannot parse {f}. Probably it not a valid modelica file"
        super().__init__(self.message)


def main():
    argv = sys.argv
    if len(argv) != 2:
        raise SystemExit("mofmt takes one argument (file/directory path)")
    p = Path(argv[1])
    if p.is_dir():
        modelica_files = get_files_from_dir(p)
    else:
        modelica_files = [p]
    for file in modelica_files:
        contents = read_file(file)
        try:
            parsed = parse_source(contents)
        except Exception as e:
            raise ParsingError(file) from e
        fmt = pretty_print(parsed)
        write_file(file, fmt)


if __name__ == "__main__":
    main()
