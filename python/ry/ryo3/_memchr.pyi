"""ryo3-memchr types"""

from ry._types import Buffer

def memchr(needle: Buffer | int, haystack: Buffer) -> int | None: ...
