from __future__ import annotations

from typing import Literal, TypedDict

__all__ = (
    "JIFF_ROUND_MODE_STRING",
    "JIFF_UNIT_STRING",
    "DateTimeTypedDict",
    "DateTypedDict",
    "TimeSpanTypedDict",
    "TimeTypedDict",
)

JIFF_UNIT_STRING = Literal[
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
]

JIFF_ROUND_MODE_STRING = Literal[
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half_ceil",
    "half_floor",
    "half_expand",
    "half_trunc",
    "half_even",
]


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
