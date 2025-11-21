"""ryo3-bzip2 types"""

from typing import Literal, TypeAlias

from ry import Bytes
from ry._types import Buffer

_Quality: TypeAlias = Literal[1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"]

def bzip2_decode(data: Buffer) -> Bytes: ...
def bzip2_encode(data: Buffer, quality: _Quality = 6) -> Bytes: ...
def bzip2(data: Buffer, quality: _Quality = 6) -> Bytes:
    """Alias for bzip2_encode"""
