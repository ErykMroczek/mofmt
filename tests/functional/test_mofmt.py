"""
Module with end-to-end test of mofmt. Those tests are not concerned too
much with formatting style.
"""

from pathlib import Path

import pytest

from mofmt.io.io import NotModelicaFileError
from mofmt.mofmt import format_files

INPUT_CODE = """within Lib;
package Sources
"Source and boundary nodes"
extends Modelica.Icons.SourcesPackage;
end Sources;
"""

EXPECTED_CODE = """within Lib;
package Sources
  "Source and boundary nodes"

  extends Modelica.Icons.SourcesPackage;

end Sources;
"""


def test_files(tmp_path):
    """Check if mofmt handles proper Modelica files"""
    d: Path = tmp_path
    n_files = 3
    files = []
    for i in range(n_files):
        p = Path(d, f"file_{i}.mo")
        p.write_text(INPUT_CODE, "utf-8")
        files.append(p.as_posix())
    format_files(["mofmt"] + files)
    i = 0
    for f in files:
        i += 1
        code = Path(f).read_text("utf-8")
        assert code == EXPECTED_CODE
    assert i == n_files


def test_directories(tmp_path):
    """Check if mofmt handles directories of Modelica files"""
    d: Path = tmp_path
    n_dirs = 3
    n_files = 2
    dirs = []
    files = []
    for i in range(n_dirs):
        p = Path(d, f"directory_{i}.mo")
        p.mkdir()
        for j in range(n_files):
            f = Path(p, f"file_{j}.mo")
            f.write_text(INPUT_CODE, "utf-8")
            files.append(f)
        dirs.append(p.as_posix())
    format_files(["mofmt"] + dirs)
    i = 0
    for f in files:
        i += 1
        code = f.read_text("utf-8")
        assert code == EXPECTED_CODE
    assert i == n_files * n_dirs


def test_recursive_directories(tmp_path):
    """Check if mofmt handles recursive Modelica directories"""
    d: Path = tmp_path
    n_files = 2
    files = []
    for i in range(n_files):
        p = Path(d, f"file_{i}.mo")
        p.write_text(INPUT_CODE, "utf-8")
        files.append(p.as_posix())
    d_recursive = Path(d, "dir")
    d_recursive.mkdir()
    for i in range(n_files):
        p = Path(d_recursive, f"file_{i}.mo")
        p.write_text(INPUT_CODE, "utf-8")
        files.append(p.as_posix())
    format_files(["mofmt", d.as_posix()])
    i = 0
    for f in files:
        i += 1
        code = Path(f).read_text("utf-8")
        assert code == EXPECTED_CODE
    assert i == n_files * 2


def test_errors(tmp_path):
    """Check error handling"""
    # Check file extension reckognition
    p = Path(tmp_path, "file.txt")
    p.write_text(INPUT_CODE, "utf-8")
    with pytest.raises(NotModelicaFileError):
        format_files(["mofmt", p.as_posix()])
