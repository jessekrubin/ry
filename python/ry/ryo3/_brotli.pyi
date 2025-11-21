"""ryo3-brotli types"""

from typing import Literal, TypeAlias

from ry._types import Buffer

_Quality: TypeAlias = Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]

def brotli_encode(
    data: Buffer, quality: _Quality = 11, *, magic_number: bool = False
) -> bytes: ...
def brotli_decode(data: Buffer) -> bytes: ...
def brotli(
    data: Buffer, quality: _Quality = 11, *, magic_number: bool = False
) -> bytes:
    """Alias for brotli_encode"""
