import builtins
import typing as t

from ry.protocols import FromStr, _Parse

FormatSizeBase: t.TypeAlias = t.Literal[2, 10]  # default=2
FormatSizeStyle: t.TypeAlias = t.Literal[  # default="default"
    "default",
    "abbreviated",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
]

def fmt_size(
    n: int,
    *,
    base: FormatSizeBase = 2,
    style: FormatSizeStyle = "default",
) -> str:
    """Return human-readable string representation of bytes-size."""

def parse_size(s: str) -> int:
    """Return integer representation of human-readable bytes-size string.

    Raises:
        ValueError: If string is not a valid human-readable bytes-size string.
    """

@t.final
class SizeFormatter:
    """Human-readable bytes-size formatter."""

    def __new__(
        cls,
        base: FormatSizeBase = 2,
        style: FormatSizeStyle = "default",
    ) -> t.Self:
        """Initialize human-readable bytes-size formatter."""

    def format(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __call__(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    @property
    def base(self) -> FormatSizeBase:
        """Return base used by formatter."""
    @property
    def style(self) -> FormatSizeStyle:
        """Return style used by formatter."""

    def with_base(self, base: FormatSizeBase) -> SizeFormatter:
        """Return new `SizeFormatter` with specified base."""

    def with_style(self, style: FormatSizeStyle) -> SizeFormatter:
        """Return new `SizeFormatter` with specified style."""

@t.final
class Size(FromStr, _Parse):
    """Bytes-size object."""

    def __new__(cls, size: int) -> t.Self: ...
    @property
    def bytes(self) -> int: ...
    def format(
        self,
        *,
        base: FormatSizeBase = 2,
        style: FormatSizeStyle = "default",
    ) -> str: ...

    # =========================================================================
    # CLASS-METHODS
    # =========================================================================

    # -------------------------------------------------------------------------
    # PARSING
    # -------------------------------------------------------------------------
    @classmethod
    def parse(cls, value: str | builtins.bytes, /) -> Size: ...
    @classmethod
    def from_str(cls, s: str, /) -> Size: ...

    # -------------------------------------------------------------------------
    # BYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_bytes(cls, size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # KILOBYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_kb(cls, size: float) -> Size: ...
    @classmethod
    def from_kib(cls, size: float) -> Size: ...
    @classmethod
    def from_kibibytes(cls, size: float) -> Size: ...
    @classmethod
    def from_kilobytes(cls, size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # MEGABYTES
    # -------------------------------------------------------------------------

    @classmethod
    def from_mb(cls, size: float) -> Size: ...
    @classmethod
    def from_mebibytes(cls, size: float) -> Size: ...
    @classmethod
    def from_megabytes(cls, size: float) -> Size: ...
    @classmethod
    def from_mib(cls, size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # GIGABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_gb(cls, size: float) -> Size: ...
    @classmethod
    def from_gib(cls, size: float) -> Size: ...
    @classmethod
    def from_gibibytes(cls, size: float) -> Size: ...
    @classmethod
    def from_gigabytes(cls, size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # TERABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_tb(cls, size: float) -> Size: ...
    @classmethod
    def from_tebibytes(cls, size: float) -> Size: ...
    @classmethod
    def from_terabytes(cls, size: float) -> Size: ...
    @classmethod
    def from_tib(cls, size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # PETABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_pb(cls, size: float) -> Size: ...
    @classmethod
    def from_pebibytes(cls, size: float) -> Size: ...
    @classmethod
    def from_petabytes(cls, size: float) -> Size: ...
    @classmethod
    def from_pib(cls, size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # EXABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_eb(cls, size: float) -> Size: ...
    @classmethod
    def from_eib(cls, size: float) -> Size: ...
    @classmethod
    def from_exabytes(cls, size: float) -> Size: ...
    @classmethod
    def from_exbibytes(cls, size: float) -> Size: ...

    # =========================================================================
    # DUNDERS
    # =========================================================================
    def __add__(self, other: Size | float) -> Size: ...
    def __sub__(self, other: Size | float) -> Size: ...
    def __mul__(self, other: Size | float) -> Size: ...
    def __rmul__(self, other: Size | float) -> Size: ...
    def __neg__(self) -> Size: ...
    def __pos__(self) -> Size: ...
    def __abs__(self) -> Size: ...
    def __invert__(self) -> Size: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Size | float) -> bool: ...
    def __le__(self, other: Size | float) -> bool: ...
    def __gt__(self, other: Size | float) -> bool: ...
    def __ge__(self, other: Size | float) -> bool: ...
    def __hash__(self) -> int: ...
    def __bool__(self) -> bool: ...
    def __int__(self) -> int: ...

    # -------------------------------------------------------------------------
    # CONSTANTS
    # -------------------------------------------------------------------------
    ZERO: t.Final[Size]  # Size(0)
    MAX: t.Final[Size]  # Size(9_223_372_036_854_775_807)
    MIN: t.Final[Size]  # Size(-9_223_372_036_854_775_808)
    # byte
    B: t.Final[Size]  # Size(1)
    BYTE: t.Final[Size]  # Size(1)
    # kilobyte
    KB: t.Final[Size]  # Size(1_000)
    KIB: t.Final[Size]  # Size(1_024)
    KIBIBYTE: t.Final[Size]  # Size(1_024)
    KILOBYTE: t.Final[Size]  # Size(1_000)
    # megabyte
    MB: t.Final[Size]  # Size(1_000_000)
    MEBIBYTE: t.Final[Size]  # Size(1_048_576)
    MEGABYTE: t.Final[Size]  # Size(1_000_000)
    MIB: t.Final[Size]  # Size(1_048_576)
    # gigabyte
    GB: t.Final[Size]  # Size(1_000_000_000)
    GIB: t.Final[Size]  # Size(1_073_741_824)
    GIBIBYTE: t.Final[Size]  # Size(1_073_741_824)
    GIGABYTE: t.Final[Size]  # Size(1_000_000_000)
    # terabyte
    TB: t.Final[Size]  # Size(1_000_000_000_000)
    TEBIBYTE: t.Final[Size]  # Size(1_099_511_627_776)
    TERABYTE: t.Final[Size]  # Size(1_000_000_000_000)
    TIB: t.Final[Size]  # Size(1_099_511_627_776)
    # petabyte
    PB: t.Final[Size]  # Size(1_000_000_000_000_000)
    PEBIBYTE: t.Final[Size]  # Size(1_125_899_906_842_624)
    PETABYTE: t.Final[Size]  # Size(1_000_000_000_000_000)
    PIB: t.Final[Size]  # Size(1_125_899_906_842_624)
    # exabyte
    EB: t.Final[Size]  # Size(1_000_000_000_000_000_000)
    EIB: t.Final[Size]  # Size(1_152_921_504_606_846_976)
    EXABYTE: t.Final[Size]  # Size(1_000_000_000_000_000_000)
    EXBIBYTE: t.Final[Size]  # Size(1_152_921_504_606_846_976)
