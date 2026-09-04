"""ryo3-fnv ~ types"""

import typing as t

from ry._types import Buffer

@t.final
class crc32:  # noqa: N801
    name: t.Literal["crc32"]
    digest_size: t.Literal[4]
    block_size: t.Literal[1]
    default_seed: t.Literal[0]

    def __new__(
        cls, data: Buffer | None = None, *, seed: int | bytes = 0
    ) -> t.Self: ...
    def update(self, data: Buffer) -> None: ...
    def digest(self) -> bytes: ...
    def intdigest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> t.Self: ...
    @staticmethod
    def oneshot(data: Buffer, *, seed: int | bytes = 0) -> bytes: ...
    @staticmethod
    def oneshot_int(data: Buffer, *, seed: int | bytes = 0) -> int: ...
    @staticmethod
    def oneshot_hex(data: Buffer, *, seed: int | bytes = 0) -> str: ...

    # FUTURE/DEV/TBD
    def _combine(self, other: t.Self) -> t.Self:
        """Return new crc32 hash state after combining with other

        Used for combining the current hash state with the hash state for the subsequent block of bytes.

        Parameters
        ----------
        other : t.Self
            other crc32 hash state

        Returns
        -------
        t.Self

        """
