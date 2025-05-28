from __future__ import annotations

import json
import os
import sys

from ry import ryo3
from ry.__about__ import __authors__, __pkgroot__, __target__, __title__, __version__


def _ext_info() -> dict[str, str | int]:
    size = os.path.getsize(ryo3.__file__)
    return {
        "abspath": os.path.abspath(ryo3.__file__),
        "fsize": size,
        "fsize_str": ryo3.fmt_size(size),
        "build_profile": ryo3.__build_profile__,
        "build_timestamp": ryo3.__build_timestamp__,
        "target": __target__,
    }


def _lib_info() -> dict[str, str | int | dict[str, str | int]]:
    return {
        "package": __title__,
        "version": __version__,
        "pkgroot": __pkgroot__,
        "authors": __authors__,
        "ryo3": _ext_info(),
    }


def main() -> None:
    """Print package metadata"""
    json_out = json.dumps(_lib_info(), indent=2)
    sys.stdout.write(f"{json_out}\n")


if __name__ == "__main__":
    if sys.argv[-1].endswith("__main__.py"):
        main()
