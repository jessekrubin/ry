from __future__ import annotations

import json
import os
import sys

from ry import _ry
from ry.__about__ import __authors__, __pkgroot__, __title__, __version__


def _ext_info() -> dict[str, str | int]:
    size = os.path.getsize(_ry.__file__)
    return {
        "abspath": os.path.abspath(_ry.__file__),
        "fsize": size,
        "fsize_str": _ry.fmt_nbytes(size),
        "build_profile": _ry.__build_profile__,
        "build_timestamp": _ry.__build_timestamp__,
    }


def _lib_info() -> dict[str, str | int | dict[str, str | int]]:
    return {
        "package": __title__,
        "version": __version__,
        "pkgroot": __pkgroot__,
        "authors": __authors__,
        "ry": _ext_info(),
    }


def main() -> None:
    """Print package metadata"""
    sys.stdout.write(json.dumps(_lib_info(), indent=2))


if __name__ == "__main__":
    if sys.argv[-1].endswith("__main__.py"):
        main()
