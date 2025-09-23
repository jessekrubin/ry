"""ry-types"""

from __future__ import annotations

import sys
from os import PathLike
from typing import TYPE_CHECKING, Literal, Protocol, Self, TypeAlias, TypedDict, TypeVar

if TYPE_CHECKING:
    import datetime as pydt


if sys.version_info >= (3, 12):
    from collections.abc import Buffer
    from typing import Unpack
else:
    from typing_extensions import Buffer, Unpack

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

__all__ = (
    "Buffer",
    "DateDifferenceTypedDict",
    "DateTimeDifferenceTypedDict",
    "DateTimeRoundTypedDict",
    "DateTimeTypedDict",
    "DateTimeTypedDict",
    "DateTypedDict",
    "DateTypedDict",
    "FileTypeDict",
    "FromStr",
    "FsPathLike",
    "ISOWeekDateTypedDict",
    "JiffRoundMode",
    "JiffUnit",
    "MetadataDict",
    "OffsetRoundTypedDict",
    "SignedDurationRoundTypedDict",
    "TimeDifferenceTypedDict",
    "TimeRoundTypedDict",
    "TimeSpanTypedDict",
    "TimeTypedDict",
    "TimestampDifferenceTypedDict",
    "TimestampRoundTypedDict",
    "TimestampTypedDict",
    "Unpack",
    "ZonedDateTimeDifferenceTypedDict",
    "ZonedDateTimeRoundTypedDict",
    "deprecated",
)

FsPathLike = str | PathLike[str]

T_co = TypeVar("T_co", covariant=True)


class FromStr(Protocol):
    @classmethod
    def from_str(cls, s: str) -> Self: ...


# =============================================================================
# STD
# =============================================================================
class FileTypeDict(TypedDict):
    is_dir: bool
    is_file: bool
    is_symlink: bool


class MetadataDict(TypedDict):
    is_dir: bool
    is_file: bool
    is_symlink: bool
    len: int
    readonly: bool
    file_type: FileTypeDict | None
    accessed: pydt.datetime
    created: pydt.datetime
    modified: pydt.datetime


# =============================================================================
# JIFF
# =============================================================================
JiffUnit: TypeAlias = Literal[
    "year",  # 9
    "month",  # 8
    "day",  # 6
    "hour",  # 5
    "minute",  # 4
    "second",  # 3
    "millisecond",  # 2
    "microsecond",  # 1
    "nanosecond",  # 0
]
JiffRoundMode: TypeAlias = Literal[
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half-ceil",
    "half-floor",
    "half-expand",
    "half-trunc",
    "half-even",
]


class DateTypedDict(TypedDict):
    year: int
    month: int
    day: int


class TimeTypedDict(TypedDict):
    hour: int
    minute: int
    second: int
    nanosecond: int


class DateTimeTypedDict(TypedDict):
    year: int
    month: int
    day: int
    hour: int
    minute: int
    second: int
    nanosecond: int


class ZonedDateTimeTypedDict(TypedDict):
    year: int
    month: int
    day: int
    hour: int
    minute: int
    second: int
    nanosecond: int
    tz: str


class TimestampTypedDict:
    second: int
    nanosecond: int


class SignedDurationTypedDict(TypedDict):
    secs: int
    nanos: int


class TimeSpanTypedDict(TypedDict):
    """TimeSpan TypedDict

    Examples:
        >>> import ry
        >>> ts = ry.timespan(years=1, months=2, weeks=3)
        >>> ts.to_dict()
        {'years': 1, 'months': 2, 'weeks': 3, 'days': 0, 'hours': 0, 'minutes': 0, 'seconds': 0, 'milliseconds': 0, 'microseconds': 0, 'nanoseconds': 0}

    """

    years: int
    months: int
    weeks: int
    days: int
    hours: int
    minutes: int
    seconds: int
    milliseconds: int
    microseconds: int
    nanoseconds: int


class OffsetTypedDict(TypedDict):
    seconds: int
    fmt: str


class ISOWeekDateTypedDict(TypedDict):
    year: int
    week: int
    weekday: int


