"""ryo3-fnv types"""

import typing as t

from ry._types import Buffer

@t.final
class FnvHasher:
    name: t.Literal["fnv1a"]
    digest_size: t.Literal[8]
    block_size: t.Literal[1]

    def __init__(
        self,
        data: Buffer | None = None,
        *,
        key: int | bytes = 0xCBF29CE484222325,  # noqa: PYI054
    ) -> None: ...
    def update(self, data: Buffer) -> None: ...
    def digest(self) -> bytes: ...
    def intdigest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> FnvHasher: ...

def fnv1a(
    data: Buffer | None = None,
    *,
    key: int | bytes = 0xCBF29CE484222325,  # noqa: PYI054
) -> FnvHasher: ...
