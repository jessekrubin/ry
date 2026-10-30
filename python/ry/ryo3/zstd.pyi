"""ry.zstd submodule ~ DEPRECATED, remove in v0.0.109

use the flat `ry.zstd_compress` / `ry.zstd_decompress` / `ry.is_zstd` /
`ry.ZSTD_VERSION` / `ry.ZSTD_VERSION_INFO` instead
"""

import sys

from ry import Bytes
from ry._types import Buffer
from ry.ryo3._zstd import _ZstdLevel

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

# deprecated constants ~ they warn at runtime (served by a module
# `__getattr__`), but variables can't be marked `@deprecated` in stubs
__zstd_version__: str  # -> `ry.ZSTD_VERSION`
BLOCKSIZELOG_MAX: int  # no replacement
BLOCKSIZE_MAX: int  # no replacement
CLEVEL_DEFAULT: int  # -> `ry.zstd_compress` default `level=3`
CONTENTSIZE_ERROR: int  # no replacement
CONTENTSIZE_UNKNOWN: int  # no replacement
MAGICNUMBER: int  # -> `ry.is_zstd`
MAGIC_DICTIONARY: int  # no replacement
MAGIC_SKIPPABLE_MASK: int  # no replacement
MAGIC_SKIPPABLE_START: int  # no replacement
VERSION_MAJOR: int  # -> `ry.ZSTD_VERSION_INFO`
VERSION_MINOR: int  # -> `ry.ZSTD_VERSION_INFO`
VERSION_NUMBER: int  # -> `ry.ZSTD_VERSION_INFO`
VERSION_RELEASE: int  # -> `ry.ZSTD_VERSION_INFO`

@deprecated(
    "`ry.zstd.compress` is deprecated; use `ry.zstd_compress` instead [removal: v0.0.109]"
)
def compress(data: Buffer, level: _ZstdLevel = 3) -> Bytes: ...
@deprecated(
    "`ry.zstd.decompress` is deprecated; use `ry.zstd_decompress` instead [removal: v0.0.109]"
)
def decompress(data: Buffer) -> Bytes: ...
@deprecated(
    "`ry.zstd.decode` is deprecated; use `ry.zstd_decompress` instead [removal: v0.0.109]"
)
def decode(data: Buffer) -> Bytes: ...
@deprecated(
    "`ry.zstd.unzstd` is deprecated; use `ry.zstd_decompress` instead [removal: v0.0.109]"
)
def unzstd(data: Buffer) -> Bytes: ...
@deprecated(
    "`ry.zstd.is_zstd` is deprecated; use `ry.is_zstd` instead [removal: v0.0.109]"
)
def is_zstd(data: Buffer) -> bool: ...