# -----------------------------------------------------------------------------
# JIFF ROUND
# -----------------------------------------------------------------------------
class DateTimeRoundTypedDict(TypedDict):
    smallest: Literal[
        "day",
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]
    mode: JiffRoundMode
    increment: int


class SignedDurationRoundTypedDict(TypedDict):
    smallest: Literal[
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]
    mode: JiffRoundMode
    increment: int


class TimeRoundTypedDict(TypedDict):
    smallest: Literal[
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]
    mode: JiffRoundMode
    increment: int


class TimestampRoundTypedDict(TypedDict):
    smallest: Literal[
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]
    mode: JiffRoundMode
    increment: int


class ZonedDateTimeRoundTypedDict(TypedDict):
    smallest: Literal[
        "day",
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]
    mode: JiffRoundMode
    increment: int


class OffsetRoundTypedDict(TypedDict):
    smallest: Literal[
        "second",
        "minute",
        "hour",
    ]
    mode: JiffRoundMode
    increment: int


# -----------------------------------------------------------------------------
# JIFF DIFFERENCE
# -----------------------------------------------------------------------------
class _DifferenceTypedDict(TypedDict):
    mode: JiffRoundMode
    increment: int


DateDifferenceUnit: TypeAlias = Literal["month", "year", "day"]


class DateDifferenceTypedDict(_DifferenceTypedDict):
    smallest: DateDifferenceUnit
    largest: DateDifferenceUnit | None


class DateTimeDifferenceTypedDict(_DifferenceTypedDict):
    smallest: JiffUnit
    largest: JiffUnit | None


TimeDifferenceUnit: TypeAlias = Literal[
    "hour", "minute", "second", "millisecond", "microsecond", "nanosecond"
]


class TimeDifferenceTypedDict(_DifferenceTypedDict):
    smallest: TimeDifferenceUnit
    largest: TimeDifferenceUnit | None


class ZonedDateTimeDifferenceTypedDict(_DifferenceTypedDict):
    smallest: JiffUnit
    largest: JiffUnit | None


TimeStampDifferenceUnit: TypeAlias = Literal[
    "hour", "minute", "second", "millisecond", "microsecond", "nanosecond"
]


class TimestampDifferenceTypedDict(_DifferenceTypedDict):
    smallest: TimeStampDifferenceUnit
    largest: TimeStampDifferenceUnit | None


# =============================================================================
# OPEN MODES (CANONICAL)
# =============================================================================
# ry accepts the non-cannonical modes, but they are mapped to the canonical ones]

OpenTextModeUpdating: TypeAlias = Literal[
    "a+", "at+", "r+", "rt+", "w+", "wt+", "x+", "xt+"
]
OpenTextModeWriting: TypeAlias = Literal["a", "at", "w", "wt", "x", "xt"]
OpenTextModeReading: TypeAlias = Literal["r", "rt"]
OpenTextMode: TypeAlias = Literal[
    "a",
    "a+",
    "at",
    "at+",
    "r",
    "r+",
    "rt",
    "rt+",
    "w",
    "w+",
    "wt",
    "wt+",
    "x",
    "x+",
    "xt",
    "xt+",
]
OpenBinaryModeUpdating: TypeAlias = Literal["ab+", "rb+", "wb+", "xb+"]
OpenBinaryModeWriting: TypeAlias = Literal["ab", "wb", "xb"]
OpenBinaryModeReading: TypeAlias = Literal["rb"]
OpenBinaryMode: TypeAlias = Literal["ab", "ab+", "rb", "rb+", "wb", "wb+", "xb", "xb+"]
OpenMode: TypeAlias = Literal[
    "a",
    "a+",
    "ab",
    "ab+",
    "at",
    "at+",
    "r",
    "r+",
    "rb",
    "rb+",
    "rt",
    "rt+",
    "w",
    "w+",
    "wb",
    "wb+",
    "wt",
    "wt+",
    "x",
    "x+",
    "xb",
    "xb+",
    "xt",
    "xt+",
]
