from ry import Bytes
from ry._types import Buffer

__zstd_version__: str  # zstd version string ("1.5.7" as of 2025-03-14)
BLOCKSIZELOG_MAX: int
BLOCKSIZE_MAX: int
CLEVEL_DEFAULT: int  # default=3 (as of 2025-03-14)
CONTENTSIZE_ERROR: int
CONTENTSIZE_UNKNOWN: int
MAGICNUMBER: int
MAGIC_DICTIONARY: int
MAGIC_SKIPPABLE_MASK: int
MAGIC_SKIPPABLE_START: int
VERSION_MAJOR: int
VERSION_MINOR: int
VERSION_NUMBER: int
VERSION_RELEASE: int

# =============================================================================
# PYFUNCTIONS
# =============================================================================
# __COMPRESSION__
def compress(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...
def encode(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...
def zstd(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...
def zstd_compress(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...
def zstd_encode(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...

# __DECOMPRESSION__
def decode(data: Buffer) -> Bytes: ...
def decompress(data: Buffer) -> Bytes: ...
def unzstd(data: Buffer) -> Bytes: ...
def zstd_decode(data: Buffer) -> Bytes: ...
def zstd_decompress(data: Buffer) -> Bytes: ...

# __MAGIC__
def is_zstd(data: Buffer) -> bool: ...
