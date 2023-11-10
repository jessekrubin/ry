from __future__ import annotations

import json
import os
import sys
from typing import Dict, Union

from ry import _ry as libry
from ry.__about__ import __pkgroot__, __title__, __version__


def _nbytes_str(nbytes: Union[int, float]) -> str:
    """Format nbytesber of bytes to human readable form

    Ripped from `fmts` library which I wrote...

    ref: https://github.com/dynamic-graphics-inc/dgpy-libs/blob/main/libs/fmts/README.md

    Args:
        nbytes: number of bytes

    Returns:
        str: nbytesber of bytes formatted

    Raises:
        ValueError: If given number of bytes is invalid/negative

    Examples:
        >>> _nbytes_str(100)
        '100.0 bytes'
        >>> _nbytes_str(1000)
        '1000.0 bytes'
        >>> _nbytes_str(10000)
        '9.8 KB'
        >>> _nbytes_str(100000)
        '97.7 KB'
        >>> _nbytes_str(1000000)
        '976.6 KB'
        >>> _nbytes_str(10_000_000)
        '9.5 MB'
        >>> _nbytes_str(100_000_000)
        '95.4 MB'
        >>> _nbytes_str(1000000000)
        '953.7 MB'
        >>> _nbytes_str(10000000000)
        '9.3 GB'
        >>> _nbytes_str(100000000000)
        '93.1 GB'
        >>> _nbytes_str(1000000000000)
        '931.3 GB'
        >>> _nbytes_str(10000000000000)
        '9.1 TB'
        >>> _nbytes_str(100000000000000)
        '90.9 TB'

    """
    for x in ["bytes", "KB", "MB", "GB", "TB"]:
        if nbytes < 1024.0 or x == "TB":
            _str = f"{nbytes:3.1f} {x}"
            return _str
        nbytes /= 1024.0
    msg = f"Invalid number of bytes: {nbytes}"
    raise ValueError(msg)  # pragma: no cover


def _ext_info() -> Dict[str, Union[str, int]]:
    size = os.path.getsize(libry.__file__)
    return {
        "abspath": os.path.abspath(libry.__file__),
        "fsize": size,
        "fsize_str": _nbytes_str(size),
    }


def main() -> None:
    """Print package metadata"""

    sys.stdout.write(
        json.dumps(
            {
                "package": __title__,
                "version": __version__,
                "pkgroot": __pkgroot__,
                "libry": _ext_info(),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    if sys.argv[-1].endswith("__main__.py"):
        main()
