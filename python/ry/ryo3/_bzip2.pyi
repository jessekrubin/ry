"""ryo3-bzip2 ~ types (DEPRECATED ~ use the stdlib `bz2`)"""

import sys
import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

_Bzip2Quality: t.TypeAlias = t.Literal[1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"]

@deprecated(
    "`bzip2_decode` is deprecated; use `bz2.decompress` instead [removal: v0.0.109]"
)
def bzip2_decode(data: Buffer) -> Bytes: ...
@deprecated(
    "`bzip2_encode` is deprecated; use `bz2.compress` instead [removal: v0.0.109]"
)
def bzip2_encode(data: Buffer, quality: _Bzip2Quality = 6) -> Bytes: ...
@deprecated("`bzip2` is deprecated; use `bz2.compress` instead [removal: v0.0.109]")
def bzip2(data: Buffer, quality: _Bzip2Quality = 6) -> Bytes: ...
