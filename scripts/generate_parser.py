"""Script for parser generationa after grammar updates"""

import subprocess


def update_parser() -> None:
    """Generate new parser"""
    subprocess.run(
        [
            "antlr4",
            "-v",
            "4.12.0",
            "-Dlanguage=Python3",
            "-o",
            "mofmt/parsing/generated",
            "grammar/ModelicaLexer.g4",
            "grammar/Modelica.g4",
        ]
    )
    print("Parser updated")


if __name__ == "__main__":
    update_parser()
