"""ry-types"""

from __future__ import annotations

import sys
from typing import TypedDict

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer
__all__ = (
    "Buffer",
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
