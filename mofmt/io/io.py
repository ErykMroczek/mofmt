"""Helper functions for file IO"""

from pathlib import Path


class NotModelicaFileError(Exception):
    """Custom exception raised when specified file is not a valid Modelica"""

    def __init__(self, file: Path) -> None:
        self.message = f"{file} is not a valid Modelica file"
        super().__init__(self.message)


def get_files_from_dir(directory: Path) -> list[Path]:
    """Return list of Modelica files inside directory"""
    return list(directory.rglob("*.mo"))


def read_file(path: Path) -> str:
    """Return file contents"""

    if not path.is_file():
        raise FileNotFoundError(f"file {path} not found")
    if path.suffix != ".mo":
        raise NotModelicaFileError(path)
    try:
        with path.open("r", encoding="utf-8") as file:
            contents = file.read()
    except UnicodeDecodeError as exc:
        raise UnicodeError(f"{path} is not a valid text file") from exc
    return contents


def write_file(path: Path, code: str) -> None:
    """Write code string to a file"""
    try:
        with path.open("w", encoding="utf-8") as file:
            file.write(code)
    except PermissionError as exc:
        raise PermissionError(f"Couldn't write to {path}") from exc
