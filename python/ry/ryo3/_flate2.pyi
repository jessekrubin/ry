"""ryo3-flate2 types"""

from __future__ import annotations

# =============================================================================
# GZIP
# =============================================================================
def gzip_encode(input: bytes, quality: int = 9) -> bytes: ...
def gzip_decode(input: bytes) -> bytes: ...
def gzip(input: bytes, quality: int = 9) -> bytes:
    """Alias for gzip_encode"""

def gunzip(input: bytes) -> bytes:
    """Alias for gzip_decode"""
