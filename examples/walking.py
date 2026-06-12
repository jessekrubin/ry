"""
ry.walkdir example
"""

from __future__ import annotations

import os

import ry

PWD = os.path.dirname(os.path.abspath(__file__))


def _print_br(s: str | None = None) -> None:
    print("_" * 79)
    if s:
        print(s)


def main() -> None:
    dir2walk = PWD

    _print_br("Walking the directory tree")
    # Walking the directory tree
    for direntry in ry.walkdir(dir2walk):
        print(direntry, type(direntry))

    _print_br("Only files")
    # walking only files
    for filepath in ry.walkdir(dir2walk, dirs=False):
        print(filepath)
        assert ry.FsPath(filepath).is_file()

    _print_br("Only directories")
    # walking only directories
    for filepath in ry.walkdir(dir2walk, files=False):
        print(filepath)
        assert ry.FsPath(filepath).is_dir()

    _print_br("Glob pattern")
    # globset/globster
    for filepath in ry.walkdir(
        dir2walk,
        glob=ry.globster([
            "*.py",
        ]),
    ):
        assert str(filepath).endswith(".py")


if __name__ == "__main__":
    main()
