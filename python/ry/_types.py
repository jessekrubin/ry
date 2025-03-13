"""ry-types"""

from __future__ import annotations

import sys
from os import PathLike
from typing import Protocol, TypedDict, TypeVar

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

__all__ = (
    "Buffer",
    "DateTimeTypedDict",
    "DateTypedDict",
    "FsPathLike",
    "TimeSpanTypedDict",
    "TimeTypedDict",
)

FsPathLike = str | PathLike[str]

T_co = TypeVar("T_co", covariant=True)


class ToPy(Protocol[T_co]):
    def to_py(self) -> T_co: ...


# =============================================================================
# JIFF
# =============================================================================
class DateTypedDict(TypedDict):
    year: int
    month: int
    day: int


class TimeTypedDict(TypedDict):
    hour: int
    minute: int
    second: int
    millisecond: int
    microsecond: int
    nanosecond: int
    subsec_nanosecond: int


class DateTimeTypedDict(TypedDict):
    year: int
    month: int
    day: int
    hour: int
    minute: int
    second: int
    millisecond: int
    microsecond: int
    nanosecond: int
    subsec_nanosecond: int


class TimeSpanTypedDict(TypedDict):
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
