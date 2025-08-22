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
    for filepath in ry.walkdir(dir2walk):
        print(filepath)

    _print_br("Walking the directory tree with entries")
    # Walking the directory tree
    for direntry in ry.walkdir(dir2walk, objects=True):
        print(direntry, type(direntry))

    _print_br("Walking the directory tree with depth 1")
    # walking only files
    for filepath in ry.walkdir(dir2walk, dirs=False):
        print(filepath)
        assert ry.FsPath(filepath).is_file()

    # walking only directories
    for filepath in ry.walkdir(dir2walk, files=False):
        print(filepath)
        assert ry.FsPath(filepath).is_dir()

    # globset/globster
    for filepath in ry.walkdir(
        dir2walk,
        glob=ry.globster([
            "*.py",
        ]),
    ):
        assert filepath.endswith(".py")


if __name__ == "__main__":
    main()
