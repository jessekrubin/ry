import typing as t

FormatSizeBase: t.TypeAlias = t.Literal[2, 10]  # default=2
FormatSizeStyle: t.TypeAlias = t.Literal[  # default="default"
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

    def __init__(
        self,
        base: FormatSizeBase = 2,
        style: FormatSizeStyle = "default",
    ) -> None:
        """Initialize human-readable bytes-size formatter."""

    def format(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __call__(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

@t.final
class Size:
    """Bytes-size object."""

    def __init__(self, size: int) -> None: ...
    @property
    def bytes(self) -> int: ...
    def format(
        self,
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
    def parse(cls, size: str) -> Size: ...
    @classmethod
    def from_str(cls, size: str) -> Size: ...

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
