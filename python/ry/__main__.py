from __future__ import annotations

import json
import os
import sys

from ry import _ry as libry
from ry.__about__ import __pkgroot__, __title__, __version__


def _ext_info() -> dict[str, str | int]:
    size = os.path.getsize(libry.__file__)
    return {
        "abspath": os.path.abspath(libry.__file__),
        "fsize": size,
        "fsize_str": libry.nbytes_str(size),
        "build_profile": libry.__build_profile__,
        "build_timestamp": libry.__build_timestamp__,
    }


def _lib_info() -> dict[str, str | int]:
    return {
        "package": __title__,
        "version": __version__,
        "pkgroot": __pkgroot__,
        "ry": _ext_info(),
    }


def main() -> None:
    """Print package metadata"""
    sys.stdout.write(json.dumps(_lib_info(), indent=2))


if __name__ == "__main__":
    if sys.argv[-1].endswith("__main__.py"):
        main()
