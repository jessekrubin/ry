"""ry-types"""

from __future__ import annotations

import sys
from os import PathLike
from typing import Literal, TypeAlias

if sys.version_info >= (3, 12):  # pragma: no cover
    from collections.abc import Buffer
    from typing import TypedDict, Unpack
else:  # pragma: no cover
    from typing_extensions import Buffer, TypedDict, Unpack

__all__ = (
    "Buffer",
    "FsPathLike",
    "TypedDict",
    "Unpack",
)
FsPathLike: TypeAlias = str | PathLike[str]
# =============================================================================
# OPEN MODES (CANONICAL)
# =============================================================================
# ry accepts the non-canonical modes, but they are mapped to the canonical ones]
# fmt: off
OpenTextModeUpdating: TypeAlias = Literal[
    "a+", "at+",
    "r+", "rt+",
    "w+", "wt+",
    "x+", "xt+"
]
OpenTextModeWriting: TypeAlias = Literal["a", "at", "w", "wt", "x", "xt"]
OpenTextModeReading: TypeAlias = Literal["r", "rt"]
OpenTextMode: TypeAlias = Literal[
    "a", "a+", "at", "at+",
    "r", "r+", "rt", "rt+",
    "w", "w+", "wt", "wt+",
    "x", "x+", "xt", "xt+"
]
OpenBinaryModeUpdating: TypeAlias = Literal["ab+", "rb+", "wb+", "xb+"]
OpenBinaryModeWriting: TypeAlias = Literal["ab", "wb", "xb"]
OpenBinaryModeReading: TypeAlias = Literal["rb"]
OpenBinaryMode: TypeAlias = Literal[
    "ab", "ab+",
    "rb", "rb+",
    "wb", "wb+",
    "xb", "xb+"
]
OpenMode: TypeAlias = Literal[
    "a", "a+", "ab", "ab+", "at", "at+",
    "r", "r+", "rb", "rb+", "rt", "rt+",
    "w", "w+", "wb", "wb+", "wt", "wt+",
    "x", "x+", "xb", "xb+", "xt", "xt+",
]
# fmt: on
