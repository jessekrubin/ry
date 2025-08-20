"""ryo3-bzip2 types"""

from ry._types import Buffer

# =============================================================================
# BZIP2
# =============================================================================
def bzip2_encode(data: Buffer, quality: int = 9) -> bytes: ...
def bzip2_decode(data: Buffer) -> bytes: ...
def bzip2(data: Buffer, quality: int = 9) -> bytes:
    """Alias for bzip2_encode"""
