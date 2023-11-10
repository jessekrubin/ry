from __future__ import annotations

import json
import os
import sys
from typing import Dict, Union

from ry import _ry as libry
from ry.__about__ import __pkgroot__, __title__, __version__

def _ext_info() -> Dict[str, Union[str, int]]:
    size = os.path.getsize(libry.__file__)
    return {
        "abspath": os.path.abspath(libry.__file__),
        "fsize": size,
        "fsize_str": libry.nbytes_str(size),
    }

def main() -> None:
    """Print package metadata"""

    sys.stdout.write(
        json.dumps(
            {
                "package": __title__,
                "version": __version__,
                "pkgroot": __pkgroot__,
                "ry": _ext_info(),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    if sys.argv[-1].endswith("__main__.py"):
        main()
