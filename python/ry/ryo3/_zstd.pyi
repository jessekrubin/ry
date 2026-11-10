"""ryo3-zstd ~ types"""

import typing as t

from ry import Bytes
from ry._types import Buffer

_ZstdLevel: t.TypeAlias = t.Literal[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
]

ZSTD_VERSION: str  # "1.5.7" (c. 2026-10-29)
ZSTD_VERSION_INFO: tuple[int, int, int]  # (1, 5, 7) (c. 2026-10-29)

def zstd_compress(data: Buffer, /, level: _ZstdLevel = 3) -> Bytes: ...
def zstd_decompress(data: Buffer, /) -> Bytes: ...
def is_zstd(data: Buffer, /) -> bool: ...
