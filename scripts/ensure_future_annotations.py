import sys
from pathlib import Path
from subprocess import run

from ry import which

echo = print
isort_path = which("isort")
if not isort_path:
    echo("plz install isort - `pip install isort`")
    sys.exit(1)

PWD = Path(__file__).parent.resolve()
REPO_ROOT = PWD.parent

FROM_FUTURE_IMPORT_ANNOTATIONS = "from __future__ import annotations"


def file_string_is_empty(string: str) -> bool:
    """returns true if file is empty only contains whitespace or newlines"""
    return not string.strip(" \t\n\r")


for file in REPO_ROOT.joinpath("tests").resolve().glob("**/*.py"):
    echo("=" * 80)
    echo(file)
    with open(
        file,
        encoding="utf-8",
    ) as f:
        string = f.read()
    has_future_annotations = FROM_FUTURE_IMPORT_ANNOTATIONS in string
    file_is_empty = file_string_is_empty(string)
    if not has_future_annotations and not file_is_empty:
        echo("Adding import to file" + str(file))
        res = run(
            [isort_path, "-a", FROM_FUTURE_IMPORT_ANNOTATIONS, str(file)], check=True
        )
