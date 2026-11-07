"""ryo3-brotli ~ types"""

import sys

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

def brotli_decompress(data: Buffer, /) -> Bytes: ...
@deprecated(
    "`brotli_decode` is deprecated; use `brotli_decompress` instead [removal: v0.0.109]"
)
def brotli_decode(data: Buffer) -> Bytes: ...
