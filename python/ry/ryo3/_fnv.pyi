"""ryo3-fnv types"""

import typing as t

from ry._types import Buffer

@t.final
class fnv1a:  # noqa: N801
    name: t.Literal["fnv1a"]
    digest_size: t.Literal[8]
    block_size: t.Literal[1]
    default_seed: t.Literal[0xCBF29CE484222325]  # noqa: PYI054

    def __new__(
        cls,
        data: Buffer | None = None,
        *,
        seed: int | bytes = 0xCBF29CE484222325,  # noqa: PYI054
    ) -> t.Self: ...
    def update(self, data: Buffer) -> None: ...
    def digest(self) -> bytes: ...
    def intdigest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> t.Self: ...
    @staticmethod
    def oneshot(data: Buffer, *, seed: int | bytes = 0xCBF29CE484222325) -> bytes: ...  # noqa: PYI054
    @staticmethod
    def oneshot_int(data: Buffer, *, seed: int | bytes = 0xCBF29CE484222325) -> int: ...  # noqa: PYI054
    @staticmethod
    def oneshot_hex(data: Buffer, *, seed: int | bytes = 0xCBF29CE484222325) -> str: ...  # noqa: PYI054
