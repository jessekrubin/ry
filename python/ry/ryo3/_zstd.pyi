"""ry.ryo3 root level zstd exports"""

from ry.ryo3.zstd import compress as zstd_compress
from ry.ryo3.zstd import decode as zstd_decode
from ry.ryo3.zstd import decompress as zstd_decompress
from ry.ryo3.zstd import encode as zstd_encode
from ry.ryo3.zstd import is_zstd as is_zstd

__all__ = (
    "is_zstd",
    "zstd_compress",
    "zstd_decode",
    "zstd_decompress",
    "zstd_encode",
)
