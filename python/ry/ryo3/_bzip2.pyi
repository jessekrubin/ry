"""ryo3-bzip2 types"""

from ry._types import Buffer

# =============================================================================
# BZIP2
# =============================================================================
def bzip2_encode(input: Buffer, quality: int = 9) -> bytes: ...
def bzip2_decode(input: Buffer) -> bytes: ...
def bzip2(input: Buffer, quality: int = 9) -> bytes:
    """Alias for bzip2_encode"""
