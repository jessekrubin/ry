from __future__ import annotations

from typing import Literal

FORMAT_SIZE_BASE = Literal[2, 10]  # default=2
FORMAT_SIZE_STYLE = Literal[  # default="default"
    "default",
    "abbreviated",
    "abbreviated_lowercase",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
    "full_lowercase",
]

def fmt_size(
    n: int,
    *,
    base: FORMAT_SIZE_BASE | None = 2,
    style: FORMAT_SIZE_STYLE | None = "default",
) -> str:
    """Return human-readable string representation of bytes-size."""

def parse_size(s: str) -> int:
    """Return integer representation of human-readable bytes-size string.

    Raises:
        ValueError: If string is not a valid human-readable bytes-size string.
    """

class SizeFormatter:
    """Human-readable bytes-size formatter."""

    def __init__(
        self,
        base: FORMAT_SIZE_BASE | None = 2,
        style: FORMAT_SIZE_STYLE | None = "default",
    ):
        self.base = base
        self.style = style

    def format(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __call__(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __repr__(self) -> str: ...

class Size:
    """Bytes-size object."""

    def __init__(self, size: int) -> None: ...
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __hash__(self) -> int: ...
    def __abs__(self) -> Size: ...
    def __neg__(self) -> Size: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Size | int | float) -> bool: ...
    def __le__(self, other: Size | int | float) -> bool: ...
    def __gt__(self, other: Size | int | float) -> bool: ...
    def __ge__(self, other: Size | int | float) -> bool: ...
    def __bool__(self) -> bool: ...
    def __pos__(self) -> Size: ...
    def __invert__(self) -> Size: ...
    def __add__(self, other: Size | int | float) -> Size: ...
    def __sub__(self, other: Size | int | float) -> Size: ...
    def __mul__(self, other: Size | int | float) -> Size: ...
    @property
    def bytes(self) -> int: ...
    def format(
        self,
        base: FORMAT_SIZE_BASE | None = 2,
        style: FORMAT_SIZE_STYLE | None = "default",
    ) -> str: ...

    # =========================================================================
    # CLASS-METHODS
    # =========================================================================

    # -------------------------------------------------------------------------
    # PARSING
    # -------------------------------------------------------------------------
    @classmethod
    def parse(cls: type[Size], size: str) -> Size: ...
    @classmethod
    def from_str(cls: type[Size], size: str) -> Size: ...

    # -------------------------------------------------------------------------
    # BYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_bytes(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # KILOBYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_kb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_kib(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_kibibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_kilobytes(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # MEGABYTES
    # -------------------------------------------------------------------------

    @classmethod
    def from_mb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_mebibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_megabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_mib(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # GIGABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_gb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_gib(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_gibibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_gigabytes(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # TERABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_tb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_tebibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_terabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_tib(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # PETABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_pb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_pebibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_petabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_pib(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # EXABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_eb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_eib(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_exabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_exbibytes(cls: type[Size], size: int | float) -> Size: ...
