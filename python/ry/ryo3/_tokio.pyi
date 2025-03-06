"""ryo3-tokio types"""

from __future__ import annotations

from typing import NoReturn

from ry import Bytes
from ry._types import Buffer, FsPathLike

# =============================================================================
# FS
# =============================================================================
async def copy_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def create_dir_async(path: FsPathLike) -> None: ...
async def metadata_async(path: FsPathLike) -> None: ...
async def read_async(path: FsPathLike) -> Bytes: ...
async def read_dir_async(path: FsPathLike) -> NoReturn: ...
async def remove_dir_async(path: FsPathLike) -> None: ...
async def remove_file_async(path: FsPathLike) -> None: ...
async def rename_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def write_async(path: FsPathLike, data: Buffer) -> None: ...

# =============================================================================
# SLEEP
# =============================================================================
async def sleep_async(seconds: float) -> float: ...
async def asleep(seconds: float) -> float:
    """Alias for sleep_async"""
    ...
