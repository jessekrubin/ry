"""ry.ryo3.dev"""

import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

def devfn() -> t.Literal["_ryo3-dev"]: ...

# =============================================================================
# LZ4RIP
# =============================================================================

def lz4_compress(data: Buffer, block: bool = False) -> bytes: ...
@t.overload
def lz4_decompress(
    data: Buffer,
    uncompressed_size: int | None = None,
    *,
    block: t.Literal[True] = ...,
    dictionary: Buffer | None = None,
) -> Bytes: ...
@t.overload
def lz4_decompress(
    data: Buffer,
    uncompressed_size: int,
    *,
    block: t.Literal[False] = False,
    dictionary: Buffer | None = None,
    dict_id: int = 0,
) -> Bytes: ...
