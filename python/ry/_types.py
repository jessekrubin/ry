"""ry-types"""

from __future__ import annotations

from typing import TypedDict

__all__ = (
    "DateTimeTypedDict",
    "DateTypedDict",
    "TimeSpanTypedDict",
    "TimeTypedDict",
)


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
