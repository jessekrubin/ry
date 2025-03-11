"""ryo3-fnv types"""

import typing as t

from ry._types import Buffer

# =============================================================================
# FNV
# =============================================================================
class FnvHasher:
    name: t.Literal["fnv1a"]

    def __init__(self, input: Buffer | None = None) -> None: ...
    def update(self, input: Buffer) -> None: ...
    def digest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> FnvHasher: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

def fnv1a(input: Buffer) -> FnvHasher: ...
