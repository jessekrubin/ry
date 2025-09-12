"""ry-types"""

from __future__ import annotations

import sys
from os import PathLike
from typing import TYPE_CHECKING, Literal, Protocol, TypeAlias, TypedDict, TypeVar

if TYPE_CHECKING:
    import datetime as pydt

if sys.version_info >= (3, 11):
    from typing import Never, Self
else:
    from typing_extensions import Never, Self

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
    "Never",
    "OffsetRoundTypedDict",
    "Self",
    "SignedDurationRoundTypedDict",
    "TimeDifferenceTypedDict",
    "TimeRoundTypedDict",
    "TimeSpanTypedDict",
    "TimeTypedDict",
    "TimestampDifferenceTypedDict",
    "TimestampRoundTypedDict",
    "TimestampTypedDict",
    "ToPy",
    "ToPyDate",
    "ToPyDateTime",
    "ToPyTime",
    "ToPyTimeDelta",
    "ToPyTzInfo",
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


class ToPy(Protocol[T_co]):
    """Objects that can be converted to a python stdlib type (`T_co`) via `obj.to_py()`."""

    def to_py(self) -> T_co: ...


class ToPyDate(Protocol):
    """Objects that can be converted to a Python `datetime.date`."""

    def to_pydate(self) -> pydt.date: ...


class ToPyTime(Protocol):
    """Objects that can be converted to a Python `datetime.time`."""

    def to_pytime(self) -> pydt.time: ...


class ToPyDateTime(Protocol):
    def to_pydatetime(self) -> pydt.datetime: ...


class ToPyTimeDelta(Protocol):
    def to_pytimedelta(self) -> pydt.timedelta: ...


class ToPyTzInfo(Protocol):
    def to_pytzinfo(self) -> pydt.tzinfo: ...


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


class DateTimeTypedDict(DateTypedDict, TimeTypedDict): ...


class ZonedDateTimeTypedDict(DateTimeTypedDict):
    tz: str


class TimestampTypedDict(DateTimeTypedDict):
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
class _RoundTypedDict(TypedDict):
    mode: JiffRoundMode
    increment: int


class DateTimeRoundTypedDict(_RoundTypedDict):
    smallest: Literal[
        "day",
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]


class SignedDurationRoundTypedDict(_RoundTypedDict):
    smallest: Literal[
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]


class TimeRoundTypedDict(TypedDict):
    smallest: Literal[
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]


class TimestampRoundTypedDict(_RoundTypedDict):
    smallest: Literal[
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]


class ZonedDateTimeRoundTypedDict(_RoundTypedDict):
    smallest: Literal[
        "day",
        "hour",
        "minute",
        "second",
        "millisecond",
        "microsecond",
        "nanosecond",
    ]


class OffsetRoundTypedDict(_RoundTypedDict):
    smallest: Literal[
        "second",
        "minute",
        "hour",
    ]


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
