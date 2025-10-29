from __future__ import annotations

import os
import sys

from ry import ryo3
from ry.__about__ import (
    __allocator__,
    __authors__,
    __build_profile__,
    __build_timestamp__,
    __opt_level__,
    __pkgroot__,
    __target__,
    __title__,
    __version__,
)


def _ext_info() -> dict[str, str | int]:
    size = os.path.getsize(ryo3.__file__)
    return {
        "abspath": os.path.abspath(ryo3.__file__),
        "allocator": __allocator__,
        "build_profile": __build_profile__,
        "build_timestamp": __build_timestamp__,
        "fsize": size,
        "fsize_str": ryo3.fmt_size(size),
        "opt-level": __opt_level__,
        "target": __target__,
    }


def _lib_info() -> dict[str, str | int | dict[str, str | int]]:
    return {
        "package": __title__,
        "version": __version__,
        "authors": __authors__,
        "pkgroot": __pkgroot__,
        "ryo3": _ext_info(),
    }


def main() -> None:
    """Print package metadata"""
    json_out = ryo3.stringify(_lib_info(), fmt=True, pybytes=True, append_newline=True)
    sys.stdout.buffer.write(json_out)


if __name__ == "__main__":
    if sys.argv[-1].endswith("__main__.py"):
        main()
