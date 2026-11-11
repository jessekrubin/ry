"""ryo3-brotli ~ types"""

from ry._types import Buffer
from ry.ryo3._bytes import Bytes

def brotli_decompress(data: Buffer, /) -> Bytes: ...
