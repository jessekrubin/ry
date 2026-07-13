"""ry.ryo3.dev"""

import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

_Lz4BlockSize: t.TypeAlias = t.Literal[
    "auto", "max-64kb", "max-256kb", "max-1mb", "max-4mb", 0, 4, 5, 6, 7
]
_Lz4BlockMode: t.TypeAlias = t.Literal["independent", "linked"]

class Lz4FrameInfo(t.TypedDict, total=False):
    block_size: _Lz4BlockSize
    """block size for frame compression (default: `"auto"`/`0`)"""
    block_mode: _Lz4BlockMode
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
) -> Bytes:
    """Compress data into a complete lz4 frame.

    Parameters
    ----------
    data : Buffer
        Data to compress.
    dictionary : Buffer or None, default None
        Optional compression dictionary (e.g. from `lz4_train_dict`).
    dict_id : int or None, default None
        Dictionary id recorded in the frame header (default 0).
    frame_info : Lz4FrameInfo or None, default None
        Frame options (block size/mode, checksums, content size).

    Returns
    -------
    Bytes
        A complete lz4 frame.
    """

def lz4_compress_block(
    data: Buffer,
    *,
    size: bool = True,
    dictionary: Buffer | None = None,
) -> Bytes:
    """Compress a raw lz4 block.

    Parameters
    ----------
    data : Buffer
        Data to compress.
    size : bool, default True
        If True (python-lz4 compatible), prepend the uncompressed size to
        the block as a u32-le integer.
    dictionary : Buffer or None, default None
        Optional compression dictionary (e.g. from `lz4_train_dict`).

    Returns
    -------
    Bytes
        The compressed block.
    """

def lz4_decompress(
    data: Buffer,
    *,
    dictionary: Buffer | None = None,
    dict_id: int | None = None,
) -> Bytes:
    """Decompress a complete lz4 frame.

    Parameters
    ----------
    data : Buffer
        Complete lz4 frame data.
    dictionary : Buffer or None, default None
        Optional dictionary; must match the one used for compression.
    dict_id : int or None, default None
        Expected dictionary id (default 0).

    Returns
    -------
    Bytes
        The decompressed data.
    """

def lz4_decompress_block(
    data: Buffer,
    size: int | None = None,
    *,
    dictionary: Buffer | None = None,
) -> Bytes:
    """Decompress a raw lz4 block.

    Parameters
    ----------
    data : Buffer
        Compressed lz4 block data.
    size : int or None, default None
        If None (python-lz4 compatible), the block is expected to start
        with a u32-le uncompressed-size prefix (as written by
        `lz4_compress_block(..., size=True)`). Pass an explicit size
        (>= the actual uncompressed size) for prefix-less raw blocks.
    dictionary : Buffer or None, default None
        Optional dictionary; must match the one used for compression.

    Returns
    -------
    Bytes
        The decompressed data.
    """

def lz4_train_dict(samples: t.Iterable[Buffer], dict_size: int = 65535) -> Bytes:
    """Train an lz4 dictionary from sample messages (COVER algorithm).

    Signature mirrors `compression.zstd.train_dict` but returns raw
    dictionary bytes usable as the `dictionary` argument of the lz4
    (de)compression functions/classes.

    Parameters
    ----------
    samples : Iterable[Buffer]
        Sample messages to train on. Samples shorter than 4 bytes or
        longer than `dict_size` are silently skipped.
    dict_size : int, default 65535
        Maximum size of the trained dictionary in bytes; capped at 65535
        (the lz4 max match distance).

    Returns
    -------
    Bytes
        The trained dictionary (at most `dict_size` bytes).

    Raises
    ------
    ValueError
        If `dict_size` is not positive, or if training yields an empty
        dictionary (fewer than 2 usable samples).
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

    Parameters
    ----------
    dictionary : Buffer or None, default None
        Optional compression dictionary (e.g. from `lz4_train_dict`).
    dict_id : int or None, default None
        Dictionary id recorded in the frame header (default 0).
    frame_info : Lz4FrameInfo or None, default None
        Frame options (block size/mode, checksums, content size).
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
