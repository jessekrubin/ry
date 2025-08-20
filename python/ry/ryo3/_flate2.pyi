"""ryo3-flate2 types"""

from ry import Bytes
from ry._types import Buffer

# =============================================================================
# GZIP
# =============================================================================
def gzip_encode(data: Buffer, quality: int = 9) -> Bytes: ...
def gzip_decode(data: Buffer) -> Bytes: ...
def gzip(data: Buffer, quality: int = 9) -> Bytes:
    """Alias for gzip_encode"""

def gunzip(data: Buffer) -> Bytes:
    """Alias for gzip_decode"""

def is_gzipped(data: Buffer) -> bool: ...
