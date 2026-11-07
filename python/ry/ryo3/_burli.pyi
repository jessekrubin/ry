"""ryo3-burli ~ types"""

import sys
import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

_BrotliLevel: t.TypeAlias = t.Literal[0, 1, 2, 3, 4, 5]

def brotli_compress(data: Buffer, /, level: _BrotliLevel = 5) -> Bytes: ...
@deprecated(
    "`brotli_encode` is deprecated; use `brotli_compress` instead [removal: v0.0.109]"
)
def brotli_encode(data: Buffer, quality: _BrotliLevel = 5) -> Bytes: ...
@deprecated("`brotli` is deprecated; use `brotli_compress` instead [removal: v0.0.109]")
def brotli(data: Buffer, quality: _BrotliLevel = 5) -> Bytes: ...
