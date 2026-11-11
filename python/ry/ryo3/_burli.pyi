"""ryo3-burli ~ types"""

import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

_BrotliLevel: t.TypeAlias = t.Literal[0, 1, 2, 3, 4, 5]

def brotli_compress(data: Buffer, /, level: _BrotliLevel = 5) -> Bytes: ...
