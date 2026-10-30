"""ryo3-flate2 ~ types"""

import sys
import typing as t

from ry import Bytes
from ry._types import Buffer

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

_GzipLevel: t.TypeAlias = t.Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"]

def gzip_compress(data: Buffer, /, level: _GzipLevel = 6) -> Bytes: ...
def gzip_decompress(data: Buffer, /) -> Bytes: ...
def is_gzip(data: Buffer, /) -> bool: ...

# ==== DEPRECATED ~ remove in v0.0.109 ====
@deprecated(
    "`gzip_encode` is deprecated; use `gzip_compress` instead [removal: v0.0.109]"
)
def gzip_encode(data: Buffer, quality: _GzipLevel = 6) -> Bytes: ...
@deprecated(
    "`gzip_decode` is deprecated; use `gzip_decompress` instead [removal: v0.0.109]"
)
def gzip_decode(data: Buffer) -> Bytes: ...
@deprecated("`gzip` is deprecated; use `gzip_compress` instead [removal: v0.0.109]")
def gzip(data: Buffer, quality: _GzipLevel = 6) -> Bytes: ...
@deprecated("`gunzip` is deprecated; use `gzip_decompress` instead [removal: v0.0.109]")
def gunzip(data: Buffer) -> Bytes: ...
@deprecated("`is_gzipped` is deprecated; use `is_gzip` instead [removal: v0.0.109]")
def is_gzipped(data: Buffer) -> bool: ...
