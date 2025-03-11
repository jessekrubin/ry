"""ryo3-zstd types"""

from __future__ import annotations

from ry import Bytes
from ry._types import Buffer

# =============================================================================
# ZSTD
# =============================================================================

def zstd_decode(input: Buffer) -> Bytes: ...
def zstd_encode(input: Buffer, level: int = 3) -> Bytes: ...
def zstd(input: Buffer, level: int = 3) -> Bytes:
    """Alias for zstd_encode"""
