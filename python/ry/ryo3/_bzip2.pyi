"""ryo3-bzip2 types"""

from __future__ import annotations

# =============================================================================
# BZIP2
# =============================================================================
def bzip2_encode(input: bytes, quality: int = 9) -> bytes: ...
def bzip2_decode(input: bytes) -> bytes: ...
def bzip2(input: bytes, quality: int = 9) -> bytes:
    """Alias for bzip2_encode"""
