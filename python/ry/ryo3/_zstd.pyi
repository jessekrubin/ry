"""ryo3-zstd types"""

from __future__ import annotations

# =============================================================================
# ZSTD
# =============================================================================

def zstd_decode(input: bytes) -> bytes: ...
def zstd_encode(input: bytes, level: int = 3) -> bytes: ...
def zstd(input: bytes, level: int = 3) -> bytes:
    """Alias for zstd_encode"""
