"""ryo3-flate2 types"""

from ry import Bytes
from ry._types import Buffer

# =============================================================================
# GZIP
# =============================================================================
def gzip_encode(input: Buffer, quality: int = 9) -> Bytes: ...
def gzip_decode(input: Buffer) -> Bytes: ...
def gzip(input: Buffer, quality: int = 9) -> Bytes:
    """Alias for gzip_encode"""

def gunzip(input: Buffer) -> Bytes:
    """Alias for gzip_decode"""

def is_gzipped(input: Buffer) -> bool: ...
