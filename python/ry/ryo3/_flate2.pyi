"""ryo3-flate2 ~ types"""

import typing as t

from ry import Bytes
from ry._types import Buffer

_GzipLevel: t.TypeAlias = t.Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"]

def gzip_compress(data: Buffer, /, level: _GzipLevel = 6) -> Bytes: ...
def gzip_decompress(data: Buffer, /) -> Bytes: ...
def is_gzip(data: Buffer, /) -> bool: ...
