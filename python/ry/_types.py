"""ry-types"""

from __future__ import annotations

import sys
from os import PathLike
from typing import TYPE_CHECKING, Protocol, TypedDict, TypeVar

if TYPE_CHECKING:
    import datetime as pydt

if sys.version_info >= (3, 11):
    from typing import Self
else:
    from typing_extensions import Self
if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

__all__ = (
    "Buffer",
    "DateTimeTypedDict",
    "DateTypedDict",
    "FromStr",
    "FsPathLike",
    "Self",
    "TimeSpanTypedDict",
    "TimeTypedDict",
    "ToPy",
    "ToPyDate",
    "ToPyDateTime",
    "ToPyTime",
    "ToPyTimeDelta",
    "ToPyTzInfo",
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


# protocol for function defining __json__() -> bytes / buffer:
class Stringify(Protocol):
    def __json__(self) -> Buffer | bytes | str: ...


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


class DateTimeTypedDict(DateTypedDict, TimeTypedDict): ...


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
