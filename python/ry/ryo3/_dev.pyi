"""ry.ryo3.dev"""

import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

def devfn() -> t.Literal["_ryo3-dev"]: ...

# =============================================================================
# LZ4RIP
# =============================================================================

Lz4BlockSize: t.TypeAlias = t.Literal[
    "auto", "max-64kb", "max-256kb", "max-1mb", "max-4mb", 0, 4, 5, 6, 7
]
Lz4BlockMode: t.TypeAlias = t.Literal["independent", "linked"]

class Lz4FrameInfo(t.TypedDict, total=False):
    block_size: Lz4BlockSize
    """block size for frame compression (default: `"auto"`/`0`)"""
    block_mode: Lz4BlockMode
    """block dependency mode for frame compression (default: `"independent"`)"""
    block_checksums: bool
    """include a checksum (xxh32<seed=0>) for each frame block (default: `False`)"""
    content_checksum: bool
    """include a checksum (xxh32<seed=0>) of the uncompressed data (default: `False`)"""
    content_size: int | None
    """include the total uncompressed size of data in the frame (default: `None`)"""

def lz4_compress(
    data: Buffer,
    *,
    dictionary: Buffer | None = None,
    dict_id: int | None = None,
    frame_info: Lz4FrameInfo | None = None,
) -> Bytes: ...
def lz4_compress_block(
    data: Buffer,
    *,
    size: bool = True,
    dictionary: Buffer | None = None,
) -> Bytes:
    """Compress a raw lz4 block.

    If `size` is `True` (default; python-lz4 compatible) the uncompressed
    size is prepended to the block as a u32-le integer.
    """

def lz4_decompress(
    data: Buffer,
    *,
    dictionary: Buffer | None = None,
    dict_id: int | None = None,
) -> Bytes: ...
def lz4_decompress_block(
    data: Buffer,
    size: int | None = None,
    *,
    dictionary: Buffer | None = None,
) -> Bytes:
    """Decompress a raw lz4 block.

    If `size` is `None` (default; python-lz4 compatible) the block is
    expected to start with a u32-le uncompressed-size prefix (as written by
    `lz4_compress_block(..., size=True)`). Pass an explicit `size` (>= the
    actual uncompressed size) for prefix-less raw blocks.
    """

class Lz4BlockCompressor:
    def __init__(self, dictionary: Buffer | None = None) -> None: ...
    def compress(self, data: Buffer, *, size: bool = True) -> Bytes: ...

class Lz4BlockDecompressor:
    def __init__(self, dictionary: Buffer | None = None) -> None: ...
    def decompress(self, data: Buffer, size: int | None = None) -> Bytes: ...

class Lz4FrameCompressor:
    """Streaming lz4 frame compressor.

    `compress`/`flush` return the newly produced compressed bytes; `finish`
    ends the frame and returns the tail. The concatenation of all returned
    chunks is one complete lz4 frame.
    """

    def __init__(
        self,
        *,
        dictionary: Buffer | None = None,
        dict_id: int | None = None,
        frame_info: Lz4FrameInfo | None = None,
    ) -> None: ...
    def compress(self, data: Buffer) -> Bytes: ...
    def flush(self) -> Bytes: ...
    def finish(self) -> Bytes: ...
