"""ryo3-brotli types"""

from ry._types import Buffer

def brotli_encode(
    data: Buffer, quality: int = 11, magic_number: bool = False
) -> bytes: ...
def brotli_decode(data: Buffer) -> bytes: ...
def brotli(data: Buffer, quality: int = 11, magic_number: bool = False) -> bytes:
    """Alias for brotli_encode"""
