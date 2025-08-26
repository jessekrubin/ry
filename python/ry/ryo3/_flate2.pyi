"""ryo3-flate2 types"""

from typing import Literal, TypeAlias

from ry import Bytes
from ry._types import Buffer

Quality: TypeAlias = Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"]

def gzip_encode(data: Buffer, quality: Quality = 6) -> Bytes: ...
def gzip_decode(data: Buffer) -> Bytes: ...
def gzip(data: Buffer, quality: Quality = 6) -> Bytes:
    """Alias for gzip_encode"""

def gunzip(data: Buffer) -> Bytes:
    """Alias for gzip_decode"""

def is_gzipped(data: Buffer) -> bool: ...
